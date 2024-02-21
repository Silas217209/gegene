#![allow(dead_code)]
#![warn(unused_must_use)]

pub(crate) mod bishop_mask;
pub(crate) mod bishop_moves;
pub(crate) mod direction_mask;
pub(crate) mod extended_bishop_mask;
pub(crate) mod king;
pub mod knight;
pub(crate) mod pin_mask;
pub(crate) mod rook_mask;
pub(crate) mod rook_moves;
pub(crate) mod zobrist;

use crate::bitboard::Bitboard;
use crate::bmi::Bmi;
use crate::board::Board;
use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::direction_mask::DIRECTION_MASK;
use crate::lookup::rook_mask::ROOK_MASK;
use std::fs::File as Fil;
use std::io::Write;

pub const fn generate_king_moves() -> [Bitboard; 64] {
    let mut moves: [Bitboard; 64] = [Bitboard(0); 64];

    let mut square = 0;
    while square < 64 {
        let mut bitboard = 0;

        let north = square < 56;
        let south = square > 7;
        let east = square % 8 < 7;
        let west = square % 8 > 0;

        if north {
            bitboard |= 1 << square + 8;
        }

        if south {
            bitboard |= 1 << square - 8;
        }

        if east {
            bitboard |= 1 << square + 1;
        }

        if west {
            bitboard |= 1 << square - 1;
        }

        if north && east {
            bitboard |= 1 << square + 9;
        }

        if north && west {
            bitboard |= 1 << square + 7;
        }

        if south && east {
            bitboard |= 1 << square - 7;
        }

        if south && west {
            bitboard |= 1 << square - 9;
        }

        moves[square] = Bitboard(bitboard);
        square += 1;
    }

    return moves;
}

pub const fn generate_knight_moves() -> [Bitboard; 64] {
    let mut moves: [Bitboard; 64] = [Bitboard(0); 64];

    let mut square = 0;
    while square < 64 {
        let mut bitboard = 0;

        let up_2 = square < 48;
        let down_2 = square > 15;

        let up_1 = square < 56;
        let down_1 = square > 7;

        let left_2 = square % 8 > 1;
        let right_2 = square % 8 < 6;

        let left_1 = square % 8 > 0;
        let right_1 = square % 8 < 7;

        if up_2 && right_1 {
            bitboard |= 1 << square + 17;
        }

        if up_2 && left_1 {
            bitboard |= 1 << square + 15;
        }

        if up_1 && right_2 {
            bitboard |= 1 << square + 10;
        }

        if up_1 && left_2 {
            bitboard |= 1 << square + 6;
        }

        if down_1 && right_2 {
            bitboard |= 1 << square - 6;
        }

        if down_1 && left_2 {
            bitboard |= 1 << square - 10;
        }

        if down_2 && right_1 {
            bitboard |= 1 << square - 15;
        }

        if down_2 && left_1 {
            bitboard |= 1 << square - 17;
        }

        moves[square] = Bitboard(bitboard);
        square += 1;
    }

    return moves;
}

pub const fn generate_rook_mask() -> [(Bitboard, u64); 64] {
    // gererate bitboards for all possible rook moves, excluding the borders (rank 1, 8, file A, H)
    let mut moves: [(Bitboard, u64); 64] = [(Bitboard(0), 0); 64];
    let mut offset = 0;

    let mut square = 0;
    while square < 64 {
        let rank_number = square / 8;
        let file_number = square % 8;

        let rank = Bitboard::from_rank_number(rank_number).0;
        let file = Bitboard::from_file_number(file_number).0;

        let excluded = (rank & (Bitboard::from_file_number(0).0 | Bitboard::from_file_number(7).0))
            | (file & (Bitboard::from_rank_number(0).0 | Bitboard::from_rank_number(7).0))
            | (1 << square);

        let mask = (rank | file) ^ excluded;
        let combinations = 2u64.pow(mask.count_ones());
        moves[square as usize] = (Bitboard(mask), offset);
        offset += combinations;
        square += 1;
    }
    return moves;
}

