use crate::memory_manager::memory_block::allocated_block::AllocatedBlock;

// The function does not need the 'memory' argument if it's not used in the function
pub fn find_block<'a>(blocks: &'a [AllocatedBlock], id: &'a str) -> Option<&'a AllocatedBlock> {
    blocks.iter().find(|b| b.id == id)
}
