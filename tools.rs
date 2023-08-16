use std::env;
use std::fs;
mod main;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let rawboard = parse_board(contents);
    println!("{:?}", board);
}

fn parse_board(contents: String) -> [[i8; 9]; 9] {
    let mut thisindex = 0;
    let mut thischar;
    let mut negative = 1;
    let mut board: [[i8; 9]; 9] = [[0; 9]; 9];
    let mut curindex: usize = 0;
    let mut updateBoard = |number: i8, neg: i8| {
	board[curindex / 9][curindex % 9] = neg * number;
	curindex += 1;
    };
    while thisindex < contents.len() {
	thischar = contents.chars().nth(thisindex).unwrap();
	match thischar {
	    '-' => negative = -1,
	    '0' => {updateBoard(0, negative); negative = 1},
	    '1' => {updateBoard(1, negative); negative = 1},
	    '2' => {updateBoard(2, negative); negative = 1},
	    _ => (),
	}
	thisindex += 1;
    }
    return board;
}

fn simulate(
