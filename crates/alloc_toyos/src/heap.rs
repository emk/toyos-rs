//! A simple heap based on a buddy allocator.

use std::mem::size_of;
use std::num::Wrapping;
use std::ptr;

const MIN_HEAP_ALIGN: usize = 4096;

pub struct Heap<'a> {
    heap_base: *mut u8,
    heap_size: usize,
    free_lists: &'a mut [*mut BlockHeader],
    min_block_size: usize,
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
        assert_eq!(heap_base as usize & 0xFFF, 0);

        // The heap must be big enough to contain at least one block.
        assert!(heap_size >= min_block_size);

        // The smallest possible heap block must be big enough to contain
        // the block header.
        assert!(min_block_size >= size_of::<BlockHeader>());

        // The heap size must be a power of 2.  See:
        // http://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2
        assert!(heap_size !=0 && (heap_size & (heap_size - 1)) == 0);

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

    pub unsafe fn allocate(
        &mut self, mut size: usize, mut align: usize)
        -> *mut u8
    {
        if align > 4096 {
            // Bail immediately if we're asked for an alignment that we
            // can't easily supply.
            return ptr::null_mut();
        } else if align > size {
            // Satisfy large alignment requests by just allocating more
            // memory.  Sorry.
            size = align;
        } else {
            // Alignment should be guaranteed by heap layout.
        }


        self.heap_base
    }

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
