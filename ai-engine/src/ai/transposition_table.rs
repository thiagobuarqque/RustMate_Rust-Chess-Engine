use std::collections::HashMap;

use crate::game_bit_board::_move::_move::Move;

pub struct TranspositionTableEntry {
    pub depth: u8,
    pub value: f32,
    pub best_move: Option<Move>,
}

impl TranspositionTableEntry {
    pub fn estimated_size(&self) -> usize {
        std::mem::size_of::<u8>()
            + std::mem::size_of::<f32>()
            + std::mem::size_of::<Option<Move>>()
    }
}

pub struct TranspositionTable {
    table: HashMap<(u64, u8), TranspositionTableEntry>,
    hits: u64,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
            hits: 0,
        }
    }

    pub fn store(&mut self, key: (u64, u8), entry: TranspositionTableEntry) {
        // Handle entry storage, collision resolution, etc.
        self.table.insert(key, entry);
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn retrieve(&mut self, key: (u64, u8)) -> Option<&TranspositionTableEntry> {
        let entry = self.table.get(&key);

        if entry.is_some() {
            self.hits += 1
        }

        entry
    }

    pub fn get_hits(&self) -> u64 {
        self.hits
    }

    pub fn estimated_memory_usage_kb(&self) -> usize {
        let entry_size = if let Some(entry) = self.table.values().next() {
            entry.estimated_size()
        } else {
            return 0;
        };

        let total_size = entry_size * self.table.len();
        total_size / 1024
    }
}
