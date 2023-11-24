use std::io;

struct Player {
    x: String,
    y: String,
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
fn get_player_move(player: &mut Player) {
    println!("player place your x position:");
    io::stdin()
        .read_line(&mut player.x)
        .expect("failed to read input");

    println!("player place your y position:");
    io::stdin()
        .read_line(&mut player.y)
        .expect("failed to read input");
}

fn main() {
    let mut game_moves = 0;
    let board: [[&str; 3]; 3] = [
        [" ", " ", " "],
        [" ", " ", " "],
        [" ", " ", " "]
    ];

    print!("press X to exit game or ctrl-c\n");

    'game_loop: loop {
        print_board(&board);
        // TOD0: change player input to struct for x and y cords
        let mut player1= Player{x: String::new(), y: String::new()};
        let mut player2 = Player{x: String::new(), y: String::new()};

        // get players move
        if game_moves % 2 != 0 {
            get_player_move(&mut player1);
        }else {
            get_player_move(&mut player2);
        }
        
        if player1.x == "X" || player1.y == "X" || player2.x == "X" || player2.y == "X"{
            break 'game_loop;
        }


        game_moves += 1;
    }
    println!("game ended");
}
