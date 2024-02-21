#[derive(Debug, Copy, Clone)]
pub enum Role {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

impl Role {
    pub fn from_char(c: char) -> Role {
        match c {
            'P' | 'p' => Role::Pawn,
            'B' | 'b' => Role::Bishop,
            'N' | 'n' => Role::Knight,
            'R' | 'r' => Role::Rook,
            'Q' | 'q' => Role::Queen,
            'K' | 'k' => Role::King,
            _ => panic!("invalid role"),
        }
    }
}

pub enum PromotionRole {
    Queen = 0,
    Rook = 1,
    Bishop = 2,
    Knight = 3,
}
