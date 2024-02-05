use crate::game::Game;

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
    const DEPTH: u32 = 3;

    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");

    println!("{}", game.board);
    println!("Color: {:?}", game.turn);

    let start = std::time::Instant::now();
    let nodes = game.perft(DEPTH, DEPTH);
    let duration = start.elapsed();
    println!("Perft({DEPTH}) = nodes: {nodes}");
    println!(
        "Nodes/s: {}",
        nodes as u128 / (duration.as_millis() + 1) * 1000
    );
}
