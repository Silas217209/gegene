use clap::Parser;

use crate::{bitboard::Bitboard, board::Color, game::Game};

mod bitboard;
mod board;
mod game;
mod lookup;
pub mod r#move;
mod pdep;
mod pext;
mod piece;
mod role;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"
    )]
    fen: String,

    #[arg(short, long, default_value_t = 1)]
    depth: u32,

    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {

    let args = Args::parse();

    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(args.fen.as_str()).expect("invalid FEN");

    if args.debug {
        println!("board: \n{}", game.board);
        println!("pin mask vh: \n{}", game.board.pin_mask(game.turn).0);
        println!("pin mask diagonal: \n{}", game.board.pin_mask(game.turn).1);
        println!("check capture mask: \n{}", game.board.check_mask(game.turn).1);
        println!("check move mask: \n{}", game.board.check_mask(game.turn).0);
    }

    let start = std::time::Instant::now();
    let nodes = game.perft(args.depth, args.depth);
    let duration = start.elapsed();
    println!("Perft({}) = nodes: {nodes}", args.depth);
}
