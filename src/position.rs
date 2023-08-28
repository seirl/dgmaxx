use std::ops::RangeInclusive;

use crate::players::PlayerColor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub rank: i32,
    pub file: i32,
}

pub const RANKS: RangeInclusive<i32> = 1..=13;
pub const FILES: RangeInclusive<i32> = 1..=11;

impl Position {
    pub fn new(rank: i32, file: i32) -> Position {
        Position { rank, file }
    }

    pub fn is_valid(&self) -> bool {
        // Is within bounds.
        if !RANKS.contains(&self.rank) || !FILES.contains(&self.file) {
            return false;
        }
        // Diagonal squares: both must be even or odd.
        if (self.rank % 2 == 0) != (self.file % 2 == 0) {
            return false;
        }
        return true;
    }

    pub fn is_diagonal(&self) -> bool {
        assert!(self.is_valid());
        return self.rank % 2 == 0;
    }

    pub fn is_stronghold(&self) -> Option<PlayerColor> {
        if [Position::new(1, 5), Position::new(2, 6), Position::new(1, 7)].contains(self) {
            return Some(PlayerColor::South);
        }
        if [Position::new(13, 5), Position::new(12, 6), Position::new(13, 7)].contains(self) {
            return Some(PlayerColor::North);
        }
        return None;
    }
}