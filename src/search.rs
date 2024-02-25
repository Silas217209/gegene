use hashbrown::HashMap;
use std::fmt::Display;
use std::ops::Neg;
use std::time::Instant;
use std::usize;
use std::time::Duration;
use crate::bmi::Bmi;
use crate::board::Board;
use crate::game::Outcome;
use crate::{game::Game, r#move::Move, uci::TimeControl};
use crate::r#move::{MoveType, Square};
use crate::role::Role;
use crate::score::Score;
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
    score: Score,
    depth: u32,
    node_type: NodeType,
    best_move: Move,
}

pub struct SearchResult {
    pub best_move: Move,
    pub best_score: Score,
    pub time: Duration,
}

pub fn search(game: Game, time_control: TimeControl) -> SearchResult {
    let start = Instant::now();
    let (mut moves, count) = game.get_legal_moves();
    sort_moves(&mut moves, count, game_phase(game.board), None);
    let mut evaluations: HashMap<u32, Score> = HashMap::new();
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

    let mut best_value = Score::CP(MIN);

    let mut alpha = Score::CP(MIN);
    let beta = Score::MateIn(1);

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
                depth,
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
                    depth,
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
                        .unwrap_or(&Score::CP(MIN))
                        .cmp(evaluations.get(&a.0).unwrap_or(&Score::CP(MIN)))
                });
                println!("info timeout");
                println!("info score cp {}", best_value);

                for i in 0..count {
                    println!("{}: {}", moves[i], evaluations.get(&moves[i].0).unwrap_or(&Score::CP(MIN)));
                }

                return SearchResult {
                    best_move: moves[0],
                    time: start.elapsed(),
                    best_score: best_value,
                };
            }
        }
        if count == 1 {
            break;
        }
        println!("info depth {}", depth);
        println!("info score {}", best_value);
        println!("info currmove {}", moves[0].to_algebraic());
        println!("info time {}", start.elapsed().as_millis() as u64);
        println!("info string ---------------------------------");

        // Sort moves based on evaluations
        moves.sort_by(|&a, &b| {
            (evaluations
                .get(&b.0)
                .unwrap_or(&Score::CP(MIN)))
                .cmp(&(evaluations.get(&a.0).unwrap_or(&Score::CP(MIN))))
        });

        if matches!(best_value, Score::MateIn(x) if x > 0) {
            break;
        }

        depth += 1;
    }
    moves.sort_by(|&a, &b| {
        evaluations
            .get(&b.0)
            .unwrap_or(&Score::CP(MIN))
            .cmp(&(evaluations.get(&a.0).unwrap_or(&Score::CP(MIN))))
    });

    println!("info score {}", best_value);

    for i in 0..count {
        println!("info string {}: {}", moves[i], evaluations.get(&moves[i].0).unwrap_or(&Score::CP(MIN)));
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
    max_depth: u32,
    alpha: Score,
    beta: Score,
    tt: &mut TranspositionTable,
    start: Instant,
    time: u64,
) -> Score {
    let mut alpha = alpha;
    let mut beta = beta;
    let mut best_move: Option<Move> = None;
    let mut best_value = Score::CP(MIN);

    if let Some(tt_entry) = tt.get(game.board.zobrist) {
        if tt_entry.depth >= depth {
            match tt_entry.node_type {
                NodeType::Cut => {
                    if beta <= tt_entry.score {
                        return tt_entry.score;
                    }
                    if alpha < tt_entry.score {
                        alpha = tt_entry.score;
                    }
                }
                NodeType::All => {
                    if tt_entry.score <= alpha {
                        return tt_entry.score;
                    }
                    if tt_entry.score < beta {
                        beta = tt_entry.score;
                    }
                }
                NodeType::PV => {
                    return tt_entry.score;
                }
            }
        }
    }

    if depth <= 0 || start.elapsed() > Duration::from_millis(time) {
        return Score::CP(evaluate(&game));
    }

    let (mut moves, count) = game.get_legal_moves();
    let tt_best_move = if let Some(tt_entry) = tt.get(game.board.zobrist) {
        Some(tt_entry.best_move)
    } else {
        None
    };
    sort_moves(&mut moves, count, game_phase(game.board), tt_best_move);

    if count == 0 {
        let check_mask = game.board.check_mask(game.is_white);
        if !check_mask.0.0 == 0 && !check_mask.1.0 == 0 {
            // Stalemate
            return Score::CP(-10);
        }
        // Checkmate
        return Score::MateIn(-(max_depth as i32 - depth as i32));
    }


    if let Outcome::Draw(_) = game.outcome {
        return Score::CP(-10);
    }

    // Skip this position if a mating sequence has already been found earlier in
    // the search, which would be shorter than any mate we could find from here.
    let mut alpha = alpha.max(Score::MateIn(-(max_depth as i32 - depth as i32)));
    let mut beta = beta.min(Score::MateIn(max_depth as i32 - depth as i32));
    if alpha >= beta {
        return alpha;
    }


    for i in 0..count {
        let mut new_game = game.clone();
        new_game.play(moves[i]);
        let mut value = -negamax(
            new_game,
            depth - 1,
            max_depth,
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
                max_depth,
                -beta,
                -value,
                tt,
                start,
                time,
            );
        }

        if value > best_value {
            best_value = value;
            best_move = Some(moves[i]);
        }

        if best_value >= beta {
            return best_value;
        }

        alpha = alpha.max(best_value);
    }

    if let Some(best_move) = best_move {
        tt.insert(game.board.zobrist, TTEntry {
            score: best_value,
            depth,
            node_type: NodeType::PV, // or NodeType::Cut or NodeType::All depending on the situation
            best_move,
        });
    }

    return best_value;
}

