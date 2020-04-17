// Jack Alpert 2020

use crate::board::*;
use std::iter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    Increasing,
    Decreasing,
    Zero,
}
impl Sign {
    pub fn from_int(x: isize) -> Sign {
        match x {
            x if x < 0 => Decreasing,
            x if x > 0 => Increasing,
            _ => Zero,
        }
    }
}
pub use Sign::*;

pub type Direction = (Sign, Sign);

pub fn is_horizontal(from: Square, to: Square) -> bool {
    from.0 == to.0
}

pub fn is_vertical(from: Square, to: Square) -> bool {
    from.1 == to.1
}

pub fn is_diagonal(from: Square, to: Square) -> bool {
    to.0 - from.0 == to.1 - from.1 || to.0 - from.0 == -(to.1 - from.1)
}

pub fn in_bounds(loc: Square) -> bool {
    0 <= loc.0 && loc.0 < 8 && 0 <= loc.1 && loc.1 < 8
}

impl Board {
    /**
    Returns the direction from FROM to TO
    Returns None if no path exists or FROM == TO
    Panics if either FROM or TO is out of bounds
    */
    pub fn get_direction(from: Square, to: Square) -> Option<Direction> {
        assert!(in_bounds(from));
        assert!(in_bounds(to));
        if from != to && (is_horizontal(from, to) || is_vertical(from, to) || is_diagonal(from, to))
        {
            Some((Sign::from_int(to.0 - from.0), Sign::from_int(to.1 - from.1)))
        } else {
            None
        }
    }

    /**
    Returns a vector of sqaures in a straight line starting at FROM
    and going until TO (not including either FROM or TO) in whatever direction specified
    Returns None if no path exists or FROM == TO
    */
    pub fn get_path(from: Square, to: Square) -> Option<Vec<Square>> {
        Some(match Self::get_direction(from, to)? {
            (Zero, Increasing) => iter::repeat(from.0).zip((from.1 + 1)..to.1).collect(),
            (Zero, Decreasing) => iter::repeat(from.0).zip((to.1 + 1)..from.1).collect(),
            (Increasing, Zero) => ((from.0 + 1)..to.0).zip(iter::repeat(from.1)).collect(),
            (Decreasing, Zero) => ((to.0 + 1)..from.0).zip(iter::repeat(from.1)).collect(),
            (Increasing, Increasing) => (from.0 + 1..to.0).zip(from.1 + 1..to.1).collect(),
            (Increasing, Decreasing) => (from.0 + 1..to.0).zip((to.1 + 1..from.1).rev()).collect(),
            (Decreasing, Increasing) => (to.0 + 1..from.0).rev().zip(from.1 + 1..to.1).collect(),
            (Decreasing, Decreasing) => (to.0 + 1..from.0)
                .rev()
                .zip((to.1 + 1..from.1).rev())
                .collect(),
            (Zero, Zero) => return None,
        })
    }