pub const fn generate_bishop_mask() -> [(Bitboard, u64); 64] {
    // gererate bitboards for all possible bishop moves (diagonal), excluding the borders (rank 1, 8, file A, H)
    let mut moves: [(Bitboard, u64); 64] = [(Bitboard(0), 0); 64];
    let mut offset = 0;

    let mut square = 0;
    while square < 64 {
        square += 1;
        let rank_number = square / 8;
        let file_number = square % 8;

        let mut diagonals: u64 = 0;
        // loop sout east
        let mut i = 1;
        while i < 8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number + i > 6 || file_number + i > 6 {
                break;
            }
            diagonals |= 1 << (square + i * 9);
            i += 1;
        }

        // loop sout west
        let mut i = 1;
        while i < 8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number + i > 6 || file_number - i < 1 {
                break;
            }
            diagonals |= 1 << (square + i * 7);
            i += 1;
        }

        // loop north east
        let mut i = 1;
        while i < 8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number - i < 1 || file_number + i > 6 {
                break;
            }
            diagonals |= 1 << (square - i * 7);
            i += 1;
        }

        // loop north west
        let mut i = 1;
        while i < 8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number - i < 1 || file_number - i < 1 {
                break;
            }
            diagonals |= 1 << (square - i * 9);
            i += 1;
        }

        moves[square as usize] = (Bitboard(diagonals), offset);

        let combinations = 2u64.pow(diagonals.count_ones());
        offset += combinations;
    }
    return moves;
}

pub fn generate_rook_moves() -> std::io::Result<()> {
    let mut moves: [Bitboard; 102400] = [Bitboard(0); 102400];
    for (square, (mask, offset)) in ROOK_MASK.iter().enumerate() {
        let combinations: u64 = 2u64.pow(mask.0.count_ones());

        for i in 0..(combinations) {
            let mut current_move = Bitboard(0);
            let blockers = Bitboard(i.pdep(mask.0));

            // loop north
            for i in 1..8 {
                let new_square = square + i * 8;

                // check border
                if new_square > 63 {
                    break;
                }

                let new_square_bitboard = Bitboard(1 << new_square);

                current_move |= new_square_bitboard;

                if new_square_bitboard & blockers != Bitboard(0) {
                    break;
                }
            }

            // loop south
            for i in 1..8 {
                let new_square: i32 = square as i32 - i * 8;

                // check border
                if new_square < 0 {
                    break;
                }

                let new_square_bitboard = Bitboard(1 << new_square);

                current_move |= new_square_bitboard;

                if new_square_bitboard & blockers != Bitboard(0) {
                    break;
                }
            }

            // loop east
            for i in 1..8 {
                let new_square: i32 = square as i32 + i;

                // check border
                if new_square % 8 == 0 {
                    break;
                }

                let new_square_bitboard = Bitboard(1 << new_square);

                current_move |= new_square_bitboard;

                if new_square_bitboard & blockers != Bitboard(0) {
                    break;
                }
            }

            // loop west
            for i in 1..8 {
                let new_square: i32 = square as i32 - i;

                // check border
                if new_square % 8 == 7 || new_square < 0 {
                    break;
                }

                let new_square_bitboard = Bitboard(1 << new_square);

                current_move |= new_square_bitboard;

                if new_square_bitboard & blockers != Bitboard(0) {
                    break;
                }
            }

            moves[(offset + i) as usize] = current_move;
        }
    }

    let mut output = Fil::create("lookup/rook_moves.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(
        output,
        "pub const ROOK_MOVES: [Bitboard; {}] = [",
        moves.len()
    );
    for mv in moves {
        writeln!(output, "\tBitboard(0x{:x}),", mv.0);
    }
    writeln!(output, "];")
}

