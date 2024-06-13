mod tictactoe;

use std::io::{self, Read};

use tictactoe::{Board, Move, Player, State};

fn get_move_from_user(b: &Board) -> Move {
    let mut next_move = String::new();
    io::stdin()
        .read_line(&mut next_move)
        .expect("Failed to read line");

    next_move = next_move.trim().to_string();

    let m = b.parse_move(next_move);

    match m {
        Move::Invalid => get_move_from_user(b),
        _ => m,
    }
}

fn get_player() -> Player {
    clear();
    println!("Who begins, X or O?");

    let mut answer: [u8; 1] = [b' '];

    io::stdin()
        .read_exact(&mut answer)
        .expect("Failed to read line");

    match answer.to_ascii_uppercase()[0] {
        b'X' => Player::X,
        b'O' => Player::O,
        _ => get_player(),
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    clear();
    let mut playing_board = Board::new();

    playing_board.current_player = get_player();

    clear();
    println!("{}", &playing_board);
    println!("Enter move for {}", playing_board.current_player);

    loop {
        let next_move = get_move_from_user(&playing_board);

        if next_move == Move::Exit {
            break;
        };

        match playing_board.play_move(next_move) {
            Err(_) => (),
            Ok(state) => match state {
                State::Draw => {
                    clear();
                    println!("{}", &playing_board);
                    println!("It's a draw!");
                    break;
                }
                State::Won(player) => {
                    clear();
                    println!("{}", &playing_board);
                    println!("{} won this game!", player);
                    break;
                }
                State::Playing => (),
            },
        };

        clear();
        println!("{}", &playing_board);
        println!("Enter move for {}", playing_board.current_player);
    }
}
