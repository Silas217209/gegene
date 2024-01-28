pub(crate) mod bishop_mask;
pub(crate) mod bishop_moves;
pub(crate) mod direction_mask;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod rook_mask;
pub(crate) mod rook_moves;

use crate::bitboard::Bitboard;
use crate::board::{File, Rank};
use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::rook_mask::ROOK_MASK;
use crate::pdep::Pdep;
use crate::pext::Pext;
use std::collections::HashMap;
use std::fs::File as Fil;
use std::io::Write;
use std::u16;

pub fn generate_king_moves() -> std::io::Result<()> {
    let mut moves: [Bitboard; 64] = [Bitboard(0); 64];

    for square in 0..64 {
        println!("square: {}", square);
        let mut bitboard = Bitboard(0);

        let north = square < 56;
        let south = square > 7;
        let east = square % 8 < 7;
        let west = square % 8 > 0;

        if north {
            bitboard |= Bitboard(1 << square + 8);
        }

        if south {
            bitboard |= Bitboard(1 << square - 8);
        }

        if east {
            bitboard |= Bitboard(1 << square + 1);
        }

        if west {
            bitboard |= Bitboard(1 << square - 1);
        }

        if north && east {
            bitboard |= Bitboard(1 << square + 9);
        }

        if north && west {
            bitboard |= Bitboard(1 << square + 7);
        }

        if south && east {
            bitboard |= Bitboard(1 << square - 7);
        }

        if south && west {
            bitboard |= Bitboard(1 << square - 9);
        }

        println!("{}", bitboard);

        moves[square] = bitboard;
    }

    let mut output = Fil::create("lookup/king.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(output, "pub const KING_MOVES: [Bitboard; 64] = [");
    for mv in moves {
        writeln!(output, "\tBitboard(0x{:x}),", mv.0);
    }
    writeln!(output, "];")
}

pub fn generate_knight_moves() -> std::io::Result<()> {
    let mut moves: [Bitboard; 64] = [Bitboard(0); 64];

    for square in 0..64 {
        let mut bitboard = Bitboard(0);

        let up_2 = square < 48;
        let down_2 = square > 15;

        let up_1 = square < 56;
        let down_1 = square > 7;

        let left_2 = square % 8 > 1;
        let right_2 = square % 8 < 6;

        let left_1 = square % 8 > 0;
        let right_1 = square % 8 < 7;

        if up_2 && right_1 {
            bitboard |= Bitboard(1 << square + 17);
        }

        if up_2 && left_1 {
            bitboard |= Bitboard(1 << square + 15);
        }

        if up_1 && right_2 {
            bitboard |= Bitboard(1 << square + 10);
        }

        if up_1 && left_2 {
            bitboard |= Bitboard(1 << square + 6);
        }

        if down_1 && right_2 {
            bitboard |= Bitboard(1 << square - 6);
        }

        if down_1 && left_2 {
            bitboard |= Bitboard(1 << square - 10);
        }

        if down_2 && right_1 {
            bitboard |= Bitboard(1 << square - 15);
        }

        if down_2 && left_1 {
            bitboard |= Bitboard(1 << square - 17);
        }

        moves[square] = bitboard;
        println!("square {}", square);
        println!("{}", bitboard);
    }

    let mut output = Fil::create("lookup/knight.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(output, "pub const KNIGHT_MOVES: [Bitboard; 64] = [");
    for mv in moves {
        writeln!(output, "\tBitboard(0x{:x}),", mv.0);
    }
    writeln!(output, "];")
}

pub fn generate_rook_mask() -> std::io::Result<()> {
    // gererate bitboards for all possible rook moves, excluding the borders (rank 1, 8, file A, H)
    let mut moves: [(Bitboard, u64); 64] = [(Bitboard(0), 0); 64];
    let mut keys: HashMap<u16, Bitboard> = HashMap::new();
    let mut offset = 0;

    for square in 0..64 {
        let rank_number = square / 8;
        let file_number = square % 8;

        let rank = Bitboard::from_rank_number(rank_number);
        let file = Bitboard::from_file_number(file_number);

        let excluded = (rank & (Bitboard::from_file_number(0) | Bitboard::from_file_number(7)))
            | (file & (Bitboard::from_rank_number(0) | Bitboard::from_rank_number(7)))
            | Bitboard(1 << square);

        let mask = (rank | file) ^ excluded;
        let combinations = 2u64.pow(mask.0.count_ones());
        moves[square as usize] = (mask, offset);
        offset += combinations;
    }

    let mut output = Fil::create("lookup/rook_mask.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(output, "pub const ROOK_MASK: [(Bitboard, u64); 64] = [");
    for mv in moves {
        writeln!(output, "\t(Bitboard(0x{:x}), {}),", mv.0 .0, mv.1);
    }
    writeln!(output, "];")
}

pub fn generate_bishop_mask() -> std::io::Result<()> {
    // gererate bitboards for all possible bishop moves (diagonal), excluding the borders (rank 1, 8, file A, H)
    let mut moves: [(Bitboard, u64); 64] = [(Bitboard(0), 0); 64];
    let mut offset = 0;

    for square in 0..64 {
        let rank_number = square / 8;
        let file_number = square % 8;

        let mut diagonals = Bitboard(0);
        // loop sout east
        for i in 1..8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number + i > 6 || file_number + i > 6 {
                break;
            }
            diagonals |= Bitboard(1 << (square + i * 9));
        }

        // loop sout west
        for i in 1..8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number + i > 6 || file_number - i < 1 {
                break;
            }
            diagonals |= Bitboard(1 << (square + i * 7));
        }

        // loop north east
        for i in 1..8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number - i < 1 || file_number + i > 6 {
                break;
            }
            diagonals |= Bitboard(1 << (square - i * 7));
        }

        // loop north west
        for i in 1..8 {
            // check if at border, but dont add the square itself to the mask
            if rank_number - i < 1 || file_number - i < 1 {
                break;
            }
            diagonals |= Bitboard(1 << (square - i * 9));
        }

        moves[square as usize] = (diagonals, offset);

        let combinations = 2u64.pow(diagonals.0.count_ones());
        offset += combinations;
    }

    let mut output = Fil::create("lookup/bishop_mask.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(output, "pub const BISHOP_MASK: [(Bitboard, u64); 64] = [");
    for mv in moves {
        writeln!(output, "\t(Bitboard(0x{:x}), {}),", mv.0 .0, mv.1);
    }
    writeln!(output, "];")
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

pub fn generate_ray_mask() -> std::io::Result<()> {
    let mut moves: [(Bitboard, Bitboard, Bitboard, Bitboard); 64] =
        [(Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)); 64];

    for square in 0..64 {
        let file = square % 8;
        let rank = square / 8;

        println!("{file}, {rank}");

        let mut north_mask = Bitboard(0);
        for i in rank..8 {
            north_mask |= Bitboard::from_rank_number(i);
        }

        let mut east_mask = Bitboard(0);
        for i in file..8 {
            east_mask |= Bitboard::from_file_number(i);
        }

        let mut south_mask = Bitboard(0);
        for i in 0..=rank {
            south_mask |= Bitboard::from_rank_number(i);
        }

        let mut west_mask = Bitboard(0);
        for i in 0..=file {
            west_mask |= Bitboard::from_file_number(i);
        }

        if square == 22 {
            println!("north mask:\n{}", north_mask);
            println!("east mask:\n{}", east_mask);
            println!("south mask:\n{}", south_mask);
            println!("west mask:\n{}", west_mask);
        }

        moves[square] = (north_mask, east_mask, south_mask, west_mask);
    }

    let mut output = Fil::create("src/lookup/direction_mask.rs")?;
    writeln!(output, "use crate::bitboard::Bitboard;\n");
    writeln!(
        output,
        "pub const DIRECTION_MASK: [(Bitboard, Bitboard, Bitboard, Bitboard); 64] = ["
    );
    for mv in moves {
        writeln!(
            output,
            "\t(Bitboard(0x{:x}), Bitboard(0x{:x}), Bitboard(0x{:x}), Bitboard(0x{:x})),",
            mv.0 .0, mv.1 .0, mv.2 .0, mv.3 .0
        );
    }
    writeln!(output, "];")
}
