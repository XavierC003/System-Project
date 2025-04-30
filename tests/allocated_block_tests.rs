use rust_template::memory_manager::memory_block::allocated_block::AllocatedBlock;
use rust_template::memory_manager::memory_block::MemoryBlock;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocated_block_creation() {
        let block = AllocatedBlock::new(10, 100, 1, 50);
        assert_eq!(block.get_start(), 10);
        assert_eq!(block.get_size(), 100);
        assert_eq!(block.id, 1);
        assert_eq!(block.used_size, 50);
    }

    #[test]
    fn test_update_used_size_valid() {
        let mut block = AllocatedBlock::new(0, 100, 2, 25);
        let result = block.update_used_size(50);
        assert!(result.is_ok());
        assert_eq!(block.used_size, 50);
    }

    #[test]
    fn test_update_used_size_invalid() {
        let mut block = AllocatedBlock::new(0, 100, 3, 25);
        let result = block.update_used_size(150);
        assert!(result.is_err());
        assert_eq!(block.used_size, 25); // Should remain unchanged
    }
}
