use rust_template::memory_manager::{dump, update, MemoryManager};



#[test]
fn test_insert_block() {
    let mut manager = MemoryManager::new(100);
    let result = manager.insert(20, &[1, 2, 3]);
    assert!(result.is_some());
}

#[test]
fn test_read_block() {
    let mut manager = MemoryManager::new(100);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();
    let block = manager.read(&id);
    assert!(block.is_some());
    assert_eq!(block.unwrap().id, id);
}

#[test]
fn test_delete_block() {
    let mut manager = MemoryManager::new(100);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();
    let deleted = manager.delete(&id);
    assert!(deleted);
    assert!(manager.read(&id).is_none());
}

#[test]
fn test_find_block() {
    let mut manager = MemoryManager::new(100);
    let id = manager.insert(20, &[1, 2, 3]).unwrap();
    
    // Now, calling the `find` method, which internally uses `find_block`
    let block = manager.find(&id);
    
    // Assert the block exists and matches the id
    assert!(block.is_some());
    assert_eq!(block.unwrap().id, id);
}


#[test]
fn test_dump_memory_state() {
    let mut manager = MemoryManager::new(100);
    manager.insert(20, &[1, 2, 3]);

    // Capture the output into a string
    let dump_output = dump::dump(&manager);

    // Print the output for debugging
    println!("Captured dump output: {}", dump_output);

    // Test if the expected text is present
    assert!(dump_output.trim().contains("Allocated"));
    assert!(dump_output.trim().contains("Free"));
}

#[test]
fn test_update_block() {
    let mut manager = MemoryManager::new(100);
    let block_id = manager.insert(10, &[1, 2, 3, 4, 5]).unwrap();

    // Perform update
    update::update(&mut manager, &block_id, &[1, 2, 9, 9, 5]);

    // Get the updated block
    let block = manager.read(&block_id).expect("Block should exist");

    // Read updated memory content
    let updated_data = manager.read_range(block.start, block.used_size).unwrap();

    assert_eq!(updated_data, vec![1, 2, 9, 9, 5]);
}


