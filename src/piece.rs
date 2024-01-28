use crate::board::Color;
use crate::role::Role;

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) role: Role,
}
impl Piece {
    pub fn from_char(c: char) -> Piece {
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        let role = Role::from_char(c.to_ascii_uppercase());

        Piece { color, role }
    }

    pub fn get_unicode(&self) -> &str {
        match self.role {
            Role::Pawn => {
                if self.color == Color::White {
                    "♙"
                } else {
                    "♟︎︎"
                }
            }
            Role::Bishop => {
                if self.color == Color::White {
                    "♗"
                } else {
                    "♝"
                }
            }
            Role::Knight => {
                if self.color == Color::White {
                    "♘"
                } else {
                    "♞"
                }
            }
            Role::Rook => {
                if self.color == Color::White {
                    "♖"
                } else {
                    "♜"
                }
            }
            Role::Queen => {
                if self.color == Color::White {
                    "♕"
                } else {
                    "♛"
                }
            }
            Role::King => {
                if self.color == Color::White {
                    "♔"
                } else {
                    "♚"
                }
            }
        }
    }
}
