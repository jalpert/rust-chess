use crate::board::*;
use std::fmt::*;
use ansi_term::Style;
use ansi_term::Colour;


impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match &self {
                White => "White",
                Black => "Black",
            }
        )
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match &self {
                Increasing => "Increasing",
                Decreasing => "Decreasing",
                Zero => "Zero",
            }
        )
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match &self {
                Piece(Pawn, White) => "♙",
                Piece(Pawn, Black) => "♟",
                Piece(Rook, White) => "♖",
                Piece(Rook, Black) => "♜",
                Piece(Knight, White) => "♘",
                Piece(Knight, Black) => "♞",
                Piece(Bishop, White) => "♗",
                Piece(Bishop, Black) => "♝",
                Piece(Queen, White) => "♕",
                Piece(Queen, Black) => "♛",
                Piece(King, White) => "♔",
                Piece(King, Black) => "♚",
            }
        )
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Label each column with its appropriate index
        write!(f, "     1  2  3  4  5  6  7  8\n")?;
        for row in 0..8 {
            // Mark each row with its index
            write!(f, " {}  ", row + 1)?;
            for col in 0..8 {
                let tile_color = if (row + col) % 2 == 0 {
                    Colour::Fixed(245)
                } else {
                    Colour::White
                };
                write!(
                    f,
                    "{}",
                    Style::new().on(tile_color).paint(
                        match self.get((row, col)) {
                            Some(p) => format!(" {} ", p),
                            None => format!("   "),
                        }
                    ),
                )?;
            }
            // Add a new line for each row
            write!(f, "\n")?;
        }

        // Return a Result
        write!(f, "")
    }
}
