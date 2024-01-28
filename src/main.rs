use crate::{board::Color, game::Game};

mod bitboard;
mod board;
mod game;
mod lookup;
mod r#move;
mod pdep;
mod pext;
mod piece;
mod role;

fn main() {
    let fen = "8/8/8/2k5/3Pp3/8/8/4K3 b - d4 0 1";
    let game = Game::from_fen(fen);

    println!("board: \n{}", game.board);
    println!("pin mask: \n{}", game.board.pin_mask(Color::Black));
    println!(
        "check move mask: \n{}",
        game.board.check_mask(Color::Black).0
    );
    println!(
        "check capture mask: \n{}",
        game.board.check_mask(Color::Black).1
    );
}
