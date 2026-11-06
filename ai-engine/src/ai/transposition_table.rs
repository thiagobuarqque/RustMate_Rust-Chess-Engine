use crate::game_bit_board::_move::_move::Move;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntryFlag {
    Upperbound = 0,
    Lowerbound = 1,
    Exact = 2,
}

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry {
    pub key: u64,
    pub depth: u8,
    pub value: f32,
    pub best_move: Option<Move>,
    pub flag: EntryFlag,
}

/// Fixed-size table indexed by the lower bits of the zobrist hash
pub struct TranspositionTable {
    entries: Vec<Option<TranspositionTableEntry>>,
    hits: u64,
}

impl TranspositionTable {
    /// Must be a power of two
    const SIZE: usize = 1 << 20;

    pub fn new() -> Self {
        TranspositionTable {
            entries: vec![None; Self::SIZE],
            hits: 0,
        }
    }

    fn index(key: u64) -> usize { key as usize & (Self::SIZE - 1) }

    /// Always replaces the existing entry, recent positions are the most
    /// relevant
    pub fn store(&mut self, entry: TranspositionTableEntry) {
        self.entries[Self::index(entry.key)] = Some(entry);
    }

    pub fn retrieve(&mut self, key: u64) -> Option<TranspositionTableEntry> {
        let entry = self.entries[Self::index(key)].filter(|entry| entry.key == key);

        if entry.is_some() {
            self.hits += 1
        }

        entry
    }

    pub fn get_hits(&self) -> u64 { self.hits }

    pub fn reset_hits(&mut self) { self.hits = 0 }

    pub fn estimated_memory_usage_kb(&self) -> usize {
        self.entries.len() * std::mem::size_of::<Option<TranspositionTableEntry>>() / 1024
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryFlag, TranspositionTable, TranspositionTableEntry};

    fn entry(key: u64) -> TranspositionTableEntry {
        TranspositionTableEntry {
            key,
            depth: 3,
            value: 42.0,
            best_move: None,
            flag: EntryFlag::Exact,
        }
    }

    #[test]
    fn test_retrieve_stored_entry() {
        let mut table = TranspositionTable::new();

        table.store(entry(7));

        assert_eq!(42.0, table.retrieve(7).unwrap().value);
        assert_eq!(1, table.get_hits());
    }

    #[test]
    fn test_retrieve_ignores_other_position_in_same_slot() {
        let mut table = TranspositionTable::new();

        table.store(entry(7));

        assert!(table.retrieve(7 + TranspositionTable::SIZE as u64).is_none());
        assert_eq!(0, table.get_hits());
    }
}
