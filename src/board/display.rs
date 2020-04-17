// Jack Alpert 2020

use crate::board::*;
use ansi_term::{Colour, Style};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl FromStr for Piece {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match s.trim() {
            "♙" => Ok(Piece(Pawn, White)),
            "♟" => Ok(Piece(Pawn, Black)),
            "♖" => Ok(Piece(Rook, White)),
            "♜" => Ok(Piece(Rook, Black)),
            "♘" => Ok(Piece(Knight, White)),
            "♞" => Ok(Piece(Knight, Black)),
            "♗" => Ok(Piece(Bishop, White)),
            "♝" => Ok(Piece(Bishop, Black)),
            "♕" => Ok(Piece(Queen, White)),
            "♛" => Ok(Piece(Queen, Black)),
            "♔" => Ok(Piece(King, White)),
            "♚" => Ok(Piece(King, Black)),
            _ => Err(String::from("Not a recognized piece")),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
                    Style::new()
                        .on(tile_color)
                        .paint(match self.get((row, col)) {
                            Some(p) => format!(" {} ", p),
                            None => format!("   "),
                        }),
                )?;
            }
            // Add a new line for each row
            write!(f, "\n")?;
        }

        // Return a Result
        write!(f, "")
    }
}
