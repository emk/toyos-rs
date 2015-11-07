//! A simple heap based on a buddy allocator.  For the theory of buddy
//! allocators, see https://en.wikipedia.org/wiki/Buddy_memory_allocation
//!
//! The basic idea is that our heap size is a power of two, and the heap
//! starts out as one giant free block.  When a memory allocation request
//! is received, we round the requested size up to a power of two, and find
//! the smallest available block we can use.  If the smallest free block is
//! too big (more than twice as big as the memory we want to allocate), we
//! split the smallest free block in half recursively until it's the right
//! size.  This simplifies a lot of bookkeeping, because all our block
//! sizes are a power of 2, which makes it easy to have one free list per
//! block size.

use std::cmp::max;
use std::mem::size_of;
use std::ptr;

use math::PowersOf2;

const MIN_HEAP_ALIGN: usize = 4096;

/// A free block in our heap.  This is actually a header that we store at
/// the start of the block.  We don't store any size information in the
/// header, because we a separate free block list for each block size.
pub struct FreeBlock {
    /// The next block in the free list, or NULL if this is the final
    /// block.
    next: *mut FreeBlock,
}

impl FreeBlock {
    /// Construct a `FreeBlock` header pointing at `next`.  This is sort of
    /// like `cons` in LISP, except that here, we're building a link list
    /// with no data slot.  The "data" is actually the address at which we
    /// store the `FreeBlock`!
    ///
    /// I use functional programming terminology (`head` and `tail`)
    /// because these sorts of data structures are much easier to reason
    /// about correctly in functional languages.
    fn head(next: *mut FreeBlock) -> FreeBlock {
        FreeBlock { next: next }
    }

    /// The final block in a `FreeBlock` list.
    fn tail() -> FreeBlock {
        FreeBlock { next: ptr::null_mut() }
    }
}

/// The interface to a heap.  This data structure is stored _outside_ the
/// heap somewhere, because every single byte of our heap is potentially
/// available for allocation.
pub struct Heap<'a> {
    /// The base address of our heap.  This must be aligned on a
    /// `MIN_HEAP_ALIGN` boundary.
    heap_base: *mut u8,

    /// The space available in our heap.  This must be a power of 2.
    heap_size: usize,

    /// The free lists for our heap.  The list at `free_lists[0]` contains
    /// the smallest block size we can allocate, and the list at the end
    /// can only contain a single free block the size of our entire heap,
    /// and only when no memory is allocated.
    free_lists: &'a mut [*mut FreeBlock],

    /// Our minimum block size.  This is calculated based on `heap_size`
    /// and the length of the provided `free_lists` array, and it must be
    /// big enough to contain a `FreeBlock` header object.
    min_block_size: usize,

    /// The log base 2 of our block size.  Cached here so we don't have to
    /// recompute it on every allocation (but we haven't benchmarked the
    /// performance gain).
    min_block_size_log2: u8,
}

impl<'a> Heap<'a> {
    /// Create a new heap.  `heap_base` must be aligned on a
    /// `MIN_HEAP_ALIGN` boundary, `heap_size` must be a power of 2, and
    /// `heap_size / 2.pow(free_lists.len()-1)` must be greater than or
    /// equal to `size_of::<FreeBlock>()`.  Passing in invalid parameters
    /// may do horrible things.
    pub unsafe fn new(
        heap_base: *mut u8,
        heap_size: usize,
        free_lists: &mut [*mut FreeBlock])
        -> Heap
    {
        // The heap base must not be null.
        assert!(heap_base != ptr::null_mut());

        // We must have at least one free list.
        assert!(free_lists.len() > 0);

        // Calculate our minimum block size based on the number of free
        // lists we have available.
        let min_block_size = heap_size >> (free_lists.len()-1);

        // The heap must be aligned on a 4K bounday.
        assert_eq!(heap_base as usize & (MIN_HEAP_ALIGN-1), 0);

        // The heap must be big enough to contain at least one block.
        assert!(heap_size >= min_block_size);

        // The smallest possible heap block must be big enough to contain
        // the block header.
        assert!(min_block_size >= size_of::<FreeBlock>());

        // The heap size must be a power of 2.  See:
        // http://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2
        assert!(heap_size.is_power_of_2());

        // We must have one free list per possible heap block size.
        assert_eq!(min_block_size *
                   (2u32.pow(free_lists.len() as u32 - 1)) as usize,
                   heap_size);

        // Zero out our free list pointers.
        for ptr in free_lists.iter_mut() {
            *ptr = ptr::null_mut();
        }

        // Store all the info about our heap in our struct.
        let result = Heap {
            heap_base: heap_base,
            heap_size: heap_size,
            free_lists: free_lists,
            min_block_size: min_block_size,
            min_block_size_log2: min_block_size.log2(),
        };

        // Set up the first free list, which contains exactly
        // one block the size of the entire heap.
        let header_ptr = result.heap_base as *mut FreeBlock;
        *header_ptr = FreeBlock::tail();
        let root_block_idx = result.allocation_order(heap_size, 1)
            .expect("Failed to calculate order for root heap block");
        result.free_lists[root_block_idx] = header_ptr;
        
        // Return our newly-created heap.
        result
    }

