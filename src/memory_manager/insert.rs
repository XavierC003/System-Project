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
        let data_size = data.len();

        if data_size > required_size {
            return None;
        }

        // Sort free blocks to find smallest possible suitable buddy
        self.free_handles.sort_by_key(|b| b.get_size());

        let mut block_index = None;

        // Find the smallest free block that can fit the required size
        for (i, block) in self.free_handles.iter().enumerate() {
            if block.get_size() >= required_size {
                block_index = Some(i);
                break;
            }
        }

        let index = block_index?;

        // Start splitting if necessary to reach the exact required size
        let mut block = self.free_handles.remove(index);

        while block.get_size() > required_size {
            // Split the block in half
            let half_size = block.get_size() / 2;
            let buddy = FreeBlock::new(block.get_start() + half_size, half_size);
            self.free_handles.push(buddy);
            block = FreeBlock::new(block.get_start(), half_size);
        }

        // At this point, `block` is exactly the size we need
        let id = self.next_id;
        self.next_id += 1;

        self.allocated_handles.push(AllocatedBlock::new(
            block.get_start(),
            block.get_size(),
            id,
            size,
        ));

        // Copy data into memory
        let len_to_copy = data.len().min(required_size);
        self.data[block.get_start()..block.get_start() + len_to_copy]
            .copy_from_slice(data);

        Some(id)
    }
}