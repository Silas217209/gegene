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
use crate::r#move::{MoveType, Square};
use crate::role::Role;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20_000;

const MG_PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const EG_PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const MG_KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const EG_KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const MG_BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const EG_BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const MG_ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const EG_ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const MG_QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const EG_QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const MG_KING_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

const EG_KING_TABLE: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

const MAX_DEPTH: u32 = 40;
const MIN: i32 = -100_000;
const MAX: i32 = 100_000;
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

pub fn evaluate(game: &Game) -> i32 {
    let mut me = 0;
    let mut opponent = 0;

    let my_bitboard = game.board.my_bitboard(game.is_white);
    let enemy_bitboard = game.board.enemy_bitboard(game.is_white);

    let mut loop_bitboard = my_bitboard | enemy_bitboard;
    while loop_bitboard.0 != 0 {
        let square = loop_bitboard.0.trailing_zeros();
        let piece = game.board.piece_at(square as i32).unwrap();

        let (mg_table, eg_table) = match piece.role {
            Role::Pawn => (MG_PAWN_TABLE, EG_PAWN_TABLE),
            Role::Bishop => (MG_BISHOP_TABLE, EG_BISHOP_TABLE),
            Role::Knight => (MG_KNIGHT_TABLE, EG_KNIGHT_TABLE),
            Role::Rook => (MG_ROOK_TABLE, EG_ROOK_TABLE),
            Role::Queen => (MG_QUEEN_TABLE, EG_QUEEN_TABLE),
            Role::King => (MG_KING_TABLE, EG_KING_TABLE),
        };

        let table = if is_endgame(game) { eg_table } else { mg_table };
        let score = if piece.is_white {
            table[63 - square as usize]
        } else {
            table[square as usize]
        };

        if piece.is_white == game.is_white {
            me += score;
        } else {
            opponent += score;
        }

        loop_bitboard.0 &= loop_bitboard.0.blsr();
    }

    let my_pawns = (game.board.by_role.pawns & my_bitboard).0.count_ones() as i32;
    let my_knights = (game.board.by_role.knights & my_bitboard).0.count_ones() as i32;
    let my_bishops = (game.board.by_role.bishops & my_bitboard).0.count_ones() as i32;
    let my_rooks = (game.board.by_role.rooks & my_bitboard).0.count_ones() as i32;
    let my_queens = (game.board.by_role.queens & my_bitboard).0.count_ones() as i32;

    let enemy_pawns = (game.board.by_role.pawns & enemy_bitboard).0.count_ones() as i32;
    let enemy_knights = (game.board.by_role.knights & enemy_bitboard).0.count_ones() as i32;
    let enemy_bishops = (game.board.by_role.bishops & enemy_bitboard).0.count_ones() as i32;
    let enemy_rooks = (game.board.by_role.rooks & enemy_bitboard).0.count_ones() as i32;
    let enemy_queens = (game.board.by_role.queens & enemy_bitboard).0.count_ones() as i32;

    me += my_pawns * PAWN_VALUE;
    me += my_knights * KNIGHT_VALUE;
    me += my_bishops * BISHOP_VALUE;
    me += my_rooks * ROOK_VALUE;
    me += my_queens * QUEEN_VALUE;

    opponent += enemy_pawns * PAWN_VALUE;
    opponent += enemy_knights * KNIGHT_VALUE;
    opponent += enemy_bishops * BISHOP_VALUE;
    opponent += enemy_rooks * ROOK_VALUE;
    opponent += enemy_queens * QUEEN_VALUE;

    // bishop pair
    if my_bishops > 1 {
        me += 50;
    } else {
        me -= 50;
    }
    if enemy_bishops > 1 {
        opponent += 50;
    } else {
        opponent -= 50;
    }

    let pawn_count = my_pawns + enemy_pawns;
    // sliding pieces better with less pawns
    if pawn_count < 8 {
        me += my_bishops * 10;
        me += my_rooks * 10;
        me += my_queens * 10;
        opponent += enemy_bishops * 10;
        opponent += enemy_rooks * 10;
        opponent += enemy_queens * 10;
    }
    // knights better with more pawns
    if pawn_count > 8 {
        me += my_knights * 10;
        opponent += enemy_knights * 10;
    }

    return me - opponent;
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
