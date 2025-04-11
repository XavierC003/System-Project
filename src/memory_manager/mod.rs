pub mod memory_block;
pub use memory_block::{FreeBlock, allocated_block};

use crate::memory_manager::memory_block::allocated_block::AllocatedBlock;
use crate::memory_manager::memory_block::MemoryBlock;





pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
}

impl MemoryManager {
    /// Create a new MemoryManager with the given size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            free_handles: vec![],
            allocated_handles: vec![],
        }
    }
        /// Read a byte from the memory at a given index
    pub fn read_byte(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
        }
    
        /// Write a byte to the memory at a given index
    pub fn write_byte(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            self.data[index] = value;
            }
    }

    /// Free an allocated block
    pub fn free(&mut self, block: AllocatedBlock) {
        self.free_handles
            .push(FreeBlock::new(block.get_start(), block.get_size()));
        self.allocated_handles
            .retain(|b| b.get_start() != block.get_start());
    }
}
