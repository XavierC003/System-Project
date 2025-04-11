use crate::memory_block::FreeBlock;
use super::MemoryManager;

pub fn delete(manager: &mut MemoryManager, id: &str) -> bool {
    // Find the block with the specified id in allocated_handles
    if let Some(index) = manager.allocated_handles.iter().position(|b| b.id == id) {
        // Remove the block from allocated_handles
        let block = manager.allocated_handles.remove(index);
        
        // Add the freed block to free_handles
        manager.free_handles.push(FreeBlock::new(block.get_start(), block.get_size()));
        
        println!("Deleted block with id: {}", id);
        return true;
    }
    // Return false if no block was found with the given id
    println!("No block found with id: {}", id);
    false
}

