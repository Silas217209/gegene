use crate::role::Role;

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub is_white: bool,
    pub role: Role,
}
impl Piece {
    pub fn from_char(c: char) -> Piece {
        let color = c.is_uppercase();

        let role = Role::from_char(c.to_ascii_uppercase());

        Piece { is_white: color, role }
    }

    pub fn get_unicode(&self) -> &str {
        match self.role {
            Role::Pawn => {
                if self.is_white {
                    "♙"
                } else {
                    "♟︎︎"
                }
            }
            Role::Bishop => {
                if self.is_white {
                    "♗"
                } else {
                    "♝"
                }
            }
            Role::Knight => {
                if self.is_white {
                    "♘"
                } else {
                    "♞"
                }
            }
            Role::Rook => {
                if self.is_white {
                    "♖"
                } else {
                    "♜"
                }
            }
            Role::Queen => {
                if self.is_white {
                    "♕"
                } else {
                    "♛"
                }
            }
            Role::King => {
                if self.is_white {
                    "♔"
                } else {
                    "♚"
                }
            }
        }
    }
}
