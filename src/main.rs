use crate::position::Position;

mod position;
mod players;
mod board;
mod bitboard;

fn main() {
    for rank in position::RANKS.rev() {
        for file in position::FILES {
            let pos = Position::new(rank, file);
            if pos.is_valid() {
                print!("#");
            } else {
                print!(".");
            }
            print!(" ");
        }
        println!();
    }
}