    /**
    Returns a vector of sqaures in a straight line starting at FROM (inclusive)
    and going until TO (exclusive) in whatever direction specified
    Returns None if no path exists or FROM == TO
    */
    pub fn get_directed_path(start: Square, direction: Direction) -> Vec<Square> {
        // TODO: split into row and column
        match direction {
            (Increasing, Zero) => (start.0 + 1..8).zip(iter::repeat(start.1)).collect(),
            (Decreasing, Zero) => (0..start.0).rev().zip(iter::repeat(start.1)).collect(),
            (Zero, Increasing) => iter::repeat(start.0).zip(start.1 + 1..8).collect(),
            (Zero, Decreasing) => iter::repeat(start.0).zip((0..start.1).rev()).collect(),
            (Increasing, Increasing) => (start.0 + 1..8).zip(start.1 + 1..8).collect(),
            (Increasing, Decreasing) => (start.0 + 1..8).zip((0..start.1).rev()).collect(),
            (Decreasing, Increasing) => (0..start.0).rev().zip(start.1 + 1..8).collect(),
            (Decreasing, Decreasing) => (0..start.0).rev().zip((0..start.1).rev()).collect(),
            (Zero, Zero) => panic!("This is not a valid direction!"),
        }
    }
    /**
    Ensure that there exists a path from FROM to TO (not including FROM or TO) such that all
    intermediate squares are None.
    Panics if no path exists or FROM == TO
    */
    pub fn clear_path(&self, from: Square, to: Square) -> bool {
        assert_ne!(from, to); // TODO TBD
        match Self::get_path(from, to) {
            Some(path) => {
                // Check each square and ensure it is clear
                for loc in path {
                    if let Some(_) = self.get(loc) {
                        return false;
                    }
                }
                return true;
            }
            None => panic!("TBD"), // TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_rows_are_clear() {
        let board = Board::new();
        assert!(board.clear_path((2, 0), (2, 7)));
        assert!(board.clear_path((3, 0), (3, 7)));
        assert!(board.clear_path((4, 0), (4, 7)));
        assert!(board.clear_path((5, 0), (5, 7)));
    }
    #[test]
    fn occupied_rows_are_not_clear() {
        let board = Board::new();
        assert!(!board.clear_path((0, 0), (0, 7))); // occupied
        assert!(!board.clear_path((1, 0), (1, 7))); // occupied
        assert!(!board.clear_path((6, 0), (6, 7))); // occupied
        assert!(!board.clear_path((7, 0), (7, 7))); // occupied
    }
    #[test]
    fn pawns_have_clear_paths() {
        let board = Board::new();
        assert!(board.clear_path((1, 1), (5, 1))); // from pawn to in front of opposing pawn
        assert!(board.clear_path((1, 1), (6, 1))); // from pawn to opposiing pawn
        assert!(!board.clear_path((0, 1), (4, 1))); // from behind pawn to middle of board
    }
    #[test]
    fn diagonal_increasing_increasing() {
        let board = Board::new();
        // row increasing, column increasing
        assert!(board.clear_path((2, 0), (6, 4)));
        assert!(!board.clear_path((2, 0), (7, 5)));
    }
    #[test]
    fn diagonal_increasing_decreasing() {
        let mut board = Board::new();
        assert!(board.clear_path((1, 6), (6, 1)));
        assert!(!board.clear_path((1, 6), (7, 0)));
        // remove the pawn blocking the queen
        board.set((1, 2), None);
        assert!(board.clear_path((0, 3), (3, 0)));
    }
    #[test]
    fn diagonal_decreasing_increasing() {
        let board = Board::new();
        assert!(board.clear_path((6, 0), (1, 5)));
        assert!(!board.clear_path((2, 4), (0, 6)));
    }
    #[test]
    fn diagonal_decreasing_decreasing() {
        let board = Board::new();
        assert!(board.clear_path((3, 7), (1, 5)));
        assert!(!board.clear_path((7, 7), (4, 4)));
        assert!(!board.clear_path((4, 4), (0, 0)));
    }
    #[test]
    fn diagonal_not_diagonal() {
        assert!(!is_diagonal((2, 7), (0, 6)));
        assert!(!is_diagonal((1, 7), (4, 6)));
        assert!(!is_diagonal((7, 3), (1, 1)));
        assert!(!is_diagonal((2, 7), (6, 7)));
    }
    #[test]
    fn test_diagonal_path() {
        // A) (0,1) -> (2,3)
        assert_eq!(Board::get_path((0, 1), (2, 3)), Some(vec![(1, 2)]));
        // B) (3,2) -> (1,0)
        assert_eq!(Board::get_path((3, 2), (1, 0)), Some(vec![(2, 1)]));
        // C) (3,0) -> (0,3)
        assert_eq!(Board::get_path((3, 0), (0, 3)), Some(vec![(2, 1), (1, 2)]));
        // d) (1,3) -> (3,1)
        assert_eq!(Board::get_path((1, 3), (3, 1)), Some(vec![(2, 2)]));
    }
    #[test]
    fn test_directed_path() {
        let start = (4, 4);
        assert_eq!(
            *Board::get_directed_path(start, (Increasing, Zero))
                .last()
                .unwrap(),
            (7, 4)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Decreasing, Zero))
                .last()
                .unwrap(),
            (0, 4)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Zero, Increasing))
                .last()
                .unwrap(),
            (4, 7)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Zero, Decreasing))
                .last()
                .unwrap(),
            (4, 0)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Increasing, Increasing))
                .last()
                .unwrap(),
            (7, 7)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Decreasing, Decreasing))
                .last()
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Increasing, Decreasing))
                .last()
                .unwrap(),
            (7, 1)
        );
        assert_eq!(
            *Board::get_directed_path(start, (Decreasing, Increasing))
                .last()
                .unwrap(),
            (1, 7)
        );
        // Test some edges
        assert_eq!(
            Board::get_directed_path((7, 4), (Increasing, Zero)).len(),
            0
        );
        assert_eq!(
            Board::get_directed_path((7, 7), (Increasing, Increasing)).len(),
            0
        );
        assert_eq!(
            Board::get_directed_path((0, 0), (Decreasing, Decreasing)).len(),
            0
        );
    }
}
