//! A simple heap based on a buddy allocator.

use std::cmp::max;
use std::mem::size_of;
use std::ptr;

use math::PowersOf2;

const MIN_HEAP_ALIGN: usize = 4096;

#[allow(dead_code)]
pub struct Heap<'a> {
    heap_base: *mut u8,
    heap_size: usize,
    free_lists: &'a mut [*mut BlockHeader],
    min_block_size: usize,
    min_block_size_log2: u8,
}

pub struct BlockHeader {
    next: *mut u8,
}

impl<'a> Heap<'a> {
    pub unsafe fn new(
        heap_base: *mut u8,
        heap_size: usize,
        free_lists: &mut [*mut BlockHeader])
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
        assert!(min_block_size >= size_of::<BlockHeader>());

        // The heap size must be a power of 2.  See:
        // http://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2
        assert!(heap_size.is_power_of_2());

        // We must have one free list per possible heap block size.
        assert_eq!(min_block_size *
                   (2u32.pow(free_lists.len() as u32 - 1)) as usize,
                   heap_size);

        // Store all the info about our heap in our struct.
        let result = Heap {
            heap_base: heap_base,
            heap_size: heap_size,
            free_lists: free_lists as &mut [*mut BlockHeader],
            min_block_size: min_block_size,
            min_block_size_log2: min_block_size.log2(),
        };

        // Set up the first free list, which contains exactly
        // one block the size of the entire heap.
        let header = result.heap_base as *mut BlockHeader;
        (*header).next = ptr::null_mut();
        let root_block_idx = result.free_lists.len() - 1;
        result.free_lists[root_block_idx] = header;
        
        // Return our newly-created heap.
        result
    }

    /// Figure out what size block we'll need to fulfill an allocation
    /// request.  This is deterministic, and it does not depend on what
    /// we've already allocated.  In particular, it's important to be able
    /// to calculate the same `allocation_size` when freeing memory as we
    /// did when allocating it, or
    pub fn allocation_size(&self, mut size: usize, align: usize) -> Option<usize> {
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

    pub fn allocation_order(&self, size: usize, align: usize) -> Option<u8> {
        self.allocation_size(size, align).map(|s| {
            s.log2() - self.min_block_size_log2
        })
    }

    #[allow(unused_variables)]
    pub unsafe fn allocate(
        &mut self, size: usize, align: usize)
        -> *mut u8
    {
        self.heap_base
    }

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

    //use std::ptr;

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
            let mut free_lists: [*mut BlockHeader; 5] = [0 as *mut _; 5];
            let heap = Heap::new(mem, heap_size, &mut free_lists);

            // TODO: Can't align beyond MIN_HEAP_ALIGN.

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
            let mut free_lists: [*mut BlockHeader; 5] = [0 as *mut _; 5];
            let mut heap = Heap::new(mem, heap_size, &mut free_lists);

            let bottom_small = heap.allocate(8, 8);
            assert_eq!(mem, bottom_small);

            //let not_enough_space = heap.allocate(4096, heap_size);
            //assert_eq!(ptr::null_mut(), not_enough_space);

            free(mem);
        }
    }
}        
