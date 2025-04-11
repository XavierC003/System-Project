use crate::memory_manager::MemoryManager;

impl MemoryManager {
    pub fn read(&self, id: &str) -> Option<&crate::memory_manager::memory_block::allocated_block::AllocatedBlock> {
        self.allocated_handles.iter().find(|block| block.id == id)
    }
}

