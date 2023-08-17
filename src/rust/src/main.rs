use crate::game::{Game, Cell};
use std::io;

pub mod game;

enum PlayerType {
    Human,
    Ai,
}

fn choose_player_type(player_number: u8) -> PlayerType {
    let mut number_chosen;

    loop {
        println!("\nChoose a type for Player {} (1: Human or 2: AI):", player_number);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Please enter something.");
                continue
            }
        }
        match input.replace("\n", "").parse() {
            Ok(value) => number_chosen = value,
            Err(_) => {
                println!("Please enter a number.");
                continue
            }
        }

        if number_chosen == 1 || number_chosen == 2 { 
            break; 
        } else {
            println!("Please enter a number that is 1 (Human) or 2 (AI).\n")
        }
    }

    match number_chosen {
        1 => return PlayerType::Human,
        2 => return PlayerType::Ai,
        _ => panic!("number_chosen is different from 1 or 2")
    }
}

fn ask_position(player_number: usize) -> (usize, usize) {
    println!("\n-- Player {} to play! --", player_number);
    let mut x: usize;
    loop {
        println!("Choose a column to play (between 1 and 3):");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Please enter something.");
                continue
            }
        }
        match input.replace("\n", "").parse() {
            Ok(value) => x = value,
            Err(_) => {
                println!("Please enter a number.");
                continue
            }
        }

        if x == 1 || x == 2 || x == 3 { 
            break; 
        } else {
            println!("Please enter a number that is between 1 and 3.\n");
        }
    }

    let mut y: usize;
    loop {
        println!("Choose a column to play (between 1 and 3):");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Please enter a number.");
                continue
            }
        }
        match input.replace("\n", "").parse() {
            Ok(value) => y = value,
            Err(_) => {
                println!("Please enter a number.");
                continue
            }
        }

        if y == 1 || y == 2 || y == 3 { 
            break; 
        } else {
            println!("Please enter a number that is between 1 and 3.\n");
        }
    }

    (x-1, y-1) // start indexing the array at 0
}

fn main() {
    let mut tictactoe = Game::new();
    println!(r"
 _____  ___   ___       _____  ___   ___       _____   ___   ___ 
|_   _||_ _| / __|     |_   _|/   \ / __|     |_   _| / _ \ | __|
  | |   | | | (__        | |  | - || (__        | |  | (_) || _| 
  |_|  |___| \___|       |_|  |_|_| \___|       |_|   \___/ |___|
");

    let players = [choose_player_type(1), choose_player_type(2)];
    let mut winning = Cell::Empty;
    let mut turn: usize = 0;

    while winning == Cell::Empty && turn < 9 {
        println!("\n{}\n", tictactoe);

        match players[turn % 2] {
            PlayerType::Ai => {
                panic!("AI not implemented yet.")
            },
            PlayerType::Human => {
                loop {
                    let (x, y) = ask_position(turn%2 + 1);
                    match tictactoe.play(x, y, {
                        if turn % 2 == 0 { Cell::Circle } 
                        else { Cell::Cross }
                    }) {
                        Ok(()) => break,
                        Err(()) => println!("The cell ({x}, {y}) is occupied. Please choose an empty cell.")
                    }
                }
            }
        }

        winning = tictactoe.winning_position();
        turn += 1;
    }

    println!("{}", tictactoe);
    if turn == 9 {
        println!("\n--- DRAW! ---");
    } else {
        println!("\n--- PLAYER {} WON! ---", turn % 2 + 1);
    }
}
