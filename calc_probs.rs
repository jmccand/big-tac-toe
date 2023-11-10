use std::fs::File;
use std::io::Write;
use std::env;
use std::collections::HashMap;
use serde_json;

// check if there is a board winner
fn small_winner(board: &[[i8; 3]; 3]) -> i8 {
    // check rows
    for row in 0..3 {
	let mut counts = [0; 2];
	for col in 0..3 {
	    if board[row][col] == 1 {
		counts[0] += 1;
	    }
	    else if board[row][col] == -1 {
		counts[1] += 1;
	    }
	}
	// row is useless if blocked
	if counts[0] == 3 {
	    return 1;
	}
	else if counts[1] == 3 {
	    return -1;
	}
    }
    // check cols
    for col in 0..3 {
	let mut counts = [0; 2];
	for row in 0..3 {
	    if board[row][col] == 1 {
		counts[0] += 1;
	    }
	    else if board[row][col] == -1 {
		counts[1] += 1;
	    }
	}
	// col is useless if blocked
	if counts[0] == 3 {
	    return 1;
	}
	else if counts[1] == 3 {
	    return -1;
	}
    }
    // check diagonals
    {
	let mut counts = [0; 2];
	for i in 0..3 {
	    if board[i][i] == 1 {
		counts[0] += 1;
	    }
	    else if board[i][i] == -1 {
		counts[1] += 1;
	    }
	}
	if counts[0] == 3 {
	    return 1;
	}
	else if counts[1] == 3 {
	    return -1;
	}
    }
    {
	let mut counts = [0; 2];
	for i in 0..3 {
	    if board[i][2-i] == 1 {
		counts[0] += 1;
	    }
	    else if board[i][2-i] == -1 {
		counts[1] += 1;
	    }
	}
	if counts[0] == 3 {
	    return 1;
	}
	else if counts[1] == 3 {
	    return -1;
	}
    }
    return 0;
}

// convert a board to a str representation for easy json storing
fn board_to_str(board: & [[i8; 3]; 3]) -> String {
    // add 1 to avoid negatives
    return format!("{}{}{}{}{}{}{}{}{}",
		   board[0][0]+1, board[0][1]+1, board[0][2]+1,
		   board[1][0]+1, board[1][1]+1, board[1][2]+1,
		   board[2][0]+1, board[2][1]+1, board[2][2]+1)
}

// brute force try every possible board
fn sim_board(boarddb: &mut HashMap<String, (f32, f32)>, ogboard: &mut [[i8; 3]; 3]) -> (f32, f32) {
    // get string representation of this board
    let board_str: String = board_to_str(&ogboard);
    // avoid duplicate calculations
    if boarddb.contains_key(&board_str) {
	return *boarddb.get(&board_str).unwrap();
    }

    if small_winner(ogboard) == -1 {
	// O won
	boarddb.insert(board_str, (0.0, 1.0));
	return (0.0, 1.0);
    }
    else if small_winner(ogboard) == -1 {
	// X won
	boarddb.insert(board_str, (1.0, 0.0));
	return (1.0, 0.0);
    }
    let mut winprobs: (f32, f32) = (0.0, 0.0);
    let mut tmp: (f32, f32);
    let mut nummoves = 0;
    for row in 0..3 {
	for col in 0..3 {
	    if ogboard[row][col] == 0 {
		// x
		ogboard[row][col] = 1;
		tmp = sim_board(boarddb, ogboard);
		winprobs = (winprobs.0 + tmp.0 / 2.0, winprobs.1 + tmp.1 / 2.0);
		// o
		ogboard[row][col] = -1;
		tmp = sim_board(boarddb, ogboard);
		winprobs = (winprobs.0 + tmp.0 / 2.0, winprobs.1 + tmp.1 / 2.0);
		// increase nummoves
		nummoves += 1;
		// reset ogboard
		ogboard[row][col] = 0;
	    }
	}
    }
    if nummoves > 0 {
	// get average probability
	winprobs = (winprobs.0 / (nummoves as f32), winprobs.1 / (nummoves as f32));
    }
    // add to db
    boarddb.insert(board_str, winprobs);
    return winprobs;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    
    // open write file
    let mut saveto = File::create(filename).expect("failed to open file");
    println!("Calculating all board probabilities");
    
    // starting board of all zeros
    let mut startboard: [[i8; 3]; 3] = [[0; 3]; 3];
    let mut boarddb: HashMap<String, (f32, f32)> = HashMap::new();
    sim_board(&mut boarddb, &mut startboard);
    let js = serde_json::to_string(&boarddb);
    // ignore value
    let _ = saveto.write(&js.unwrap().into_bytes());
}
