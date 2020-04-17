use crate::board::*;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
pub use PieceType::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Piece(pub PieceType, pub Color);

impl Piece {
    pub fn piece_type(&self) -> PieceType {
        self.0
    }
    pub fn color(&self) -> Color {
        self.1
    }
    pub fn can_move(&self, board: &Board, from: Square, to: Square) -> Result<(), String> {
        let capturing_piece = board.get(to);
        match self {
            Piece(Pawn, White) => {
                if capturing_piece.is_none() {
                    // No capturing allowed
                    if to.1 == from.1 && to.0 - from.0 == 1 {
                       return Ok(()) // Move forward one square
                    } else if to.1 == from.1 && from.0 == 1 && to.0 - from.0 == 2 {
                       return Ok(()) // Starting at home, move forward 2 squares
                    }
                } else {
                    // Capture a piece by moving one square forward and one square to either side
                    if (to.1 - from.1).abs() == 1 && to.0 - from.0 == 1 {
                       return Ok(())
                    }
                }
            }
            Piece(Pawn, Black) => {
                if capturing_piece.is_none() {
                    // No capturing allowed
                    if to.1 == from.1 && to.0 - from.0 == -1 {
                       return Ok(())// Move forward one square
                    } else if to.1 == from.1 && from.0 == 6 && to.0 - from.0 == -2 {
                       return Ok(())// Starting at home, move forward 2 squares
                    }
                } else {
                    // Capturing allowed
                    if (to.1 - from.1).abs() == 1 && to.0 - from.0 == -1 {
                       return Ok(())// Capture a piece by moving one square forward and one square to either side
                    }
                }
            }
            Piece(Rook, ..) => {
                if is_horizontal(from, to) && board.clear_path(from, to) {
                   return Ok(())// Horizontal Move
                } else if is_vertical(from, to) && board.clear_path(from, to) {
                   return Ok(())// Vertical move
                }
            }
            Piece(Bishop, ..) => {
                if is_diagonal(from, to) && board.clear_path(from, to) {
                    return Ok(())
                }
            }
            Piece(Queen, ..) => {
                if (is_diagonal(from, to) && board.clear_path(from, to))
                    || (is_vertical(from, to) && board.clear_path(from, to))
                    || (is_horizontal(from, to) && board.clear_path(from, to))
                {
                    return Ok(())
                }
            }
            Piece(Knight, ..) => match ((from.0 - to.0).abs(), (from.1 - to.1).abs()) {
                (2, 1) => return Ok(()),
                (1, 2) => return Ok(()),
                _ => (),
            },
            Piece(King, ..) => {
                if (from.0 - to.0).abs() <= 1 && (from.1 - to.1).abs() <= 1 {
                    return Ok(())
                }
            }
        };
        return Err(String::from("Invalid move."));
    }
    pub fn is_king(&self) -> bool {
        return self.piece_type() == King;
    }
}
