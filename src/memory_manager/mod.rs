pub mod memory_block;
pub use memory_block::{FreeBlock, allocated_block};

pub mod insert;
pub mod delete;
pub mod read;
pub mod find;
pub mod update;
pub mod dump;

use crate::memory_manager::memory_block::allocated_block::AllocatedBlock;
use crate::memory_manager::memory_block::MemoryBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
    next_id: usize,
}

impl MemoryManager {
        /// Creates a new `MemoryManager` with the specified memory size.
    ///
    /// # Arguments
    ///
    /// * `size` - The total size of memory to manage.
    ///
    /// # Returns
    ///
    /// A new `MemoryManager` instance with one free block covering the entire memory.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            free_handles: vec![FreeBlock::new(0, size)],
            allocated_handles: vec![],
            next_id: 0,
        }
    }

    pub fn show_memory(&self) {
        dump::dump(self);  // Call dump function from dump.rs
    }

    // Find method that retrieves a block by ID
    pub fn find<'a>(&'a self, id: &'a str) -> Option<&'a AllocatedBlock> {
        find::find_block(&self.allocated_handles, id)
    }

    pub fn read_range(&self, start: usize, len: usize) -> Option<Vec<u8>> {
        if start + len <= self.data.len() {
            Some(self.data[start..start + len].to_vec())
        } else {
            None
        }
    }

    pub fn write_bytes(&mut self, start: usize, data: &[u8]) {
        let end = start + data.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(data);
        }
    }

    pub fn get_block(&self, id: &str) -> Option<&AllocatedBlock> {
        self.allocated_handles.iter().find(|&block| block.id == id)
    }

    pub fn free(&mut self, block: AllocatedBlock) {
        self.free_handles
            .push(FreeBlock::new(block.get_start(), block.get_size()));
        self.allocated_handles
            .retain(|b| b.get_start() != block.get_start());
    }
}

