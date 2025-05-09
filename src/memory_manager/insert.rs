use super::{AllocatedBlock, FreeBlock, MemoryManager};

impl MemoryManager {
    /// Inserts a new block of memory into the memory manager.
    ///
    /// This function finds the best-fit free block that can hold the requested size,
    /// allocates a new `AllocatedBlock`, and copies the provided data into memory.
    ///
    /// # Arguments
    ///
    /// * `size` - The requested size for the allocation. It will be rounded up
    ///            to the next power of two for alignment and efficiency.
    /// * `data` - A byte slice containing the data to be copied into the allocated block.
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The unique ID of the newly allocated block, if allocation succeeds.
    /// * `None` - If no suitable free block is found or the provided data exceeds the required size.
    pub fn insert(&mut self, size: usize, data: &[u8]) -> Option<usize> {
        let required_size = size.next_power_of_two();

        // Reject if data is too big for the requested block
        if data.len() > required_size {
            return None;
        }

        // Sort free blocks by size to ensure best fit
        self.free_handles.sort_by_key(|b| b.get_size());

        // Find the smallest free block that can fit the requested size
        let index = self
            .free_handles
            .iter()
            .position(|block| block.get_size() >= required_size)?;

        // Remove the selected free block
        let mut block = self.free_handles.remove(index);

        // Split block in halves until we get the exact size needed
        while block.get_size() > required_size {
            let half_size = block.get_size() / 2;
            let buddy = FreeBlock::new(block.get_start() + half_size, half_size);
            self.free_handles.push(buddy);
            block = FreeBlock::new(block.get_start(), half_size);
            self.free_handles.sort_by_key(|b| b.get_size());

            // Re-sort after inserting buddy to maintain order
            self.free_handles.sort_by_key(|b| b.get_size());
        }

        // Allocate the block
        let id = self.next_id;
        self.next_id += 1;

        assert!(block.get_size().is_power_of_two(), "Block size is not a power of two: {}", block.get_size());



        self.allocated_handles.push(AllocatedBlock::new(
            block.get_start(),
            block.get_size(),
            id,
            data.len(),
        ));

        // Copy data into memory
        self.data[block.get_start()..block.get_start() + data.len()]
            .copy_from_slice(data);

        Some(id)
    }
}