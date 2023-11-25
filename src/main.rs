use std::io;

struct Player {
    x: i32,
    y: i32,
    symbol: char
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
fn get_player_move() -> i32{
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer).expect("failed to read input");

    let cord: i32 = buffer.trim().parse().expect("failed to get player input");

    return cord;
}

// check if x and y are in range then checks if the moves ah already been done
fn validate_player_move(player_moves_stack: Vec<(i32, i32, char)>, x: i32, y: i32) -> bool {
    if x > 2 || y > 2 {
        return false;
    }else if x < 0 || y < 0 {
        return false;
    }
    for player_move in player_moves_stack {
        let (pl_x, pl_y, _pl_sy) = player_move;
        if pl_x == x && pl_y == y{
            return false
        }
    }
    return true;
}

fn main() {
    let mut game_moves: i32 = 0;
    let mut player_moves_stack: Vec<(i32, i32, char)> = vec![];
    let board: [[&str; 3]; 3] = [
        [" ", " ", " "],
        [" ", " ", " "],
        [" ", " ", " "]
    ];

    print!("press ctrl-c to exit game\n");
    'game_loop: loop {
        print_board(&board);
        // TOD0: change player input to struct for x and y cords
        let mut player1: Player = Player{x: 0, y: 0, symbol: 'X'};
        let mut player2: Player = Player{x: 0, y: 0, symbol: 'O'};

        // get players moves in turns and check to see if the move is valid
        if game_moves % 2 == 0 {
            'player1_move: loop{
                println!("player 1");
            
                println!("place your x position:");
                player1.x = get_player_move();

                println!("place your y position:");
                player1.y = get_player_move();

                let valid_input: bool = validate_player_move(player_moves_stack.clone(), player1.x, player1.y);

                if valid_input {
                    player_moves_stack.push((player1.x, player1.y, player1.symbol));
                    break 'player1_move;   
                }
            }
        }else {
            'player2_move: loop {
                println!("player 2");

                println!("place your x position:");
                player2.x = get_player_move();
                
                println!("place your y position:");
                player2.y = get_player_move();
                
                let valid_input: bool = validate_player_move(player_moves_stack.clone(), player2.x, player2.y);

                if valid_input {
                    player_moves_stack.push((player2.x, player2.y, player2.symbol));
                    break 'player2_move;   
                }
            }
        }
        
        let _player_list: [Player; 2] = [player1, player2];


        // TODO: clear players moves then update board and the the loop cycle

        // max moves reached
        if game_moves >= 9{
            break 'game_loop;
        }
        game_moves += 1;
    }
    println!("game ended");
}
