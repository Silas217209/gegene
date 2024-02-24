use std::io::{stdin, stdout, BufRead, Write};
use std::thread;

use clap::Parser;
use lookup::generate_zobrist_numbers;
use uci::RecceiveUCI;

use crate::{game::Game, uci::SendUCI};
use crate::uci::Score::CP;

mod bitboard;
mod bmi;
mod board;
mod game;
mod lookup;
pub mod r#move;
mod piece;
mod role;
mod search;
mod uci;
mod values;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buffer = String::new();

    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("invalid fen");

    loop {
        buffer.clear();

        stdin.lock().read_line(&mut buffer).unwrap();

        let message = RecceiveUCI::parse_str(&buffer);

        match message {
            RecceiveUCI::UCI => {
                writeln!(stdout, "id name gégène").unwrap();
                writeln!(stdout, "id author Silas Pachali").unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            RecceiveUCI::Debug(_) => {}
            RecceiveUCI::IsReady => {
                writeln!(stdout, "{}", SendUCI::ReadyOk.to_str()).unwrap();
            }
            RecceiveUCI::SetOption { id: _, value: _ } => {}
            RecceiveUCI::UCINewGame => {}
            RecceiveUCI::Position { position, moves } => {
                match position {
                    uci::Position::Startpos => {
                        game = Game::from_fen(
                            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                        )
                            .expect("invalid fen");
                    }
                    uci::Position::Fen { fen } => {
                        game = Game::from_fen(&fen).expect("invalid fen");
                    }
                }
                for move1 in &moves {
                    game.play_uci(move1).expect("error playing moves");
                }
            }
            RecceiveUCI::Go {
                ponder,
                time_control,
                depth,
            } => {
                let result = search::search(game, time_control);

                writeln!(
                    stdout,
                    "{}",
                    SendUCI::BestMove {
                        move1: result.best_move,
                        ponder: None,
                    }
                        .to_str()
                )
                    .unwrap();
                writeln!(
                    stdout,
                    "{}",
                    SendUCI::Info(uci::Info::Time(result.time)).to_str()
                )
                    .unwrap();
                writeln!(
                    stdout,
                    "{}",
                    SendUCI::Info(uci::Info::Score(CP(result.best_score as i64))).to_str()
                )
                    .unwrap();
            }
            RecceiveUCI::Stop => {}
            RecceiveUCI::PonderHit => {}
            RecceiveUCI::Quit => return,
            RecceiveUCI::Unknown(_) => {
                let _ = stdout.write_all("Unknown command\n".as_bytes());
            }
        }
        stdout.flush().expect("Failed to flush stdout");
    }
}
