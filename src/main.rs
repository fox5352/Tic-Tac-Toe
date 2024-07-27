use std::char;
use std::io;

struct Player {
    x: i32,
    y: i32,
    symbol: char,
}

struct PathPos {
    x: usize,
    y: usize,
    char: String,
}

// print game board
fn print_board(board: &[[String; 3]; 3]) {
    for index in 0..board.len() {
        row_printer(&board[index], index);
    }
}
fn row_printer(row: &[String; 3], curr_row: usize) {
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
        if item.len() == 0 {
            line += " ";
        } else {
            line += item;
        }
        line += "|";
    }
    print!("{}\n", line);

    // creates a line then prints it
    for index in 0..line.len() {
        if index == 0 {
            print!(" ");
        } else {
            print!("-");
        }
    }
    print!("\n");
}

// get user input
fn get_player_move() -> i32 {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read input");

    let cord: i32 = buffer.trim().parse().expect("failed to get player input");

    return cord;
}

// check if x and y are in range then checks if the moves ah already been done
fn validate_player_move(player_moves_stack: Vec<(i32, i32, char)>, x: i32, y: i32) -> bool {
    if x > 2 || y > 2 {
        return false;
    } else if x < 0 || y < 0 {
        return false;
    }
    for player_move in player_moves_stack {
        let (pl_x, pl_y, _pl_sy) = player_move;
        if pl_x == x && pl_y == y {
            return false;
        }
    }
    return true;
}

fn update_board(player_moves_stack: Vec<(i32, i32, char)>, board: &mut [[String; 3]; 3]) {
    for board_y in 0..board.len() {
        for board_x in 0..board[board_y].len() {
            for player_move_index in 0..player_moves_stack.len() {
                let (player_x, player_y, player_symbol) =
                    player_moves_stack[player_move_index].clone();

                if board_y as i32 == player_y && board_x as i32 == player_x {
                    board[board_y][board_x].pop();
                    board[board_y][board_x].push(player_symbol);
                }
            }
        }
    }
}

fn pattern_finder(board: &[[String; 3]; 3]) -> bool {
    // list of tuples that the path taken
    let mut path: Vec<PathPos> = Vec::new();

    // loop board y axis
    for index_y in 0..board.len() {
        // get length of row
        let _size: u32 = board[index_y].len() as u32;

        // loop board X axis
        for index_x in 0..board[index_y].len() {
            // the character to match with
            let match_char: String = board[index_y][index_x].clone();

            // loops all 8 directions
            for pattern_type in 0..7 {
                //
                path_finder(
                    pattern_type,
                    &board,
                    &mut path,
                    &match_char,
                    index_x,
                    index_y,
                    2,
                    2,
                );

                if path.len() == 3 {
                    return true;
                } else {
                    // clean path list
                    for item in 0..path.len() {
                        path.pop();
                    }
                }
            }
        }
    }
    return false;
}

fn path_finder(
    direction: i32,
    board: &[[String; 3]; 3],
    path: &mut Vec<PathPos>,
    prev_char: &String,
    curr_x: usize,
    curr_y: usize,
    x_limit: usize,
    y_limit: usize,
) {
    const EMPTY_STR: &str = "";

    if prev_char[..] != *EMPTY_STR {
        if path.len() == 3 {
            return;
        }
        // move up
        else if direction == 0 {
            // if next == current && top wall reached
            if board[curr_y][curr_x] == *prev_char && curr_y == 0 {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                return;
            }
            // if next == current && > top wall
            else if board[curr_y][curr_x] == *prev_char && curr_y > 0 {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                path_finder(
                    direction,
                    &board,
                    path,
                    prev_char,
                    curr_x,
                    curr_y - 1,
                    x_limit,
                    y_limit,
                );
            }
            // if next != current
            else if board[curr_y][curr_x] != *prev_char {
                return;
            }
        }
        // move up-right
        else if direction == 1 {
            // if next == current && top right corner
            if board[curr_y][curr_x] == *prev_char && curr_y == 0 && curr_x == x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                return;
            }
            // if next == current && < top right corner
            else if board[curr_y][curr_x] == *prev_char && curr_y > 0 && curr_x < x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                path_finder(
                    direction,
                    &board,
                    path,
                    prev_char,
                    curr_x + 1,
                    curr_y - 1,
                    x_limit,
                    y_limit,
                );
            }
            // if next != current
            else if board[curr_y][curr_x] != *prev_char {
                return;
            }
        }
        // move right
        else if direction == 2 {
            // if next == current && wall reached
            if board[curr_y][curr_x] == *prev_char && curr_x == x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                return;
            }
            // if next == current && < wall
            else if board[curr_y][curr_x] == *prev_char && curr_x < x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                path_finder(
                    direction,
                    &board,
                    path,
                    prev_char,
                    curr_x + 1,
                    curr_y,
                    x_limit,
                    y_limit,
                );
            }
            // if next != current
            else if board[curr_y][curr_x] != *prev_char {
                return;
            }
        }
        // move down-right
        else if direction == 3 {
            // if next == current && bottom right corner
            if board[curr_y][curr_x] == *prev_char && curr_y == y_limit && curr_x == x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                return;
            }
            // if next == current && < bottom right corner
            else if board[curr_y][curr_x] == *prev_char && curr_y < y_limit && curr_x < x_limit {
                path.push(PathPos {
                    x: curr_x,
                    y: curr_y,
                    char: prev_char.clone(),
                });

                path_finder(
                    direction,
                    &board,
                    path,
                    prev_char,
                    curr_x + 1,
                    curr_y + 1,
                    x_limit,
                    y_limit,
                );
            }
            // if next != current
            else if board[curr_y][curr_x] != *prev_char {
                return;
            }
        }
    }
}

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

        // TOD0: change player input to struct for x and y cords
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
