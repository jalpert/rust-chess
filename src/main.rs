use std::io::{self, BufRead};

mod board;
use crate::board::*;

enum UserInput {
    Loc(Square),
    GoBack,
    Quit,
    Undo,
    Yes,
    Random,
}
use UserInput::*;

fn main() {
    'main: loop {
        let mut board = Board::new();
        let mut history = Vec::new();
        let winner: Color = 'game: loop {
            // Display the current state of the game
            let num_checking = board.num_checking(board.find_king(None).expect("No King Found!"), None);
            let check_mate = num_checking > 0 && board.check_mate(None);
            //
            // Exit game loop if game is over
            if check_mate {
                break 'game board.player().other();
            }
            println!("Turn: {}, {} to move.", board.turn(), board.player());
            if num_checking > 0 {
                println!(
                    "{}'s king is in check by {} opposing pieces.",
                    board.player(),
                    num_checking
                );
            };
            println!("{}", board);
            println!("Select a piece to move by specifying the row then the column, separated by whitespace. Then press enter:");
            //
            // Get the piece the current player wants to move
            let from: Square = 'validate_from: loop {
                let err_msg: String = match parse_input() {
                    Some(Loc(from)) => match board.validate_from(from, None) {
                        Ok(()) => break 'validate_from from,
                        Err(msg) => msg,
                    },
                    Some(GoBack) => continue 'validate_from,
                    Some(Quit) => break 'main,
                    Some(Undo) => {
                        board = history.pop().unwrap_or(board);
                        continue 'game;
                    }
                    Some(Random) => {
                        history.push(board);
                        let (from, to) = Board::random_move(&board);
                        println!("Moving {} to {} {}", board.get(from).unwrap(), to.0+1, to.1+1);
                        board = board.execute_move(from, to);
                        continue 'game
                    },
                    Some(Yes) | None => String::from("Input not received in proper format."),
                };
                println!("{} Try again please:", err_msg);
            };

            // Get the square the current player wants to move to
            let to: Square = 'validate_move: loop {
                println!("Enter the square to which you would like to move this piece:");
                let err_msg = match parse_input() {
                    Some(Loc(to)) => match board.validate_move(from, to, None) {
                        Ok(()) => break 'validate_move to,
                        Err(msg) => msg,
                    },
                    Some(GoBack) => continue 'game,
                    Some(Quit) => break 'main,
                    Some(Undo) => {
                        board = history.pop().unwrap_or(board);
                        continue 'game;
                    }
                    Some(Random) => {
                        history.push(board);
                        let (from, to) = Board::random_move(&board);
                        println!("Moving {} to {} {}", board.get(from).unwrap(), to.0+1, to.1+1);
                        board = board.execute_move(from, to);
                        continue 'game
                    },
                    Some(Yes) | None => String::from("Input not received in proper format."),
                };
                println!("{} Try again please:", err_msg);
            };
            //
            // Execute the move
            history.push(board);
            board = board.execute_move(from, to);
            
            // Make some space before the next move
            println!("\n\n");
        };
        println!("{} wins!\n{}", winner, board);
        loop {
            println!("Play Again? Enter Yes (Y) or Quit (Q)");
            match parse_input() {
                Some(Yes) => continue 'main,
                Some(Quit) => break 'main,
                _ => continue,
            }
        }
    }
    println!("Thanks for playing. Bye bye now!");
}

// Extract the row and column from a string containing two integers separated by whitespace
// type 'r' to take a random turn
// type 'b' to go back one step
// type 'q' to quit the game
fn parse_input() -> Option<UserInput> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .lock()
        .read_line(&mut buffer) // Read a line from stdin
        .expect("Stdin not working properly.");
    //
    let buffer = buffer.trim(); // Trim whitespace
    if buffer == "r" {
        Some(Random)
    } else if buffer == "b" || buffer == "B" {
        Some(GoBack)
    } else if buffer == "q" || buffer == "Q" {
        Some(Quit)
    } else if buffer == "Yes" || buffer == "yes" || buffer == "Y" || buffer == "y" {
        Some(Yes)
    } else if buffer == "u" || buffer == "U" {
        Some(Undo)
    } else {
        let mut iter = buffer.split_whitespace();
        let row: Option<isize> = iter.next().map(|row_str| row_str.parse().ok()).flatten();
        let col: Option<isize> = iter.next().map(|col_str| col_str.parse().ok()).flatten();
        if let (Some(row), Some(col), None) = (row, col, iter.next()) {
            // Subtract 1 to zero-index
            Some(Loc((row - 1, col - 1)))
        } else {
            None
        }
    }
}
