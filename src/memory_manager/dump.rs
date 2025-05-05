use super::{memory_block::MemoryBlock, MemoryManager};

    /// Generates a formatted dump of the memory state managed by the `MemoryManager`.
///
/// This includes both allocated and free memory blocks, showing their start and end
/// addresses, sizes, statuses, and IDs (if applicable). Allocated blocks will include
/// their unique identifier, while free blocks will not.
///
/// # Arguments
///
/// * `manager` - A reference to the `MemoryManager` whose memory state will be displayed.
///
/// # Returns
///
/// * A `String` containing the formatted memory dump, with one line per memory block.
///
impl MemoryManager {
pub fn dump(&self) -> String {
    let mut all_blocks: Vec<(usize, usize, String, Option<usize>)> = Vec::new();

    // Add allocated blocks
    for block in &self.allocated_handles {
        all_blocks.push((
            block.get_start(),
            block.get_size(),
            "ALLOCATED".to_string(),
            Some(block.id),
        ));
    }

    // Add free blocks
    for block in &self.free_handles {
        all_blocks.push((
            block.start,
            block.size,
            "FREE".to_string(),
            None,
        ));
    }

    // Sort blocks by start address
    all_blocks.sort_by_key(|&(start, _, _, _)| start);

    // Build output string
    let mut output = String::new();
    for (start, size, status, id) in all_blocks {
        let end = start + size - 1;
        let line = match id {
            Some(id) => format!(
                "Start: {:04X}, End: {:04X}, Size: {}, Status: {}, ID: {}\n",
                start, end, size, status, id
            ),
            None => format!(
                "Start: {:04X}, End: {:04X}, Size: {}, Status: {}\n",
                start, end, size, status
            ),
        };
        output.push_str(&line);
    }

    output
}
}