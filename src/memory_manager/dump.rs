use super::MemoryManager;

pub fn dump(manager: &MemoryManager) -> String {
    let mut output = String::new();
    output.push_str(&format!("Memory Data: {:?}\n", manager.data));
    output.push_str(&format!("Free Handles: {:?}\n", manager.free_handles));
    output.push_str(&format!("Allocated Handles: {:?}\n", manager.allocated_handles));
    output
}
