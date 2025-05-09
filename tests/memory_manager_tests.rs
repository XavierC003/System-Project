use rust_template::memory_manager::MemoryManager;



#[test]
fn test_insert_block() {
    let mut manager = MemoryManager::new(65535);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();  // id is String

    // Ensure the id is being passed correctly
    let block = manager.read(id);  // Pass &String
    assert!(block.is_some());
    assert_eq!(block.unwrap().id, id); // ID should match directly
}

#[test]
fn test_read_block() {
    let mut manager = MemoryManager::new(65535);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();

    // Read the block using its ID
    let block = manager.read(id);
    assert!(block.is_some());
    assert_eq!(block.unwrap().id, id);
}

#[test]
fn test_delete_block() {
    let mut manager = MemoryManager::new(65535);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();

    // Delete the block
    let deleted = manager.delete(id);
    assert!(deleted);

    // Ensure the block was deleted
    let block = manager.read(id);
    assert!(block.is_none());
}

#[test]
fn test_find_block() {
    let mut manager = MemoryManager::new(65535);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();

    // Find the block by its ID
    let block = manager.find(id);

    assert!(block.is_some());
    assert_eq!(block.unwrap().id, id);
}

#[test]
fn test_dump_memory_state() {
    let mut manager = MemoryManager::new(65535);
    manager.insert(20, &[1, 2, 3]);

    let dump_output = manager.dump();
    println!("Captured dump output: {}", dump_output);

    assert!(dump_output.contains("ALLOCATED"));
    assert!(dump_output.contains("FREE"));
}

#[test]
fn test_update_block() {
    let mut manager = MemoryManager::new(65535);
    let block_id = manager.insert(10, &[1, 2, 3, 4, 5]).unwrap();

    // Update the block
    MemoryManager::update( &mut manager, block_id, &[1, 2, 9, 9, 5]);

    let block = manager.read(block_id).expect("Block should exist");
    let updated_data = manager.read_range(block.start, block.used_size).unwrap();

    assert_eq!(updated_data, vec![1, 2, 9, 9, 5]);
}

#[test]
fn test_buddy_merge_on_delete() {
    let mut manager = MemoryManager::new(65535);

    let id1 = manager.insert(16, b"block_one").unwrap(); 
    let id2 = manager.insert(16, b"block_two").unwrap();

    assert!(manager.delete(id1));
    assert!(manager.delete(id2));

    let dump = manager.dump();
    println!("{}", dump);

    assert!(dump.contains("Size: 16"));
    assert!(dump.contains("Status: FREE"));
}
