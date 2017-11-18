extern crate tictactoe_lib;

use tictactoe_lib::model::Game;
use tictactoe_lib::model::Position;
use tictactoe_lib::model::Player;
use tictactoe_lib::model::GameResult;
use tictactoe_lib::model::BOARD_SIZE;

fn main() {
    let mut game = Game::new();
    let mut board_view = [[' '; BOARD_SIZE]; BOARD_SIZE];

    loop {
        print_board(&board_view);

        println!("The next player is {}.", player_symbol(game.turn_player()));
        println!("Choose a square [row column]:");
        let mut position = String::new();
        std::io::stdin().read_line(&mut position).expect("Failed to read line");
        let mut position = position.trim().split_whitespace().map(|p| p.parse());
        let position: Position = match (position.next(), position.next(), position.next()) {
            (Some(Ok(r)), Some(Ok(c)), None) => match Position::new(r, c) {
                Ok(p) => p,
                Err(s) => {
                    println!("{}", s);
                    continue;
                },
            },
            _ => { 
                println!("Cannot parse position.");
                continue;
            },
        };

        match game.play(position) {
            Some(p) => {
                board_view[position.row()][position.column()] = player_symbol(p);
                println!("{} played at ({}, {}).", player_symbol(p), position.row(), position.column());

                match game.result(position) {
                    Some(GameResult::Draw) => {
                        print_board(&board_view);
                        println!("The game is a draw.");
                        break;
                    }
                    Some(r) => {
                        print_board(&board_view);
                        println!("{} wins.", player_symbol(r.win_player().unwrap()));
                        break;
                    }
                    None => (),
                }
            }
            None => println!("The square is already occupied."),
        }
    }
}

fn print_board(board_view: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    println!("Current board state:");
        print!("+");
        for _ in 0..BOARD_SIZE {
            print!("---+");
        }
        println!("");
    for i in 0..BOARD_SIZE {
        print!("|");
        for j in 0..BOARD_SIZE {
            print!(" {} |", board_view[i][j]);
        }
        println!("");
        print!("+");
        for _ in 0..BOARD_SIZE {
            print!("---+");
        }
        println!("");
    }
}

fn player_symbol(player: Player) -> char {
    match player {
        Player::P1 => 'X',
        Player::P2 => 'O',
    }
}
