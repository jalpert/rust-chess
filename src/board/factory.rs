// Jack Alpert 2020

use crate::board::*;
use std::str::FromStr;

use rand::seq::SliceRandom;
use rand::thread_rng;

impl Board {
    pub fn new() -> Board {
        Board {
            player: White,
            turn_no: 0,
            data: [
                [
                    Some(Piece(Rook, White)), // Row 1
                    Some(Piece(Knight, White)),
                    Some(Piece(Bishop, White)),
                    Some(Piece(Queen, White)),
                    Some(Piece(King, White)),
                    Some(Piece(Bishop, White)),
                    Some(Piece(Knight, White)),
                    Some(Piece(Rook, White)),
                ],
                [Some(Piece(Pawn, White)); 8], // Row 2
                [None; 8],                     // Row 3
                [None; 8],                     // Row 4
                [None; 8],                     // Row 5
                [None; 8],                     // Row 6
                [Some(Piece(Pawn, Black)); 8], // Row 7
                [
                    Some(Piece(Rook, Black)), // Row 8
                    Some(Piece(Knight, Black)),
                    Some(Piece(Bishop, Black)),
                    Some(Piece(Queen, Black)),
                    Some(Piece(King, Black)),
                    Some(Piece(Bishop, Black)),
                    Some(Piece(Knight, Black)),
                    Some(Piece(Rook, Black)),
                ],
            ],
        }
    }
    pub fn empty(player: Color, turn_no: u8) -> Board {
        Board {
            player: player,
            turn_no: turn_no,
            data: [[None; 8]; 8],
        }
    }
    pub fn random_move(board: &Board) -> (Square, Square) {
        let mut rng = thread_rng();
        //
        // Choose a piece at random
        let mut from_sqaures: Vec<Square> = board.find_pieces(None);
        from_sqaures.shuffle(&mut rng);
        for from in from_sqaures {
            //
            // Choose a valid TO at random
            let to_squares: Vec<Square> = iproduct!(0..8, 0..8)
                .filter(|&loc| board.validate_move(from, loc, None).is_ok())
                .collect();
            // If TO_SQAURES is Empty, no valid moves exist between FROM and TO
            // Try another FROM
            if let Some(&to) = to_squares.choose(&mut rng) {
                return (from, to);
            }
        }
        panic!(
            "On turn {}, {} has no valid moves",
            board.turn(),
            board.player()
        );
    }
    // pub fn later_board() -> Board {
    //     let mut board = Board::empty(White, 25);
    //
    //     *board
    //         // White
    //         .set((0, 0), Some(Piece(Rook, White)))
    //         .set((0, 4), Some(Piece(King, White)))
    //         .set((1, 0), Some(Piece(Pawn, White)))
    //         .set((2, 4), Some(Piece(Pawn, White)))
    //         // Black
    //         .set((1, 5), Some(Piece(Queen, Black)))
    //         .set((4, 2), Some(Piece(King, Black)))
    //         .set((3, 2), Some(Piece(Pawn, Black)))
    //         // .set((3, 3), Some(Piece(Knight, Black)))
    //         .set((3, 5), Some(Piece(Pawn, Black)))
    //         .set((3, 7), Some(Piece(Bishop, Black)))
    //         .set((2, 6), Some(Piece(Rook, Black)))
    // }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(contents: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        let mut lines = contents.lines();
        // Read the player
        let player: Color = match lines.next() {
            Some("White") => White,
            Some("Black") => Black,
            _ => return Err("Couldn't parse player!".to_string()),
        };
        //
        // Read the turn number
        let turn_no: u8 = lines
            .next()
            .map(|s| s.parse().ok())
            .flatten()
            .ok_or("Couldn't parse turn number!".to_string())?;
        //
        // Read the board setup
        let (mut num_white_kings, mut num_black_kings) = (0, 0);
        let mut data: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for (line, row) in lines.zip(0..8) {
            for (piece, col) in line.split_whitespace().zip(0..8) {
                let piece = piece.parse().ok();
                data[row][col] = piece;
                if Some(Piece(King, White)) == piece {
                    num_white_kings += 1;
                } else if Some(Piece(King, Black)) == piece {
                    num_black_kings += 1;
                }
            }
        }

        if (num_white_kings, num_black_kings) == (1, 1) {
            Ok(Board {
                player,
                turn_no,
                data,
            })
        } else {
            Err(String::from("Wrong number of Kings on the board."))
        }
    }
}
