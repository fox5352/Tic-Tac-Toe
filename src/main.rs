use std::io;

struct Player {
    x: String,
    y: String,
    symbol: String
}

// print game board
fn print_board(board: &[[&str; 3]; 3]) {
    for index in 0..board.len() {
        row_printer(&board[index], index);
    }
}
fn row_printer(row: &[&str; 3], curr_row: usize) {
    let mut line = String::new();

    // print column grind before first row
    if curr_row == 0 {
        print!(" |0|1|2|\n");
        print!(" -------\n");
    }

    // adds row index and wall
    line = line + &curr_row.to_string() + "|";

    // creates a row of the grid then prints it
    for item in row {
        line += item;
        line += "|";
    }
    print!("{}\n", line);


    // creates a line then prints it
    for index in 0..line.len() {
        if index == 0 {
            print!(" ");
        }else{
            print!("-");
        }
    }
    print!("\n");

}

// get user input
fn get_player_move(player: &mut String) {
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer).expect("failed to read input");
    *player = buffer.trim().to_string();
}

fn validate_player_move(player_moves_stack: Vec<(String, String, String)>, x: String, y: String) -> bool {
    for player_move in player_moves_stack {
        let (pl_x, pl_y, _pl_sy) = player_move;
        if pl_x == x || pl_y == y {
            return false
        }
    }
    return true;
}

fn main() {
    let mut game_moves: i32 = 0;
    let player_moves_stack: Vec<(String, String, String)> = vec![];
    let board: [[&str; 3]; 3] = [
        [" ", " ", " "],
        [" ", " ", " "],
        [" ", " ", " "]
    ];

    print!("press ctrl-c to exit game\n");
    'game_loop: loop {
        print_board(&board);
        // TOD0: change player input to struct for x and y cords
        let mut player1= Player{x: String::new(), y: String::new(), symbol: String::from("X")};
        let mut player2 = Player{x: String::new(), y: String::new(), symbol: String::from("O")};

        // get players moves in turns and check to see if the move is valid
        if game_moves % 2 == 0 {
            'player1_move: loop{
                println!("player 1");
            
                println!("place your x position:");
                get_player_move(&mut player1.x);

                println!("place your y position:");
                get_player_move(&mut player1.y);

                if validate_player_move(player_moves_stack.clone(), player1.x.clone(), player1.y.clone()) {
                    break 'player1_move;   
                }
            }
        }else {
            'player2_move: loop {
                println!("player 2");

                println!("place your x position:");
                get_player_move(&mut player2.x);
                
                println!("place your y position:");
                get_player_move(&mut player2.y);
                
                if validate_player_move(player_moves_stack.clone(), player2.x.clone(), player2.y.clone()) {
                    break 'player2_move;   
                }
            }
        }
        
        let player_list: [Player; 2] = [player1, player2];

        // TODO: clear players moves then update board and the the loop cycle
        
        for player in player_list {
            if player.x.len() > 0 {
                println!("player x:{}", player.x);
                println!("player y:{}", player.y);
            }
        }

        // max moves reached
        if game_moves >= 9{
            break 'game_loop;
        }
        game_moves += 1;
    }
    println!("game ended");
}
