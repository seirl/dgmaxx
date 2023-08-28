use crate::players::PlayerColor;
use crate::bitboard::BitBoard;
use crate::position::Position;
use enum_map::{EnumMap, Enum};

#[derive(Clone, Copy, PartialEq, Enum, Debug)]
enum Piece {
    Treto,
    Diago,
    Stello,
}

#[derive(Clone, Debug, PartialEq)]
struct PlayerPieces {
    pieces: EnumMap<Piece, BitBoard>,
}

impl PlayerPieces {
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        for piece in [Piece::Treto, Piece::Diago, Piece::Stello] {
            if self.pieces[piece].is_set(pos) {
                return Some(piece)
            }
        }
        None
    }
}

struct Board {
    pieces: EnumMap<PlayerColor, PlayerPieces>,
}

impl Board {
    fn get_piece(&self, pos: Position) -> Option<(Piece, PlayerColor)> {
        for player in [PlayerColor::South, PlayerColor::North] {
            if let Some(piece) = self.pieces[player].get_piece(pos) {
                return Some((piece, player))
            }
        }
        None
    }
}