use rust_template::file_parser;
use rust_template::memory_manager::MemoryManager;
use rust_template::memory_manager::memory_block::MemoryBlock; // Added for MemoryBlock trait

use std::env;
use std::process;

fn main() {
    // Create a new memory manager with 1024 bytes
    let mut manager = MemoryManager::new(1024);

    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file_path.cmmd>");
        process::exit(1);
    }
    let file_path = &args[1];

    // Ensure the file has the correct extension
    if !file_path.ends_with(".cmmd") {
        eprintln!("File must have a .cmmd extension");
        process::exit(1);
    }

    // Parse the file using FileParser
    let file_parser = match file_parser::FileParser::new(file_path) {
        Ok(parser) => parser,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    // Process each command from the file
    for command in file_parser.commands {
        match command.function() {
            "INSERT" => {
                // INSERT [SIZE] [DATA...]
                if let Some(size_str) = command.parameters().get(0) {
                    match size_str.parse::<usize>() {
                        Ok(size) => {
                            // Join the remaining parameters as data and convert to bytes
                            let data = command.parameters()[1..].join(" ");
                            let data_bytes = data.as_bytes();

                            if let Some(id) = manager.insert(size, data_bytes) {
                                println!("Inserted block with ID: {}", id);
                                if let Some(block) = manager.get_block(id) {
                                    print_block_info(block);
                                }
                            } else {
                                println!("Failed to insert block.");
                            }
                        }
                        Err(_) => println!("Invalid size parameter for INSERT."),
                    }
                } else {
                    println!("Missing size parameter for INSERT.");
                }
            }

            "DELETE" => {
                // DELETE [ID]
                if let Some(id_str) = command.parameters().get(0) {
                    match id_str.parse::<usize>() {
                        Ok(id) => {
                            if manager.delete(id) {
                                println!("Deleted block with ID: {}", id);
                            } else {
                                println!("Block ID not found: {}", id);
                            }
                        }
                        Err(_) => println!("Invalid block ID for DELETE."),
                    }
                } else {
                    println!("Missing block ID for DELETE.");
                }
            }

            "READ" => {
                // READ [ID]
                if let Some(id_str) = command.parameters().get(0) {
                    match id_str.parse::<usize>() {
                        Ok(id) => {
                            if let Some(block) = manager.get_block(id) {
                                let data = manager.read_range(block.get_start(), block.used_size);
                                println!("Block {}: {:?}", id, data.unwrap_or_default());
                            } else {
                                println!("Nothing at [{}]", id);
                            }
                        }
                        Err(_) => println!("Invalid ID for READ."),
                    }
                } else {
                    println!("Missing ID for READ.");
                }
            }

            "UPDATE" => {
                // UPDATE [ID] [NEW_DATA...]
                if let Some(id_str) = command.parameters().get(0) {
                    match id_str.parse::<usize>() {
                        Ok(id) => {
                            let data = command.parameters()[1..].join(" ");
                            let data_bytes = data.as_bytes();
                            MemoryManager::update(&mut manager, id, data_bytes);
                        }
                        Err(_) => println!("Invalid ID for UPDATE."),
                    }
                } else {
                    println!("Missing ID for UPDATE.");
                }
            }

            "DUMP" => {
                // DUMP
                print!("{}", manager.dump());
            }

            _ => {
                // Unknown command fallback
                println!("Unknown command: {}", command.function());
            }
        }
    }
}

// Function to print block information
fn print_block_info(block: &impl MemoryBlock) {
    println!("Block Start: {}, Block Size: {}", block.get_start(), block.get_size());
}