    /// Figure out what size block we'll need to fulfill an allocation
    /// request.  This is deterministic, and it does not depend on what
    /// we've already allocated.  In particular, it's important to be able
    /// to calculate the same `allocation_size` when freeing memory as we
    /// did when allocating it, or everything will break horribly.
    pub fn allocation_size(&self, mut size: usize, align: usize) -> Option<usize> {
        // Sorry, we don't support weird alignments.
        if !align.is_power_of_2() { return None; }

        // We can't align any more precisely than our heap base alignment
        // without getting much too clever, so don't bother.
        if align > MIN_HEAP_ALIGN { return None; }

        // We're automatically aligned to `size` because of how our heap is
        // sub-divided, but if we need a larger alignment, we can only do
        // it be allocating more memory.
        if align > size { size = align; }

        // We can't allocate blocks smaller than `min_block_size`.
        size = max(size, self.min_block_size);

        // Round up to the next power of two.
        size = size.next_power_of_2();

        // We can't allocate a block bigger than our heap.
        if size > self.heap_size { return None; }

        Some(size)
    }

    /// The "order" of an allocation is how many times we need to double
    /// `min_block_size` in order to get a large enough block, as well as
    /// the index we use into `free_lists`.
    pub fn allocation_order(&self, size: usize, align: usize) -> Option<usize> {
        self.allocation_size(size, align).map(|s| {
            (s.log2() - self.min_block_size_log2) as usize
        })
    }

    /// The size of the blocks we allocate for a given order.
    fn order_size(&self, order: usize) -> usize {
        1 << (self.min_block_size_log2 as usize + order)
    }

    /// Split a `block` of order `order` down into a block of order
    /// `order_needed`, placing any unused chunks on the free list.
    unsafe fn split_free_block(
        &mut self, block: *mut u8, mut order: usize, order_needed: usize)
    {
        // Get the size of our starting block.
        let mut size_to_split = self.order_size(order);

        // Progressively cut our block down to size.
        while order > order_needed {
            // Update our loop counters to describe a block half the size.
            size_to_split >>= 1;
            order -= 1;

            // Insert the "upper half" of the block into the free list.
            let split = block.offset(size_to_split as isize)
                as *mut FreeBlock;
            *split = FreeBlock::head(self.free_lists[order]);
            self.free_lists[order] = split;
        }
    }

    /// Allocate a block of memory large enough to contain `size` bytes,
    /// and aligned on `align`.  This will return NULL if the `align` is
    /// greater than `MIN_HEAP_ALIGN`, if `align` is not a power of 2, or
    /// if we can't find enough memory.
    ///
    /// All allocated memory must be passed to `deallocate` with the same
    /// `size` and `align` parameter, or else horrible things will happen.
    pub unsafe fn allocate(&mut self, size: usize, align: usize) -> *mut u8
    {
        // Figure out which order block we need.
        if let Some(order_needed) = self.allocation_order(size, align) {

            // Start with the smallest acceptable block size, and search
            // upwards until we reach blocks the size of the entire heap.
            for order in order_needed..self.free_lists.len() {

                // We found a block we can use!
                if self.free_lists[order] != ptr::null_mut() {

                    // Get the pointer we're going to return, and remove
                    // the block from the free list.
                    let allocated = self.free_lists[order] as *mut u8;
                    self.free_lists[order] =
                        (*self.free_lists[order]).next;

                    // If the block is too big, break it up.  This leaves
                    // the address unchanged, because we always allocate at
                    // the head of a block.
                    if order > order_needed {
                        self.split_free_block(allocated, order, order_needed);
                    }

                    // We have an allocation, so quit now.
                    return allocated;
                }
            }

            // We couldn't find a large enough block for this allocation.
            ptr::null_mut()
        } else {
            // We can't allocate a block with the specified size and
            // alignment.
            ptr::null_mut()
        }
    }

