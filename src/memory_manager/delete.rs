use crate::memory_manager::merge_free_blocks;
use crate::memory_block::FreeBlock;
use super::MemoryManager;

pub fn delete(manager: &mut MemoryManager, id: &str) -> bool {
    if let Some(index) = manager.allocated_blocks.iter().position(|b| b.id == id) {
        let block = manager.allocated_blocks.remove(index);
        manager.free_blocks.push(FreeBlock::new(block.start, block.size));
        merge_free_blocks(manager);
        return true;
    }
    false
}

pub fn print_block_info<T: memory_block::MemoryBlock>(block: &T) {
    println!("Start: {}, Size: {}", block.get_start(), block.get_size());
}