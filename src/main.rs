use crate::board::Board;

mod bitboard;
mod board;
mod role;
mod piece;
mod r#move;
mod game;

fn main() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let bitboard = format!("{}", board.by_role.kings & board.by_color.white);
    let board = format!("{}", board);

    println!("{}", board);
    println!("{}", bitboard);
}
