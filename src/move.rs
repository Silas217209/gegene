use std::fmt::Display;
use std::str;

use crate::board::{Board, File, Rank};
use crate::piece::Piece;
use crate::role::{PromotionRole, Role};

#[derive(Debug, Clone, Copy)]
pub struct Square(pub u8);

impl Square {
    pub fn from_algebraic(algebraic: &str) -> Result<Square, &str> {
        if algebraic.len() != 2 {
            return Err("The input should have a lenght of exactly 2");
        }
        let first_char = algebraic.chars().nth(0);

        if first_char.is_none() {
            return Err("square string is invalid (first char)");
        }
        let second_char = algebraic.chars().nth(1);
        if second_char.is_none() {
            return Err("square string is invalid (second char)");
        }
        let file = match first_char.unwrap() {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => return Err("invalid File"),
        };

        let rank = match second_char.unwrap() {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err("invalid Rank"),
        };

        Ok(Square((rank as u8) * 8 + (file as u8)))
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn to_algebraic(self) -> String {
        let rank = (self.rank() + 49) as char;
        let file = (self.file() + 97) as char;

        return format!("{file}{rank}");
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_algebraic().as_str())
    }
}
#[derive(Debug, Copy, Clone)]
pub enum MoveType {
    Quiet = 0,
    DoublePawnPush = 1,
    EnPassant = 2,
    Promotion = 3,
    KingsideCastle = 4,
    QueensideCastle = 5,
}

// 1: 1     color 1: white, 0: black
// 6: 2-7   from
// 6: 8-13  to
// 3: 14-16 piece type Pawn = 0, Bishop = 1, Knight = 2, Rook = 3, Queen = 4, King = 5
// 1: 17    capture 0: no capture, 1: capture
// 3: 18-20 Capture role type Pawn = 0, Bishop = 1, Knight = 2, Rook = 3, Queen = 4, King = 5
// 2: 21-22 Promotion Queen = 0, Rook = 1, Bishop = 2, Knight = 3
// 2: 23-25 Special 0: Quiet, 1: Double pawn push, 2: En Passant, 3: Promotion, 4: Kingside castle, 5: Queenside castle
#[derive(Debug, Copy, Clone)]
pub struct Move(pub u32);

impl Move {
    pub fn null() -> Move {
        return Move(0);
    }
    pub fn new(
        is_white: bool,
        from: Square,
        to: Square,
        role: Role,
        is_capture: bool,
        capture_role: Role,
        promotion: PromotionRole,
        move_type: MoveType,
    ) -> Move {
        let color = is_white as u32;
        let from_bitboard = (from.0 as u32) << 1;
        let to_bitboard = (to.0 as u32) << 7;
        let role = (role as u32).wrapping_shl(13);
        let is_capture = (is_capture as u32).wrapping_shl(16);
        let capture_role = (capture_role as u32).wrapping_shl(17);
        let promotion = (promotion as u32).wrapping_shl(20);
        let move_type = (move_type as u32).wrapping_shl(22);

        Move(
            color
                | from_bitboard
                | to_bitboard
                | role
                | is_capture
                | capture_role
                | promotion
                | move_type,
        )
    }
    pub fn kingside_castle(is_white: bool) -> Move {
        Move::new(
            is_white,
            if is_white { Square(4) } else { Square(60) },
            if is_white { Square(6) } else { Square(62) },
            Role::King,
            false,
            Role::Pawn,
            PromotionRole::Queen,
            MoveType::KingsideCastle,
        )
    }
    pub fn queenside_castle(is_white: bool) -> Move {
        Move::new(
            is_white,
            if is_white { Square(4) } else { Square(60) },
            if is_white { Square(2) } else { Square(58) },
            Role::King,
            false,
            Role::Pawn,
            PromotionRole::Queen,
            MoveType::QueensideCastle,
        )
    }
    pub fn is_white(&self) -> bool {
        self.0 & 0b1 == 1
    }
    pub fn from(&self) -> Square {
        let from = self.0.wrapping_shr(1) & 0b111111;
        Square(from as u8)
    }
    pub fn to(&self) -> Square {
        let to = self.0.wrapping_shr(7) & 0b111111;
        Square(to as u8)
    }
    pub fn role(&self) -> Role {
        match (self.0.wrapping_shr(13)) & 0b111 {
            0 => Role::Pawn,
            1 => Role::Bishop,
            2 => Role::Knight,
            3 => Role::Rook,
            4 => Role::Queen,
            5 => Role::King,
            _ => Role::Pawn,
        }
    }
    pub fn is_capture(&self) -> bool {
        self.0 & 1 << 16 != 0
    }
    pub fn capture_role(&self) -> Role {
        match (self.0.wrapping_shr(17)) & 0b111 {
            0 => Role::Pawn,
            1 => Role::Bishop,
            2 => Role::Knight,
            3 => Role::Rook,
            4 => Role::Queen,
            5 => Role::King,
            _ => Role::Pawn,
        }
    }
    pub fn promotion_role(&self) -> PromotionRole {
        match (self.0.wrapping_shr(20)) & 0b11 {
            0 => PromotionRole::Queen,
            1 => PromotionRole::Rook,
            2 => PromotionRole::Bishop,
            3 => PromotionRole::Knight,
            _ => PromotionRole::Queen,
        }
    }
    pub fn move_type(&self) -> MoveType {
        match (self.0.wrapping_shr(22)) & 0b111 {
            0 => MoveType::Quiet,
            1 => MoveType::DoublePawnPush,
            2 => MoveType::EnPassant,
            3 => MoveType::Promotion,
            4 => MoveType::KingsideCastle,
            5 => MoveType::QueensideCastle,
            _ => MoveType::Quiet,
        }
    }
    pub fn piece(&self) -> Piece {
        Piece {
            is_white: self.is_white(),
            role: self.role(),
        }
    }
    pub fn to_algebraic(self) -> String {
        let from = self.from().to_algebraic();
        let to = self.to().to_algebraic();

        let mut result = String::new();

        result.push_str(from.as_str());
        result.push_str(to.as_str());

        if let MoveType::Promotion = self.move_type() {
            match self.promotion_role() {
                PromotionRole::Queen => result.push('q'),
                PromotionRole::Rook => result.push('r'),
                PromotionRole::Bishop => result.push('b'),
                PromotionRole::Knight => result.push('n'),
            }
        }

        result
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_algebraic().as_str())
    }
}
