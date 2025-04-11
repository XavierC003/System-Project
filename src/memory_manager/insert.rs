use super::{AllocatedBlock, FreeBlock, MemoryManager};

impl MemoryManager {
    pub fn insert(&mut self, size: usize, data: &[u8]) -> Option<String> {
        let required_size = size.next_power_of_two();
        let data_size = data.len();

        if data_size > required_size {
            return None;
        }

        let best_fit_index = self.free_handles
            .iter()
            .enumerate()
            .filter(|(_, b)| b.get_size() >= required_size)
            .min_by_key(|(_, b)| b.get_size())
            .map(|(i, _)| i);

        if let Some(index) = best_fit_index {
            let free_block = self.free_handles.remove(index);
            let id = self.next_id.to_string();
            self.next_id += 1;

            self.allocated_handles.push(AllocatedBlock::new(
                free_block.get_start(),
                required_size,
                id.clone(),
                size,
            ));

            let len_to_copy = data.len().min(required_size);
            self.data[free_block.get_start()..free_block.get_start() + len_to_copy]
                .copy_from_slice(data);

            if free_block.get_size() > required_size {
                self.free_handles.push(FreeBlock::new(
                    free_block.get_start() + required_size,
                    free_block.get_size() - required_size,
                ));
            }

            Some(id)
        } else {
            None
        }
    }
}
