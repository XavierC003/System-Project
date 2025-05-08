use crate::memory_manager::memory_block::FreeBlock;
use crate::memory_manager::memory_block::MemoryBlock;

use super::MemoryManager;


impl MemoryManager {
        /// Deletes an allocated memory block by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the ID of the block to be deleted.
    ///
    /// # Returns
    ///
    /// * `true` if the block with the specified ID was found and deleted successfully.
    /// * `false` if no block with the given ID exists.
    ///
    /// # Behavior
    ///
    /// When a block is deleted, it is removed from the `allocated_handles` list and
    /// converted into a `FreeBlock`, which is then added to the `free_handles` list.
    ///
    pub fn delete(&mut self, id: usize) -> bool {
        if let Some(index) = self.allocated_handles.iter().position(|block| block.id == id) {
            let block = self.allocated_handles.remove(index);
            let mut free_block = FreeBlock::new(block.get_start(), block.get_size());
            
            loop {
                // Try to find a buddy block
                if let Some(buddy_index) = self.free_handles.iter().position(|b| b.is_buddy_of(&free_block)) {
                    // Merge with buddy block
                    let buddy = self.free_handles.remove(buddy_index);
                    let merged_start = usize::min(free_block.get_start(), buddy.get_start());
                    let merged_size = free_block.get_size() * 2;
                    free_block = FreeBlock::new(merged_start, merged_size);

                } else {
                    self.free_handles.push(free_block);
                    break;  // No more buddies to merge with
                }
            }
    
            true
        } else {
            println!("Block ID not found for deletion: {}", id);
            false
        }
    }
}
