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

        match id {
        Some(id) => {
            // Allocated block: show in hex, compact, with ID and size
            output.push_str(&format!(
                "0x{:04X} - 0x{:04X} ALLOCATED (ID: {}) (Size: {} bytes)\n",
                start, end, id, size
            ));

            // Optional: show first few bytes of data
            let data = &self.data[start..start + size];
            let shown = data.iter().take(16)  // show up to 16 bytes for cleanliness
                .map(|b| format!("0x{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            output.push_str(&format!("{}\n", shown));
        }
        None => {
            // Free block: compact style in hex
            output.push_str(&format!(
                "0x{:04X} - 0x{:04X} {} (Size: {} bytes)\n",
                start, end, status, size
            ));
        }
    }
}
    output
}
}
