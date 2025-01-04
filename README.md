# Tic-Tac-Toe CLI Game

A simple two-player Tic-Tac-Toe game built in Rust with a command-line interface. The game alternates between Player 1 and Player 2, allowing them to make moves on a 3x3 grid. The game ends when a player wins or when all moves are used up (a draw).

## Features

- **Grid Display**: The game board is printed with indices for the rows and columns, allowing players to easily identify where to place their symbol.
- **Player Turns**: Player 1 uses "X" and Player 2 uses "O". The game alternates between the two players' moves.
- **Input Validation**: Players are prompted to place their symbol on a valid, empty space on the board.
- **Win Detection**: The game checks for a winning pattern after every move.
- **Game Loop**: The game continues until there is a winner or the board is full (draw).

## Setup

To run the game, you will need Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

### Clone the Repository

```bash
git clone https://github.com/your-username/tic-tac-toe-cli.git
cd tic-tac-toe-cli
```

### Build the Application

Run the following command to build the project:

```bash
cargo build --release
```

### Running the Application

Once the build is complete, you can run the game:

```bash
cargo run
```

Alternatively, if you have an `.exe` release, you can run the executable directly:

```bash
tic_tac_toe_cli.exe
```

## Gameplay

1. The game will display a 3x3 grid with empty spaces.
2. Player 1 will be prompted to input their move for the "X" symbol, and Player 2 will input their move for the "O" symbol.
3. The players take turns to place their symbol on the board.
4. After each move, the board is updated and displayed.
5. The game ends when one player wins or when all spaces are filled (draw).

## Code Structure

The project is organized into three main modules:

### `board`
Contains functions for displaying the board and printing each row with separators.

### `player`
Contains logic for getting player input and defining the `Player` struct, which holds player positions and symbols.

### `game`
Contains the game logic, including move validation, updating the board, and detecting winning patterns.

## Example

### Game Flow:

```
press ctrl-c to exit game
 |0|1|2|
 -------
0| | | |
-------
1| | | |
-------
2| | | |
player 1
place your x position:
0
place your y position:
0
 |0|1|2|
 -------
0|X| | |
-------
1| | | |
-------
2| | | |
player 2
place your x position:
1
place your y position:
1
 |0|1|2|
 -------
0|X| | |
-------
1| |O| |
-------
2| | | |
...
```
