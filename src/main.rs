mod tictactoe;

use std::io::{ self, Read };

use tictactoe::{ Player, Board, State };

fn main() {
    let mut playing_board = Board::new();
    
    println!("Who begins, X or O?");
    
    let mut answer: [u8; 1] = [b' '];

    io::stdin()
        .read_exact(&mut answer)
        .expect("Failed to read line");

    match answer {
        [b'X'] => playing_board.current_player = Player::X,
        [b'O'] => playing_board.current_player = Player::O,
        _ => (),
    };


    loop {
        print!("{}", &playing_board);
        println!("Enter move for {}", playing_board.current_player);

        let mut next_move = String::new();

        io::stdin()
            .read_line(&mut next_move)
            .expect("Failed to read line");

        if next_move.trim() == "exit" {
            break;
        };

        match playing_board.play_move(&next_move.trim()) {
            Err(_) => (),
            Ok(state) => match state {
                State::Draw => {
                    println!("{}", playing_board);
                    println!("It's a draw!");
                    break;
                },
                State::Won(player) => {
                    println!("{}", playing_board);
                    println!("{} won this game!", player);
                    break;
                },
                State::Playing => (),
            },
        };
    };
}