pub fn generate_bishop_moves() -> std::io::Result<()> {
    let mut moves: Vec<Bitboard> = Vec::new();
    for (square, (mask, offset)) in BISHOP_MASK.iter().enumerate() {
        let square = square as i32;

        let possibilities: u64 = 2u64.pow(mask.0.count_ones());
        for i in 0..possibilities {
            let mut bitboard = Bitboard(0);
            let blocker = Bitboard(i.pdep(mask.0));

            // loop south east
            for j in 1..8 {
                // check for border
                if square + j * 9 > 63 || square % 8 + j > 7 {
                    break;
                }

                let current_move = Bitboard(1 << (square + j * 9));
                bitboard |= current_move;

                // check if blocker is in the way
                if blocker & current_move != Bitboard(0) {
                    break;
                }
            }

            // loop south west
            for j in 1..8 {
                // check for border
                if square + j * 7 > 63 || square % 8 - j < 0 {
                    break;
                }

                let current_move = Bitboard(1 << (square + j * 7));
                bitboard |= current_move;

                // check if blocker is in the way
                if blocker & current_move != Bitboard(0) {
                    break;
                }
            }

            // loop north east
            for j in 1..8 {
                // check for border
                if square - j * 7 < 0 || square % 8 + j > 7 {
                    break;
                }

                let current_move = Bitboard(1 << (square - j * 7));
                bitboard |= current_move;

                // check if blocker is in the way
                if blocker & current_move != Bitboard(0) {
                    break;
                }
            }

            // loop north west
            for j in 1..8 {
                // check for border
                if square - j * 9 < 0 || square % 8 - j < 0 {
                    break;
                }

                let current_move = Bitboard(1 << (square - j * 9));
                bitboard |= current_move;

                // check if blocker is in the way
                if blocker & current_move != Bitboard(0) {
                    break;
                }
            }

            moves.push(bitboard);
        }
    }
    let mut output = Fil::create("lookup/bishop_moves.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(
        output,
        "pub const BISHOP_MOVES: [Bitboard; {}] = [",
        moves.len()
    );
    for mv in moves {
        writeln!(output, "\tBitboard(0x{:x}),", mv.0);
    }
    writeln!(output, "];")
}

pub const fn generate_direction_mask() -> [(Bitboard, Bitboard, Bitboard, Bitboard); 64] {
    let mut moves: [(Bitboard, Bitboard, Bitboard, Bitboard); 64] =
        [(Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)); 64];

    let mut square = 0;
    while square < 64 {
        let file = square % 8;
        let rank = square / 8;

        let mut north_mask: u64 = 0;
        let mut i = rank + 1;
        while i < 8 {
            north_mask |= Bitboard::from_rank_number(i).0;
            i += 1;
        }

        let mut east_mask: u64 = 0;
        let mut i = file + 1;
        while i < 8 {
            east_mask |= Bitboard::from_file_number(i).0;
            i += 1;
        }

        let mut south_mask = 0;
        let mut i = 0;
        while i < rank {
            south_mask |= Bitboard::from_rank_number(i).0;
            i += 1;
        }

        let mut west_mask = 0;
        let mut i = 0;
        while i < file {
            west_mask |= Bitboard::from_file_number(i).0;
            i += 1;
        }

        moves[square] = (
            Bitboard(north_mask),
            Bitboard(east_mask),
            Bitboard(south_mask),
            Bitboard(west_mask),
        );
        square += 1;
    }
    return moves;
}

