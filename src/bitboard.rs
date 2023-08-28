use crate::position::{Position, FILES};

#[derive(Clone, Debug, PartialEq)]
pub struct BitBoard {
    grid: u64,
    diag: u64,
}

impl BitBoard {
    fn get_bit_idx(pos: Position) -> i32 {
        if pos.is_diagonal() {
            (pos.rank / 2) * FILES.end() + (pos.file / 2)
        } else {
            ((pos.rank - 1) / 2) * FILES.end() + ((pos.file - 1) / 2)
        }
    }
    
    fn set_bit_at(data: &u64, idx: i32, val: bool) -> u64 {
        if val {
            data | (1 << idx)
        } else {
            data & !(1 << idx)
        }
    }

    pub fn is_set(&self, pos: Position) -> bool {
        if pos.is_diagonal() {
            self.diag & (1 << Self::get_bit_idx(pos)) != 0
        } else {
            self.grid & (1 << Self::get_bit_idx(pos)) != 0
        }
    }

    pub fn set(&mut self, pos: Position, val: bool) {
        if pos.is_diagonal() {
            Self::set_bit_at(&self.diag, Self::get_bit_idx(pos), val);
        } else {
            Self::set_bit_at(&self.grid, Self::get_bit_idx(pos), val);
        }
    }
}