use crate::memory_manager::memory_block::allocated_block::AllocatedBlock;

/// Searches for an allocated memory block by its unique ID.
///
/// Iterates through a slice of `AllocatedBlock` instances and returns a reference
/// to the block that matches the provided ID, if found.
///
/// # Arguments
///
/// * `blocks` - A slice of `AllocatedBlock` instances to search through.
/// * `id` - The unique identifier of the memory block to find.
///
/// # Returns
///
/// * `Some(&AllocatedBlock)` if a block with the given ID is found.
/// * `None` if no block with the given ID exists.
// The function does not need the 'memory' argument if it's not used in the function
pub fn find_block<'a>(blocks: &'a [AllocatedBlock], id: usize) -> Option<&'a AllocatedBlock> {
    blocks.iter().find(|b| b.id == id)
}
