use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use std::time::Instant;
use std::usize;
use std::{i16, time::Duration};

use crate::bmi::Bmi;
use crate::board::Board;
use crate::game::Outcome;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::{bitboard::Bitboard, game::Game, r#move::Move, uci::TimeControl};
use crate::piece::Piece;
use crate::r#move::{MoveType, Square};
use crate::role::Role;
use crate::values::*;

pub struct TranspositionTable {
    table: HashMap<u64, TTEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: u64) -> Option<TTEntry> {
        self.table.get(&key).copied()
    }

    pub fn insert(&mut self, key: u64, value: TTEntry) {
        self.table.insert(key, value);
    }
}

#[derive(Copy, Clone)]
enum NodeType {
    Cut,
    All,
    PV,
}

#[derive(Copy, Clone)]
pub struct TTEntry {
    score: i32,
    depth: u32,
    node_type: NodeType,
    best_move: Move,
}

pub struct SearchResult {
    pub best_move: Move,
    pub best_score: i32,
    pub time: Duration,
}

pub fn search(game: Game, time_control: TimeControl) -> SearchResult {
    let start = Instant::now();
    let (mut moves, count) = game.get_legal_moves();
    sort_moves(&mut moves, count, is_endgame(&game), None);
    let mut evaluations: HashMap<u32, i32> = HashMap::new();
    let mut tt = TranspositionTable::new();

    let time = match time_control {
        TimeControl::Infinite => 30_000,
        TimeControl::Movetime(time) => time * 96 / 100,
        TimeControl::RemainingTime {
            white,
            black,
            winc,
            binc,
            movestogo,
        } => {
            if game.is_white {
                winc + (white / (movestogo + 1))
            } else {
                binc + (black / (movestogo + 1))
            }
        }
    };

    let mut tt = TranspositionTable::new();

    let mut best_value = MIN;

    let mut alpha = MIN;
    let mut beta = MAX;

    let mut depth = 1;
    while start.elapsed() < Duration::from_millis(time) && depth <= MAX_DEPTH {
        for i in 0..count {
            if moves[i].0 == 0 {
                continue;
            }
            let mut new_game = game;
            new_game.play(moves[i]);
            let mut value = -negamax(
                new_game,
                depth - 1,
                -beta,
                -alpha,
                &mut tt,
                start,
                time,
            );
            evaluations.insert(moves[i].0, value);

            if value < best_value {
                continue;
            }
            if value < beta && depth > 2 {
                value = -negamax(
                    new_game,
                    depth - 1,
                    -beta,
                    -value,
                    &mut tt,
                    start,
                    time,
                );
                evaluations.insert(moves[i].0, value);
            }

            if value > best_value {
                best_value = value;
            }

            if value >= beta {
                break;
            }

            alpha = alpha.max(value);

            // Check if the elapsed time is more than 20 seconds
            if start.elapsed() > Duration::from_millis(time) {
                moves.sort_by(|&a, &b| {
                    evaluations
                        .get(&b.0)
                        .unwrap_or(&i32::MIN)
                        .cmp(evaluations.get(&a.0).unwrap_or(&i32::MIN))
                });
                println!("info timeout");

                println!("info score cp {}", best_value);

                for i in 0..count {
                    println!("{}: {}", moves[i], evaluations.get(&moves[i].0).unwrap_or(&-42));
                }

                return SearchResult {
                    best_move: moves[0],
                    time: start.elapsed(),
                    best_score: best_value,
                };
            }
        }

        println!("info depth {}", depth);
        println!("info currmove {}", moves[0].to_algebraic());
        println!("info score cp {}", best_value);

        // Sort moves based on evaluations
        moves.sort_by(|&a, &b| {
            evaluations
                .get(&b.0)
                .unwrap_or(&i32::MIN)
                .cmp(evaluations.get(&a.0).unwrap_or(&i32::MIN))
        });
        depth += 1;
    }

    println!("info score cp {}", best_value);

    for i in 0..count {
        println!("{}: {}", moves[i], evaluations.get(&moves[i].0).unwrap_or(&-42));
    }

    return SearchResult {
        best_move: moves[0],
        time: start.elapsed(),
        best_score: best_value,
    };
}

