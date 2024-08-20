use std::char;

use tic_tac_toe::{
    player::{
        get_player_move,
        Player
    },
    board::print_board,
    game::{
        validate_player_move,
        pattern_finder,
        update_board
    }
};



fn main() {
    let mut game_moves: i32 = 0;
    let mut player_moves_stack: Vec<(i32, i32, char)> = vec![];

    let mut board: [[String; 3]; 3] = [
        [String::from(""), String::from(""), String::from("")],
        [String::from(""), String::from(""), String::from("")],
        [String::from(""), String::from(""), String::from("")],
    ];

    print!("press ctrl-c to exit game\n");
    'game_loop: loop {
        // reprints board
        print_board(&board);

        // win checker
        let player_win = pattern_finder(&board);

        if player_win {
            break 'game_loop;
        }

        let mut player1: Player = Player {
            x: 0,
            y: 0,
            symbol: 'X',
        };
        let mut player2: Player = Player {
            x: 0,
            y: 0,
            symbol: 'O',
        };

        // get players moves in turns and check to see if the move is valid
        if game_moves % 2 == 0 {
            'player1_move: loop {
                println!("player 1");

                println!("place your x position:");
                player1.x = get_player_move();

                println!("place your y position:");
                player1.y = get_player_move();

                let valid_input: bool =
                    validate_player_move(player_moves_stack.clone(), player1.x, player1.y);

                if valid_input {
                    player_moves_stack.push((player1.x, player1.y, player1.symbol));
                    break 'player1_move;
                }
                print!("invalid move try again:")
            }
        } else {
            'player2_move: loop {
                println!("player 2");

                println!("place your x position:");
                player2.x = get_player_move();

                println!("place your y position:");
                player2.y = get_player_move();

                let valid_input: bool =
                    validate_player_move(player_moves_stack.clone(), player2.x, player2.y);

                if valid_input {
                    player_moves_stack.push((player2.x, player2.y, player2.symbol));
                    break 'player2_move;
                }
                print!("invalid move try again:")
            }
        }

        update_board(player_moves_stack.clone(), &mut board);

        // max moves reached
        if game_moves >= 9 {
            break 'game_loop;
        }
        game_moves += 1;
    }
    println!("game ended");
}