    /// Deallocate a block allocated using `allocate`.  Note that the
    /// `old_size` and `align` values must match the values passed to
    /// `allocate`, or our heap will be corrupted.
    #[allow(unused_variables)]
    pub unsafe fn deallocate(
        &mut self, ptr: *mut u8, old_size: usize, align: usize)
    {
        // Ah, who cares?  We have lots of RAM.
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::ptr;

    extern "C" {
        /// We need this to allocate aligned memory for our heap.
        fn memalign(alignment: usize, size: usize) -> *mut u8;

        // Release our memory.
        fn free(ptr: *mut u8);
    }

    #[test]
    fn test_allocation_size_and_order() {
        unsafe {
            let heap_size = 256;
            let mem = memalign(4096, heap_size);
            let mut free_lists: [*mut FreeBlock; 5] = [0 as *mut _; 5];
            let heap = Heap::new(mem, heap_size, &mut free_lists);

            // TEST NEEDED: Can't align beyond MIN_HEAP_ALIGN.

            // Can't align beyond heap_size.
            assert_eq!(None, heap.allocation_size(256, 256*2));

            // Simple allocations just round up to next block size.
            assert_eq!(Some(16), heap.allocation_size(0, 1));
            assert_eq!(Some(16), heap.allocation_size(1, 1));
            assert_eq!(Some(16), heap.allocation_size(16, 1));
            assert_eq!(Some(32), heap.allocation_size(17, 1));
            assert_eq!(Some(32), heap.allocation_size(32, 32));
            assert_eq!(Some(256), heap.allocation_size(256, 256));

            // Aligned allocations use alignment as block size.
            assert_eq!(Some(64), heap.allocation_size(16, 64));

            // Block orders.
            assert_eq!(Some(0), heap.allocation_order(0, 1));
            assert_eq!(Some(0), heap.allocation_order(1, 1));
            assert_eq!(Some(0), heap.allocation_order(16, 16));
            assert_eq!(Some(1), heap.allocation_order(32, 32));
            assert_eq!(Some(2), heap.allocation_order(64, 64));
            assert_eq!(Some(3), heap.allocation_order(128, 128));
            assert_eq!(Some(4), heap.allocation_order(256, 256));
            assert_eq!(None, heap.allocation_order(512, 512));

            free(mem);
        }
    }

    #[test]
    fn test_heap() {
        unsafe {
            let heap_size = 256;
            let mem = memalign(4096, heap_size);
            let mut free_lists: [*mut FreeBlock; 5] = [0 as *mut _; 5];
            let mut heap = Heap::new(mem, heap_size, &mut free_lists);

            let block_16_0 = heap.allocate(8, 8);
            assert_eq!(mem, block_16_0);

            let bigger_than_heap = heap.allocate(4096, heap_size);
            assert_eq!(ptr::null_mut(), bigger_than_heap);

            let bigger_than_free = heap.allocate(heap_size, heap_size);
            assert_eq!(ptr::null_mut(), bigger_than_free);

            let block_16_1 = heap.allocate(8, 8);
            assert_eq!(mem.offset(16), block_16_1);

            let block_16_2 = heap.allocate(8, 8);
            assert_eq!(mem.offset(32), block_16_2);

            let block_32_1 = heap.allocate(32, 32);
            assert_eq!(mem.offset(64), block_32_1);

            let block_16_3 = heap.allocate(8, 8);
            assert_eq!(mem.offset(48), block_16_3);

            let block_128_1 = heap.allocate(128, 128);
            assert_eq!(mem.offset(128), block_128_1);

            let too_fragmented = heap.allocate(64, 64);
            assert_eq!(ptr::null_mut(), too_fragmented);

            free(mem);
        }
    }
}        
