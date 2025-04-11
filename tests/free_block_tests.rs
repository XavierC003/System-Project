use rust_template::memory_manager::memory_block::free_block::FreeBlock;

#[test]
fn test_free_block_creation() {
    let free = FreeBlock::new(5, 200);
    assert_eq!(free.get_start(), 5);
    assert_eq!(free.get_size(), 200);
}
