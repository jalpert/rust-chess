use crate::board::path::*;
use crate::board::*;

impl Board {
    
    /** Returns true if PLAYER is in checkmate. False otherwise */
    pub fn check_mate(&self, player: Option<Color>) -> bool {
        for from in self.find_pieces(player) {
            for to in iproduct!(0..8, 0..8) {
                if self.validate_move(from, to, player).is_ok() {
                    return false;
                }
            }
        }
        true
    }
    
    /**
    Returns the number of squares that would be attacking a King at square
    KING_POSITION of color KING_COLOR. Returns 0 if not in check.
    */
    pub fn num_checking(&self, king_position: Square, king_color: Option<Color>) -> usize {
        self.squares_checking(king_position, king_color).len()
    }

    /**
    Returns a vector containing all the squares with Pieces that could attack a King at square
    KING_POSITION of color KING_COLOR. Returns empty vector if so such squares exist
    */
    pub fn squares_checking(&self, king_position: Square, king_color: Option<Color>) -> Vec<Square> {
        let king_color = king_color.unwrap_or(self.player);
        [
            self.ranged_checking(king_position, king_color),
            self.pawns_checking(king_position, king_color),
            self.knights_checking(king_position, king_color),
            self.kings_checking(king_position, king_color),
        ]
        .concat()
    }

    /**
    If the sqaure at BLOCKING_SQUARE is pinned such that it is shielding SHIELDED_SQUARE,
    returns the direction of the pin, i.e. SHIELDED_SQUARE -> BLOCKING_SQUARE.
    Otherwise, returns None
    Algorithm:
        1) Ensure that the there are no pieces between the piece that is shielded and the
            piece that is blocking.
        2) Beginning at BLOCKING_SQUARE, progress along the path towards SHIELDED_SQUARE until a piece
        is found or the end of the board is reached. If the piece belongs to the opposing player
        and can attack along that direction, return the direction. Otherwise, return None.
    */
    pub fn is_pinned(
        &self,
        blocking_square: Square, // the square with the piece that may or may not be pinned
        shielded_square: Square, // The square against which the pin is taking place (usually holds a King)
        player: Option<Color>,  // the color of the blocking and shielded pieces, current player if None
    ) -> Option<Direction> {
        let player = player.unwrap_or(self.player);
        // Return None if the direction doesn't exist
        let direction = Self::get_direction(shielded_square, blocking_square)?;
        //
        if self.clear_path(blocking_square, shielded_square) {
            let path = Self::get_directed_path(blocking_square, direction);
            let closure = Self::get_directed_closure(direction, player.other());
            if self.check_squares(path, &closure).is_some() {
                return Some(direction);
            }
        }
        None
    }
    
