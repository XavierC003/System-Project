use crate::memory_manager::MemoryManager;
use crate::memory_manager::memory_block::FreeBlock;
use crate::memory_manager::memory_block::MemoryBlock;


impl MemoryManager {
    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(index) = self.allocated_handles.iter().position(|block| block.id == id) {
            let block = self.allocated_handles.remove(index);
            
            println!("Deleted block with ID: {}", block.id);  // Debugging statement
    
            let free_block = FreeBlock::new(block.get_start(), block.get_size());
            println!("FreeBlock created with start: {}, size: {}", free_block.get_start(), free_block.get_size());  // Debugging statement
    
            self.free_handles.push(free_block);
            true
        } else {
            println!("Block ID not found for deletion: {}", id);  // Debugging statement
            false
        }
    }
}
