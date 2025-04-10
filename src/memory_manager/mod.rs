mod memory_block;

use memory_block::allocated_block::{AllocatedBlock, MemoryBlock};
use memory_block::free_block::FreeBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
}

impl MemoryManager {

    /// Create a new MemoryManager with the given size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size], // Initialize the data vector with the given size
            free_handles: vec![], // Start with an empty list of free blocks
            allocated_handles: vec![], // Start with an empty list of allocated blocks
        }
    }

    // Frees an allocated block and adds it back to free_handles
    pub fn free(&mut self, block: AllocatedBlock) {
        self.free_handles.push(FreeBlock::new(block.get_start(), block.get_size()));
        self.allocated_handles.retain(|b| b.get_start() != block.get_start());
    }
}