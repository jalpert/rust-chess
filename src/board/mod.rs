// Jack Alpert 2020

use crate::board::path::*;
use crate::board::piece::*;
use itertools::iproduct;

mod check;
mod display;
mod factory;
mod path;
mod piece;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}
pub use Color::*;

impl Color {
    pub fn other(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

// row  , column
pub type Square = (isize, isize);

#[derive(Copy, Clone)]
pub struct Board {
    data: [[Option<Piece>; 8]; 8],
    player: Color,
    turn_no: u8,
}

impl Board {
    pub fn get(&self, loc: Square) -> Option<Piece> {
        self.data[loc.0 as usize][loc.1 as usize]
    }
    fn set(&mut self, loc: Square, piece: Option<Piece>) -> &mut Self {
        self.data[loc.0 as usize][loc.1 as usize] = piece;
        self
    }
    pub fn player(&self) -> Color {
        self.player
    }
    pub fn turn(&self) -> u8 {
        self.turn_no
    }
    /**
    Ensure player owns this piece and a piece exists in this location
    If player is None, defaults to current player
    */
    pub fn validate_from(&self, from: Square, player: Option<Color>) -> Result<(), String> {
        let player = player.unwrap_or(self.player);
        // Ensure square is in bounds
        if !in_bounds(from) {
            Err(format!("{} {} is out of bounds.", from.0 + 1, from.1 + 1))
        } else if self.get(from).is_none() {
            Err(String::from("No piece exists in this location."))
        } else if self.get(from).unwrap().color() != player {
            Err(String::from("This piece does not belong to you."))
        } else {
            Ok(())
        }
    }

    pub fn validate_move(
        &self,
        from: Square,
        to: Square,
        player: Option<Color>,
    ) -> Result<(), String> {
        let player = player.unwrap_or(self.player);
        // Check for out of bounds
        assert!(in_bounds(from)); // coder error
        if to.0 < 0 || to.0 >= 8 || to.1 < 0 || to.1 >= 8 {
            return Err(format!("{} {} is out of bounds.", to.0 + 1, to.1 + 1));
        }
        //
        // Get the piece
        let piece = self
            .get(from)
            .expect("No piece exists here. Must validate from."); // Coder error, hence panic
                                                                  //
                                                                  // Universal rules
        if let Some(piece_occupying_to) = self.get(to) {
            if piece_occupying_to.is_king() {
                return Err(String::from("Cannot capture the King."));
            } else if piece_occupying_to.color() == player {
                return Err(String::from("Can't move here. Square occupied."));
            }
        }
        // capturing_piece // Capture must be none or a piece belonging to the other player
        //     .map_or(true, |piece| piece.color() == player.other()) // None -> true
        //     .as_result((), "Can't move here. Square occupied.")?;
        // capturing_piece // Capture must be none or a piece other than the King
        //     .map_or(true, |piece| !piece.is_king()) // None -> true
        //     .as_result((), "Cannot capture the King.")?;
        piece.can_move(self, from, to)?; // Ensure that this type of piece can make this move

        //
        // Apply Checking Rules
        let king_position = self
            .find_king(Some(player))
            .expect("This player has no King!");
        //
        // Find any opposing piece on the board that is putting the King in check
        let squares_checking = self.squares_checking(king_position, Some(player));
        let move_dir = Self::get_direction(from, to);
        //
        // Apply Checking Rules for Moving the King
        if from == king_position {
            // Cannot move King to location that is being attacked
            if self.num_checking(to, Some(player)) > 0 {
                if squares_checking.len() > 0 {
                    Err(String::from("King is still in check"))
                } else {
                    Err(String::from("King cannot place himself in check."))
                }
            } else {
                // Edge case can arise when the King is in check and moves in the direction
                // away from the attacking piece. The new position would not register as
                // in check, because it is being blocked by the King
                for attacker in &squares_checking {
                    let attack_dir = Self::get_direction(*attacker, from);
                    if attack_dir.is_none() {
                        // If None, attacker is Knight, so this case will not occur
                        continue;
                    }
                    // MOVE_DIR Cannot be None if Moving King
                    if attack_dir == move_dir {
                        return Err(String::from("King is still in check"));
                    }
                }
                // Successfully moving the King out of check
                Ok(())
            }
        } else {
            match (self.is_pinned(from, king_position, Some(player)), &squares_checking.len()) {
                (Some(pin_dir), 0) => {
                    if move_dir.is_none() {
                        Err(String::from("This Knight is pinned. It cannot be moved."))
                    } else if pin_dir == move_dir.unwrap() {
                        // You are allowed to capture the pinning piece
                        // or maintain the pin by moving this piece between
                        // the pinner and the King
                        Ok(())
                    } else {
                        Err(String::from("This piece is pinned. It cannot be moved in this direction."))
                    }
                }
                (Some(_pin_dir), _n) => {
                    // When in check, cannot move a pinned piece no matter what.
                    // Even if capturing/blocking the checking piece, since this piece
                    // is pinned,the King would then still be in check, although
                    // from a different piece as before.
                    Err(String::from(
                        "This piece is pinned. Move another piece to get King out of check.",
                    ))
                }
                (None, 0) => {
                    // Not pinned, not in check. All good.
                    Ok(())
                }
                (None, 1) => {
                    if to == squares_checking[0] {
                        // Capturing the attacking piece
                        Ok(())
                    } else if Self::get_path(squares_checking[0], king_position).map_or(false, |path| path.contains(&to)) {
                        // Blocking i.e. moving between the King and the attacking piece
                        // Path being None indicates this is a Knight checking, which cannot be blocked,
                        // hence return false
                        Ok(())
                    } else {
                        Err(String::from("King is still in check"))
                    }
                }
                (None, _n) => {
                    // Must move the King to get out of a multi-check
                    Err(String::from("Must move King out of check"))
                }
            }
        }
    }
    // Execute the move by returning a copy of self with the changes applied
    pub fn execute_move(&self, from: Square, to: Square) -> Board {
        assert!(self.get(from).is_some());
        let mut new_board = *self; // make a copy
        new_board.set(from, None);
        // Pawns become Queens on opposite row
        let piece = match self.get(from) {
            Some(Piece(Pawn, White)) if to.0 == 7 => Some(Piece(Queen, White)),
            Some(Piece(Pawn, Black)) if to.0 == 0 => Some(Piece(Queen, Black)),
            p => p,
        };
        new_board.set(to, piece);
        new_board.player = self.player.other();
        new_board.turn_no = self.turn_no + 1;
        new_board
    }

    pub fn find_king(&self, player: Option<Color>) -> Option<Square> {
        let player = player.unwrap_or(self.player);
        for loc in iproduct!(0..8, 0..8) {
            if Some(Piece(King, player)) == self.get(loc) {
                return Some(loc);
            }
        }
        None
    }
    pub fn find_pieces(&self, player: Option<Color>) -> Vec<Square> {
        let player = player.unwrap_or(self.player);
        iproduct!(0..8, 0..8)
            .filter_map(|loc| {
                if self.get(loc)?.color() == player {
                    Some(loc)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_king() {
        let board = Board::new();
        assert_eq!(board.find_king(Some(White)), Some((0, 4)));
        assert_eq!(board.find_king(Some(Black)), Some((7, 4)));
    }
    #[test]
    fn test_find_pieces() {
        let board = Board::new();
        let white_locs_true: Vec<Square> = iproduct!(0..=1, 0..8).collect();
        let black_locs_true: Vec<Square> = iproduct!(6..8, 0..8).collect();
        let white_locs_test = board.find_pieces(Some(White));
        let black_locs_test = board.find_pieces(Some(Black));
        // Test for set equality
        for loc in &white_locs_true {
            assert!(white_locs_test.contains(&loc));
        }
        for loc in &white_locs_test {
            assert!(white_locs_true.contains(&loc));
        }
        for loc in &black_locs_true {
            assert!(black_locs_test.contains(&loc));
        }
        for loc in &black_locs_test {
            assert!(black_locs_true.contains(&loc));
        }
    }
}
