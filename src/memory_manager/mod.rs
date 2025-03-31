mod memory_block;

use memory_block::allocated_block::AllocatedBlock;
use memory_block::free_block::FreeBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
}

impl MemoryManager {}