pub fn negamax(
    game: Game,
    depth: u32,
    alpha: i32,
    beta: i32,
    tt: &mut TranspositionTable,
    start: Instant,
    time: u64,
) -> i32 {
    if depth <= 0 || start.elapsed() > Duration::from_millis(time) {
        return evaluate(&game);
    }

    let (mut moves, count) = game.get_legal_moves();
    let tt_best_move = if let Some(tt_entry) = tt.get(game.board.zobrist) {
        Some(tt_entry.best_move)
    } else {
        None
    };
    sort_moves(&mut moves, count, is_endgame(&game), tt_best_move);
    if count == 0 {
        if game.board.seen_by_enemy(game.is_white)
            & game.board.by_role.kings
            & game.board.my_bitboard(game.is_white)
            == Bitboard(0)
        {
            return -10;
        }
        return MIN;
    }

    if let Outcome::Draw(_) = game.outcome {
        return -10;
    }

    let mut best_value = MIN;
    let mut alpha = alpha;


    for i in 0..count {
        let mut new_game = game;
        new_game.play(moves[i]);
        let mut value = -negamax(
            new_game,
            depth - 1,
            -beta,
            -alpha,
            tt,
            start,
            time,
        );

        if value < best_value {
            continue;
        }
        if value < beta {
            // Re-search with full depth
            value = -negamax(
                new_game,
                depth - 1,
                -beta,
                -value,
                tt,
                start,
                time,
            );
        }

        best_value = best_value.max(value);

        if best_value >= beta {
            return best_value;
        }

        alpha = alpha.max(best_value);
    }

    return best_value;
}
// from https://github.com/MitchelPaulin/Walleye
pub fn evaluate(game: &Game) -> i32 {
    let mut white_mg = 0;
    let mut white_eg = 0;
    let mut black_mg = 0;
    let mut black_eg = 0;
    let mut game_phase = 0;

    let mut loop_bitbaord = game.board.my_bitboard(game.is_white) | game.board.enemy_bitboard(game.is_white);
    while loop_bitbaord.0 != 0 {
        let square_bitboard = Bitboard(loop_bitbaord.0.blsi());
        let square_number = square_bitboard.0.trailing_zeros() as i32;
        let piece = game.board.piece_at(square_number);

        if piece.is_none() {
            loop_bitbaord.0 = loop_bitbaord.0.blsr();
            continue;
        }
        let piece = piece.unwrap();

        game_phase += game_phase_val(piece.role);
        if piece.is_white {
            white_mg += mg_table(piece.role)[63 - square_number as usize];
            white_eg += eg_table(piece.role)[63 - square_number as usize];
        } else {
            black_mg += mg_table(piece.role)[square_number as usize];
            black_eg += eg_table(piece.role)[square_number as usize];
        }

        loop_bitbaord.0 = loop_bitbaord.0.blsr();
    }


    let (mg_score, eg_score) = if game.is_white {
        (white_mg - black_mg, white_eg - black_eg)
    } else {
        (black_mg - white_mg, black_eg - white_eg)
    };

    let mut mg_phase = game_phase;

    /* in case of early promotion */
    if mg_phase > 24 {
        mg_phase = 24;
    }
    let eg_phase = 24 - mg_phase;
    (mg_score * mg_phase + eg_score * eg_phase) / 24
}

fn is_endgame(game: &Game) -> bool {
    let mut material = 0;
    // if the piece values are less than 30 Pawns, it's endgame
    material += (game.board.by_role.pawns).0.count_ones() as i32 * PAWN_VALUE;
    material += (game.board.by_role.knights).0.count_ones() as i32 * KNIGHT_VALUE;
    material += (game.board.by_role.bishops).0.count_ones() as i32 * BISHOP_VALUE;
    material += (game.board.by_role.rooks).0.count_ones() as i32 * ROOK_VALUE;
    material += (game.board.by_role.queens).0.count_ones() as i32 * QUEEN_VALUE;
    material += (game.board.by_role.kings).0.count_ones() as i32 * KING_VALUE;

    return material < 30 * PAWN_VALUE;
}