pub fn generate_pin_mask() -> std::io::Result<()> {
    // [KingSquare * 64 + EnemySquare] = Path between King and Enemy including the Enemy. Zero if not a slider
    let mut moves: [Bitboard; 64 * 64] = [Bitboard(0); 64 * 64];

    let king_square: usize = 0;
    while king_square < 64 {
        for enemy_square in 0..64 {
            let index = king_square * 64 + enemy_square;

            let king_file = king_square % 8;
            let king_rank = king_square / 8;

            let enemy_bitboard = Bitboard(1 << enemy_square);

            let blockers = enemy_bitboard;

            let mut rook_moves = Board::rook_attacks(king_square as usize, blockers);
            let king_reaches_north = Bitboard(
                Bitboard::from_file_number(king_file as usize)
                    .0
                    .wrapping_shl((king_rank as u32 + 1) * 8),
            ) & rook_moves;

            let king_reaches_south = Bitboard(
                Bitboard::from_file_number(king_file as usize)
                    .0
                    .wrapping_shr((7 - king_rank as u32) * 8),
            ) & rook_moves;

            let horizontal_mask = Bitboard::from_rank_number(king_rank as usize);

            let king_reaches_east = Bitboard(
                Bitboard::from_rank_number(king_rank as usize)
                    .0
                    .wrapping_shl(king_file as u32),
            ) & rook_moves;
            let king_reaches_east = king_reaches_east & horizontal_mask;

            let king_reaches_west = Bitboard(
                Bitboard::from_rank_number(king_rank as usize)
                    .0
                    .wrapping_shr(8 - king_file as u32),
            ) & rook_moves;
            let king_reaches_west = king_reaches_west & horizontal_mask;

            for direction in [
                king_reaches_north,
                king_reaches_south,
                king_reaches_east,
                king_reaches_west,
            ] {
                let attack = direction & enemy_bitboard;

                if attack != Bitboard(0) {
                    let attack_square = attack.0.trailing_zeros();
                    let attacker_moves =
                        Board::rook_attacks(attack_square as usize, blockers) & direction;

                    rook_moves = direction & (attacker_moves | Bitboard(1 << attack_square));
                }
            }
            let mut bishop_moves = Board::bishop_attacks(king_square as usize, blockers);
            let (north_mask, east_mask, south_mask, west_mask) =
                DIRECTION_MASK[king_square as usize];

            let king_reaches_north_east = bishop_moves & (north_mask & east_mask);
            let king_reaches_north_west = bishop_moves & (north_mask & west_mask);
            let king_reaches_south_east = bishop_moves & (south_mask & east_mask);
            let king_reaches_south_west = bishop_moves & (south_mask & west_mask);

            for direction in [
                king_reaches_north_east,
                king_reaches_north_west,
                king_reaches_south_east,
                king_reaches_south_west,
            ] {
                let attack = direction & enemy_bitboard;
                if attack != Bitboard(0) {
                    let attack_square = attack.0.trailing_zeros();
                    let attacker_moves =
                        Board::bishop_attacks(attack_square as usize, blockers) & direction;

                    bishop_moves = direction & (attacker_moves | Bitboard(1 << attack_square));
                }
            }

            let bitboard = if rook_moves & enemy_bitboard != Bitboard(0) {
                rook_moves
            } else if bishop_moves & enemy_bitboard != Bitboard(0) {
                bishop_moves
            } else {
                Bitboard(0)
            };

            moves[index] = bitboard;
        }
    }

    let mut output = Fil::create("src/lookup/pin_mask.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(
        output,
        "// [KingSquare * 64 + EnemySquare] = Path between King and Enemy including the Enemye"
    );
    writeln!(output, "pub const PIN_MASK: [Bitboard; 4096] = [");
    for mv in moves {
        writeln!(output, "\tBitboard(0x{:x}),", mv.0);
    }
    writeln!(output, "];")
}

pub fn generate_zobrist_numbers() -> std::io::Result<()> {
    // [KingSquare * 64 + EnemySquare] = Path between King and Enemy including the Enemy. Zero if not a slider
    let mut moves: [u64; 64 * 12] = [0; 64 * 12];

    for i in 0..(64 * 12) {
        moves[i] = rand::random::<u64>();
    }

    let mut output = Fil::create("src/lookup/zobrist.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(
        output,
        "// [Square * 12 + (*1 for white, *2 for black; Pawn, Bishop, Knight, Rook, Queen, King)] = Path between King and Enemy including the Enemye"
    );
    writeln!(output, "pub const ZOBRIST_VALUES: [u64; 768] = [");
    for mv in moves {
        writeln!(output, "\t0x{:x},", mv);
    }
    writeln!(output, "];")
}