// from https://github.com/MitchelPaulin/Walleye
pub fn evaluate(game: &Game) -> i32 {
    let mut mg_white = 0;
    let mut eg_white = 0;
    let mut mg_black = 0;
    let mut eg_black = 0;
    let mut game_phase = 0;

    let mut loop_bitboard = game.board.by_color.white | game.board.by_color.black;
    while loop_bitboard.0 != 0 {
        let square_bitboard = loop_bitboard.0.blsi();
        let square = Square(square_bitboard.trailing_zeros() as u8);
        let piece = game.board.piece_at(square.0 as i32);
        if piece.is_none() {
            loop_bitboard.0 = loop_bitboard.0.blsr();
            continue;
        }

        let piece = piece.unwrap();


        game_phase += game_phase_val(piece.role);
        if piece.is_white {
            mg_white += mg_piece_val(piece.role);
            eg_white += eg_piece_val(piece.role);
        } else {
            mg_black += mg_piece_val(piece.role);
            eg_black += eg_piece_val(piece.role);
        }

        loop_bitboard.0 = loop_bitboard.0.blsr();
    }

    let mg_score = if game.is_white {
        mg_white - mg_black
    } else {
        mg_black - mg_white
    };
    let eg_score = if game.is_white {
        eg_white - eg_black
    } else {
        eg_black - eg_white
    };

    let mut mg_phase = game_phase;
    mg_phase = mg_phase.min(24);

    let eg_phase = 24 - mg_phase;

    (mg_score * mg_phase + eg_score * eg_phase) / 24
}