pub fn sort_moves(
    moves: &mut [Move; 218],
    count: usize,
    is_endgame: bool,
    tt_best_move: Option<Move>,
) {
    let mut evaluated_moves: Vec<(Move, i32)> = moves[0..count]
        .iter()
        .map(|&m| (m, evaluate_move(m, is_endgame)))
        .collect();

    evaluated_moves.sort_by(|a, b| b.1.cmp(&a.1));

    for i in 0..count {
        if tt_best_move.is_some() {
            moves[i + 1] = evaluated_moves[i].0;
        } else {
            moves[i] = evaluated_moves[i].0;
        }
    }

    if tt_best_move.is_some() {
        moves[0] = tt_best_move.unwrap();
    }
}

pub fn evaluate_move(m: Move, is_endgame: bool) -> i32 {
    let mut score = 0;
    score += match m.move_type() {
        MoveType::Promotion => match m.promotion_role() {
            crate::role::PromotionRole::Queen => QUEEN_VALUE,
            crate::role::PromotionRole::Rook => ROOK_VALUE,
            crate::role::PromotionRole::Bishop => BISHOP_VALUE,
            crate::role::PromotionRole::Knight => KNIGHT_VALUE,
        },
        MoveType::Quiet => 0,
        MoveType::DoublePawnPush => 20,
        MoveType::EnPassant => 5,
        MoveType::KingsideCastle => 10,
        MoveType::QueensideCastle => 10,
    };

    if m.is_capture() {
        let captured_value = match m.capture_role() {
            Role::Pawn => PAWN_VALUE,
            Role::Knight => KNIGHT_VALUE,
            Role::Bishop => BISHOP_VALUE,
            Role::Rook => ROOK_VALUE,
            Role::Queen => QUEEN_VALUE,
            Role::King => KING_VALUE,
        };

        // winning capture
        if m.capture_role() as u8 >= m.role() as u8 {
            score += 10;
        } else {
            score -= 10;
        }

        score += captured_value + 5;
    }
    let (mg_table, eg_table) = match m.role() {
        Role::Pawn => (MG_PAWN_TABLE, EG_PAWN_TABLE),
        Role::Bishop => (MG_BISHOP_TABLE, EG_BISHOP_TABLE),
        Role::Knight => (MG_KNIGHT_TABLE, EG_KNIGHT_TABLE),
        Role::Rook => (MG_ROOK_TABLE, EG_ROOK_TABLE),
        Role::Queen => (MG_QUEEN_TABLE, EG_QUEEN_TABLE),
        Role::King => (MG_KING_TABLE, EG_KING_TABLE),
    };

    let table = if is_endgame { eg_table } else { mg_table };

    let to_score = if m.is_white() {
        table[63 - m.to().0 as usize]
    } else {
        table[m.to().0 as usize]
    };

    let from_score = if m.is_white() {
        table[63 - m.from().0 as usize]
    } else {
        table[m.from().0 as usize]
    };

    score += to_score - from_score;

    score
}

fn mg_table(role: Role) -> &'static [i32; 64] {
    match role {
        Role::Pawn => &MG_PAWN_TABLE,
        Role::Bishop => &MG_BISHOP_TABLE,
        Role::Knight => &MG_KNIGHT_TABLE,
        Role::Rook => &MG_ROOK_TABLE,
        Role::King => &MG_KING_TABLE,
        Role::Queen => &MG_QUEEN_TABLE,
    }
}

fn eg_table(role: Role) -> &'static [i32; 64] {
    match role {
        Role::Pawn => &EG_PAWN_TABLE,
        Role::Bishop => &EG_BISHOP_TABLE,
        Role::Knight => &EG_KNIGHT_TABLE,
        Role::Rook => &EG_ROOK_TABLE,
        Role::King => &EG_KING_TABLE,
        Role::Queen => &EG_QUEEN_TABLE,
    }
}

fn mg_piece_val(role: Role) -> i32 {
    match role {
        Role::Pawn => 82,
        Role::Knight => 337,
        Role::Bishop => 365,
        Role::Rook => 477,
        Role::Queen => 1025,
        Role::King => 0,
    }
}

fn eg_piece_val(role: Role) -> i32 {
    match role {
        Role::Pawn => 94,
        Role::Knight => 281,
        Role::Bishop => 297,
        Role::Rook => 512,
        Role::Queen => 936,
        Role::King => 0,
    }
}

fn game_phase_val(role: Role) -> i32 {
    match role {
        Role::Pawn => 0,
        Role::Knight => 1,
        Role::Bishop => 1,
        Role::Rook => 2,
        Role::Queen => 4,
        Role::King => 0,
    }
}