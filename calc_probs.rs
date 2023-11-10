use std::fs::File;
use std::io::Write;
use std::env;
use std::collections::HashMap;
use serde_json;

// convert a board to a str representation for easy json storing
fn boardToStr(board: & [[i8; 3]; 3]) -> String {
    return format!("{}{}{}{}{}{}{}{}{}",
		   board[0][0], board[0][1], board[0][2],
		   board[1][0], board[1][1], board[1][2],
		   board[2][0], board[2][1], board[2][2])
}

// brute force try every possible board
fn simBoard(boarddb: &mut HashMap<String, (f32, f32)>, mut ogboard: [[i8; 3]; 3]) -> (f32, f32) {
    // get string representation of this board
    let boardStr: String = boardToStr(&ogboard);
    // avoid duplicate calculations
    if boarddb.contains_key(&boardStr) {
	return *boarddb.get(&boardStr).unwrap();
    }
    let mut winprobs: (f32, f32) = (0.0, 0.0);
    let mut tmp: (f32, f32);
    let mut nummoves = 0;
    for row in 0..3 {
	for col in 0..3 {
	    if ogboard[row][col] == 0 {
		// x
		ogboard[row][col] = 1;
		tmp = simBoard(boarddb, ogboard);
		winprobs = (winprobs.0 + tmp.0, winprobs.1 + tmp.1);
		// o
		ogboard[row][col] = -1;
		tmp = simBoard(boarddb, ogboard);
		winprobs = (winprobs.0 + tmp.0, winprobs.1 + tmp.1);
		nummoves += 1;
		// reset ogboard
		ogboard[row][col] = 0;
	    }
	}
    }
    // get average probability
    winprobs = (winprobs.0 / nummoves as f32, winprobs.1 / nummoves as f32);
    // add to db
    boarddb.insert(boardStr, winprobs);
    return winprobs;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    
    // open write file
    let mut saveto = File::create(filename).expect("failed to open file");
    println!("Calculating all board probabilities");
    
    // starting board of all zeros
    let startboard: [[i8; 3]; 3] = [[0; 3]; 3];
    let mut boarddb: HashMap<String, (f32, f32)> = HashMap::new();
    simBoard(&mut boarddb, startboard);
    let js = serde_json::to_string(&boarddb);
    // ignore value
    let _ = saveto.write(&js.unwrap().into_bytes());
}