    /**
    Returns the position of the square in SQUARES containing a piece that satisfies f.
    If no piece satisfies, returns None.
    1) Filter squares to only check squares that are in bounds.
    2) Check each square in order:
        If a piece PIECE exists in that square, end the search.
        If f(PIECE) == TRUE, return the square. Otherwise, return None
    */
    fn check_squares<F>(
        &self,
        squares: Vec<Square>,
        // TODO: directions: Box<dyn Iterator<Item = Vec<Square>>>,
        f: &F, // some closure that returns true if the piece matches, false otherwise
    ) -> Option<Square>
    where
        F: Fn(Piece) -> bool,
    {
        // Filter out squares that are out of bounds
        let squares: Vec<Square> = squares.into_iter().filter(|x| in_bounds(*x)).collect();
        for loc in squares {
            match self.get(loc) {
                None => continue,
                Some(piece) => {
                    if f(piece) {
                        return Some(loc);
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }
    // Check horizontally, vertically and diagonally for pieces checking the king
    fn ranged_checking(&self, king_position: Square, king_color: Color) -> Vec<Square> {
        let directions = [
            (Increasing, Zero),
            (Decreasing, Zero),
            (Zero, Increasing),
            (Zero, Decreasing),
            (Increasing, Increasing),
            (Decreasing, Decreasing),
            (Decreasing, Increasing),
            (Increasing, Decreasing),
        ];
        directions
            .iter()
            .filter_map(|direction| {
                let path = Self::get_directed_path(king_position, *direction);
                let closure = Self::get_directed_closure(*direction, king_color.other());
                self.check_squares(path, &closure)
            })
            .collect()
    }
    // Check each square—defined as an offset to the king's position—for pieces that match closure
    fn relative_checking<F>(
        &self,
        king_position: Square,
        squares: Vec<Square>,
        closure: &F,
    ) -> Vec<Square>
    where
        F: Fn(Piece) -> bool,
    {
        squares
            .into_iter()
            .map(|x| vec![(king_position.0 + x.0, king_position.1 + x.1)])
            .filter_map(|squares| self.check_squares(squares, &closure))
            .collect()
    }
    // Check for 2 directions, diagonal-left and diagonal-right
    fn pawns_checking(&self, king_position: Square, king_color: Color) -> Vec<Square> {
        let closure = |piece: Piece| piece == Piece(Pawn, king_color.other());
        let squares = match king_color {
            White => vec![(1, 1), (1, -1)],
            Black => vec![(-1, 1), (-1, -1)],
        };
        self.relative_checking(king_position, squares, &closure)
    }
    // Check 8 directions, each containing one square
    fn knights_checking(&self, king_position: Square, king_color: Color) -> Vec<Square> {
        let closure = |piece: Piece| piece == Piece(Knight, king_color.other());
        let squares = vec![
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ];
        self.relative_checking(king_position, squares, &closure)
    }
    fn kings_checking(&self, king_position: Square, king_color: Color) -> Vec<Square> {
        let closure = |piece: Piece| piece == Piece(King, king_color.other());
        let squares = vec![
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        self.relative_checking(king_position, squares, &closure)
    }
    
    fn get_directed_closure(dir: Direction, color: Color) -> Box<dyn Fn(Piece) -> bool> {
        match dir {
            (Zero, Zero) => panic!("This is not a direction!"),
            (Zero, _) | (_, Zero) => Box::new(move |piece: Piece| {
                piece == Piece(Rook, color) || piece == Piece(Queen, color)
            }),
            _ => Box::new(move |piece: Piece| {
                piece == Piece(Bishop, color) || piece == Piece(Queen, color)
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_checking() {
        let mut board = Board::empty(White, 0);

        let king_position = (0, 4);
        board.set(king_position, Some(Piece(King, White)));
        // These Pieces are checking the King
        board.set((0, 0), Some(Piece(Rook, Black)));
        board.set((0, 6), Some(Piece(Queen, Black)));
        board.set((1, 3), Some(Piece(Pawn, Black)));
        board.set((2, 6), Some(Piece(Bishop, Black)));
        // These Pieces are not checking the King
        board.set((3, 7), Some(Piece(Rook, Black)));
        board.set((4, 0), Some(Piece(Queen, Black)));
        board.set((5, 4), Some(Piece(Queen, White)));
        assert_eq!(board.num_checking(king_position, None), 4);
    }
    #[test]
    fn test_horizonal_check() {
        let mut board = Board::empty(Black, 0);
        let king_position = (4, 6);
        board.set(king_position, Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // The Rook is Checking the King
        board.set((4, 1), Some(Piece(Rook, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Block the Check with a Black Piece
        board.set((4, 3), Some(Piece(Pawn, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // The Queen is Checking the King
        board.set((4, 4), Some(Piece(Queen, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // The Rook is Checking the King
        board.set((4, 7), Some(Piece(Rook, White)));
        assert_eq!(board.num_checking(king_position, None), 2);
    }
    #[test]
    fn test_vertical_check() {
        let mut board = Board::empty(Black, 0);
        let king_position = (6, 4);
        board.set(king_position, Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // The Rook is Checking the King
        board.set((1, 4), Some(Piece(Rook, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Block the Check with a Black Piece
        board.set((3, 4), Some(Piece(Pawn, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // The Queen is Checking the King
        board.set((4, 4), Some(Piece(Queen, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // The Queen is Checking the King
        board.set((7, 4), Some(Piece(Queen, White)));
        assert_eq!(board.num_checking(king_position, None), 2);
    }
    #[test]
    fn test_diagonal_check() {
        let mut board = Board::empty(White, 0);
        let king_position = (4, 4);
        board.set(king_position, Some(Piece(King, White)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // The Bishop is Checking the King
        board.set((0, 0), Some(Piece(Bishop, Black)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // The Queen is Checking the King
        board.set((7, 1), Some(Piece(Queen, Black)));
        assert_eq!(board.num_checking(king_position, None), 2);
        // The Bishop is Checking the King
        board.set((5, 5), Some(Piece(Bishop, Black)));
        assert_eq!(board.num_checking(king_position, None), 3);
        // The Bishop is Checking the King
        board.set((2, 6), Some(Piece(Bishop, Black)));
        assert_eq!(board.num_checking(king_position, None), 4);
        // A Pawn Blocks the Bishop
        board.set((3, 5), Some(Piece(Pawn, White)));
        assert_eq!(board.num_checking(king_position, None), 3);
    }
    #[test]
    fn test_pawns_check_white() {
        let mut board = Board::empty(White, 0);
        let king_position = (4, 4);
        board.set(king_position, Some(Piece(King, White)));
        assert_eq!(board.num_checking(king_position, None), 0);

        let pawn_row = king_position.0 + 1;
        // Pawn is Checking King
        board.set((pawn_row, king_position.1 - 1), Some(Piece(Pawn, Black)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Pawn is NOT Checking King
        board.set((pawn_row, king_position.1), Some(Piece(Pawn, Black)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Pawn is Checking King
        board.set((pawn_row, king_position.1 + 1), Some(Piece(Pawn, Black)));
        assert_eq!(board.num_checking(king_position, None), 2);
    }
    #[test]
    fn test_pawns_check_black() {
        let mut board = Board::empty(Black, 0);
        let king_position = (4, 4);
        board.set(king_position, Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);

        let pawn_row = king_position.0 - 1;
        // Pawn is Checking King
        board.set((pawn_row, king_position.1 - 1), Some(Piece(Pawn, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Pawn is NOT Checking King
        board.set((pawn_row, king_position.1), Some(Piece(Pawn, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Pawn is Checking King
        board.set((pawn_row, king_position.1 + 1), Some(Piece(Pawn, White)));
        assert_eq!(board.num_checking(king_position, None), 2);
    }
    #[test]
    fn test_knights_check() {
        let mut board = Board::empty(Black, 0);
        let king_position = (4, 4);
        board.set(king_position, Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // Knight is checking King
        board.set((6, 3), Some(Piece(Knight, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // Knight is NOT checking King
        board.set((2, 1), Some(Piece(Knight, White)));
        assert_eq!(board.num_checking(king_position, None), 1);
    }
    #[test]
    fn test_kings_check() {
        let mut board = Board::empty(White, 0);
        let king_position = (4, 4);
        board.set(king_position, Some(Piece(King, White)));
        assert_eq!(board.num_checking(king_position, None), 0);
        // King is checking King
        board.set((4, 5), Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // King is NOT checking King
        board.set((4, 6), Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 1);
        // King is checking King
        board.set((5, 5), Some(Piece(King, Black)));
        assert_eq!(board.num_checking(king_position, None), 2);
    }
    #[test]
    fn test_pinning_single() {
        let mut board = Board::empty(White, 0);
        let (king, attacker, blocker) = ((4, 2), (0, 6), (3, 3));
        board.set(king, Some(Piece(King, White)));
        board.set(attacker, Some(Piece(Queen, Black)));
        board.set(blocker, Some(Piece(Pawn, White)));

        assert_eq!(board.is_pinned(blocker, king, None), Some((Decreasing, Increasing)));
    }
    #[test]
    fn test_pinning_double() {
        let mut board = Board::empty(White, 0);
        let (king, attacker, blocker_1, blocker_2) = ((4, 2), (0, 6), (3, 3), (2, 4));
        board.set(king, Some(Piece(King, White)));
        board.set(attacker, Some(Piece(Queen, Black)));
        board.set(blocker_1, Some(Piece(Pawn, White)));
        board.set(blocker_2, Some(Piece(Pawn, White)));
        assert!(board.is_pinned(blocker_1, king, None).is_none());
        assert!(board.is_pinned(blocker_2, king, None).is_none());
    }
    
    #[test]
    fn integration_test() {
        let board = *Board::empty(White, 25)
            // White
            .set((0, 0), Some(Piece(Rook, White)))
            .set((0, 4), Some(Piece(King, White)))
            .set((1, 0), Some(Piece(Pawn, White)))
            .set((2, 4), Some(Piece(Pawn, White)))
            // Black
            .set((1, 5), Some(Piece(Queen, Black)))
            .set((4, 2), Some(Piece(King, Black)))
            .set((3, 2), Some(Piece(Pawn, Black)))
            // .set((3, 3), Some(Piece(Knight, Black)))
            .set((3, 5), Some(Piece(Pawn, Black)))
            .set((3, 7), Some(Piece(Bishop, Black)))
            .set((2, 6), Some(Piece(Rook, Black)));
        board.validate_move((0,4), (1,5), None).unwrap();
        assert!(!board.check_mate(None));
    }
}
