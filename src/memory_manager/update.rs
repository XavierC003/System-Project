use crate::memory_manager::memory_block::FreeBlock;  // Keep only the necessary import
use super::MemoryManager;

/// Updates the data of an allocated memory block, either by modifying the existing block or reallocating a new one.
///
/// This function first tries to find the allocated block with the given `id`. If found, it checks if the new data can fit
/// in the current block. If the new data fits, it writes the new data to the block and updates its used size. If the new data
/// does not fit, the old block is freed, and a new block is allocated to accommodate the new data.
///
/// # Arguments
///
/// * `manager` - A mutable reference to the `MemoryManager` instance which manages the memory blocks.
/// * `id` - The unique identifier of the allocated block to be updated.
/// * `new_data` - A slice of bytes representing the new data to be written into the block.
///
/// # Returns
///
/// This function does not return a value. However, it performs the update or reallocation of the block based on the provided data.
///
pub fn update(manager: &mut MemoryManager, id: usize, new_data: &[u8]) {
    if let Some(index) = manager.allocated_handles.iter().position(|b| b.id == id) {
        // Get the block from the allocated handles list
        let mut block = manager.allocated_handles.remove(index);

        // Check if the new data fits into the current block
        if new_data.len() <= block.size {
            // Write the new data to the block's memory space
            manager.write_bytes(block.start, new_data);
            block.used_size = new_data.len(); // Update the used size of the block

            // Insert the updated block back in place at the same index
            manager.allocated_handles.insert(index, block); 
        } else {
            // Free the old block since it can't accommodate the new data
            manager.free_handles.push(FreeBlock::new(block.start, block.size));

            // Try to insert the new data into memory
            if let Some(new_id) = manager.insert(new_data.len(), new_data) {
                // Find the newly allocated block and update its ID
                let new_block = manager.allocated_handles
                    .iter_mut()
                    .find(|b| b.id == new_id)
                    .unwrap();
                new_block.id = id; // Set the same ID as the old block
            } else {
                // If insert fails, it means there wasn't enough space
                println!("Update failed: not enough memory for new data.");
            }
        }
    } else {
        // If no block is found with the given ID
        println!("Nothing at [{}]", id);
    }
}