pub fn sort_moves(
    moves: &mut [Move; 218],
    count: usize,
    phase: i32,
    tt_best_move: Option<Move>,
) {
    let mut evaluated_moves: Vec<(Move, i32)> = moves[0..count]
        .iter()
        .map(|&m| (m, evaluate_move(m, phase)))
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

pub fn evaluate_move(m: Move, phase: i32) -> i32 {
    let mut mg_score = 0;
    let mut eg_score = 0;

    let file_from = m.from().0 % 8;
    let rank_from = m.from().0 / 8;

    let file_to = m.to().0 % 8;
    let rank_to = m.to().0 / 8;

    let file_from = if m.is_white() { 7 - file_from } else { file_from };
    let file_to = if m.is_white() { 7 - file_to } else { file_to };

    let rank_from = if m.is_white() { 7 - rank_from } else { rank_from };
    let rank_to = if m.is_white() { 7 - rank_to } else { rank_to };

    mg_score += match m.move_type() {
        MoveType::Promotion => match m.promotion_role() {
            crate::role::PromotionRole::Queen => mg_table(Role::Queen)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Rook => mg_table(Role::Rook)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Bishop => mg_table(Role::Bishop)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Knight => mg_table(Role::Knight)[rank_to as usize][file_to as usize],
        },
        MoveType::Quiet => 0,
        MoveType::DoublePawnPush => 5,
        MoveType::EnPassant => 5,
        MoveType::KingsideCastle => 10,
        MoveType::QueensideCastle => 10,
    };

    eg_score += match m.move_type() {
        MoveType::Promotion => match m.promotion_role() {
            crate::role::PromotionRole::Queen => eg_table(Role::Queen)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Rook => eg_table(Role::Rook)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Bishop => eg_table(Role::Bishop)[rank_to as usize][file_to as usize],
            crate::role::PromotionRole::Knight => eg_table(Role::Knight)[rank_to as usize][file_to as usize],
        },
        MoveType::Quiet => 0,
        MoveType::DoublePawnPush => 5,
        MoveType::EnPassant => 5,
        MoveType::KingsideCastle => 10,
        MoveType::QueensideCastle => 10,
    };

    if m.is_capture() {
        let mg_captured_value = mg_table(m.capture_role())[rank_to as usize][file_to as usize];
        let eg_captured_value = eg_table(m.capture_role())[rank_to as usize][file_to as usize];

        // winning capture
        if m.capture_role() as u8 >= m.role() as u8 {
            mg_score += 10;
            eg_score += 10;
        } else {
            mg_score -= 10;
            eg_score -= 10;
        }

        mg_score += mg_captured_value + 5;
        eg_score += eg_captured_value + 5;
    }

    mg_score += mg_table(m.role())[rank_to as usize][file_to as usize];
    eg_score += eg_table(m.role())[rank_to as usize][file_to as usize];

    mg_score -= mg_table(m.role())[rank_from as usize][file_from as usize];
    eg_score -= eg_table(m.role())[rank_from as usize][file_from as usize];

    ((mg_score * (256 - phase)) + (eg_score * phase)) / 256
}

fn mg_table(role: Role) -> &'static [[i32; 8]; 8] {
    match role {
        Role::Pawn => &MG_PAWN_TABLE,
        Role::Bishop => &MG_BISHOP_TABLE,
        Role::Knight => &MG_KNIGHT_TABLE,
        Role::Rook => &MG_ROOK_TABLE,
        Role::King => &MG_KING_TABLE,
        Role::Queen => &MG_QUEEN_TABLE,
    }
}

fn eg_table(role: Role) -> &'static [[i32; 8]; 8] {
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

fn game_phase(board: Board) -> i32 {
    let total_phase = game_phase_val(Role::Pawn) * 16 + game_phase_val(Role::Knight) * 4 + game_phase_val(Role::Bishop) * 4 + game_phase_val(Role::Rook) * 4 + game_phase_val(Role::Queen) * 2;
    let mut phase = total_phase;

    phase -= (board.by_role.pawns & board.by_color.white).0.count_ones() as i32 * game_phase_val(Role::Pawn);
    phase -= (board.by_role.knights & board.by_color.white).0.count_ones() as i32 * game_phase_val(Role::Knight);

    phase -= (board.by_role.rooks & board.by_color.black).0.count_ones() as i32 * game_phase_val(Role::Rook);
    phase -= (board.by_role.queens & board.by_color.black).0.count_ones() as i32 * game_phase_val(Role::Queen);


    (phase * 256 + (total_phase / 2)) / total_phase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_evaluation_equal() {
        let b = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("invalid fen");
        assert_eq!(evaluate(&b), 0);
    }
}