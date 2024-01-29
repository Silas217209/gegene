use crate::{board::Color, game::Game};
use num_format::{Locale, ToFormattedString};

mod bitboard;
mod board;
mod game;
mod lookup;
pub mod r#move;
mod pdep;
mod pext;
mod piece;
mod role;

fn main() {
    let fen = "1k6/8/8/8/3Pp3/8/8/1K6 b - d3 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let start = std::time::Instant::now();
    let (nodes, en_pasaant) = game.perft(4);
    let duration = start.elapsed();

    println!("Perft(2) = {}", nodes);
    println!("Perft(2) = {}", en_pasaant);
    println!("Nodes/s: {}", (nodes as u128 / duration.as_millis() * 1000).to_formatted_string(&Locale::fr));
}
