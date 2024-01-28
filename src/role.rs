#[derive(Debug, Copy, Clone)]
pub enum Role {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
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
