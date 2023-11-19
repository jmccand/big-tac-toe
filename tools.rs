use std::env;
use std::fs;
use std::process::Command;
mod main;


fn main() {
    let args: Vec<String> = env::args().collect();
    let lookahead = args[3].parse().unwrap();
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let rawboard = parse_board(contents);
    let starter = main::Board {
	brd: rawboard,
	scope: args[2].parse::<u8>().unwrap(),
	player: -1,
	movenum: 0,
	parent: None,
	children: Vec::new(),
	prediction: Some(main::rate_board(rawboard).0 * (1.0 - main::rate_board(rawboard).1)),
    };
    main::print_board(&starter);
    println!("original board rating: h: {} c: {}", main::rate_board(rawboard).0, main::rate_board(rawboard).1);
    unsafe{main::DB.push(starter);}
    println!("building tree");
    main::buildtree();
    loop {
	let mut child = Command::new("sleep").arg("1").spawn().unwrap();
	let _result = child.wait().unwrap();
	if unsafe{main::DB[main::DB.len() - 1].movenum} > lookahead {
	    break;
	}
    }
    println!("computer move: {}", main::getcpmove(unsafe{&mut main::DB}, 0, Some(lookahead)));
    show_comparison(unsafe{&mut main::DB}, 0, lookahead);
}

fn parse_board(contents: String) -> [[i8; 9]; 9] {
    let mut thisindex = 0;
    let mut thischar;
    let mut negative = 1;
    let mut board: [[i8; 9]; 9] = [[0; 9]; 9];
    let mut curindex: usize = 0;
    let mut update_board = |number: i8, neg: i8| {
	board[curindex / 9][curindex % 9] = neg * number;
	curindex += 1;
    };
    while thisindex < contents.len() {
	thischar = contents.chars().nth(thisindex).unwrap();
	match thischar {
	    '-' => negative = -1,
	    '0' => {update_board(0, negative); negative = 1},
	    '1' => {update_board(1, negative); negative = 1},
	    '2' => {update_board(2, negative); negative = 1},
	    _ => (),
	}
	thisindex += 1;
    }
    return board;
}

fn show_comparison(db: &mut Vec<main::Board>, curindex: usize, depth: u8) {
    if db[curindex].movenum == depth {
	main::print_board(&db[curindex]);
	println!("Rating: h: {}, c: {}", main::rate_board(db[curindex].brd).0, main::rate_board(db[curindex].brd).1);
	return;
    }
    else if db[curindex].movenum == 0 {
	let thisboard = db[curindex].clone();
	for child in 0..thisboard.children.len() {
	    show_comparison(db, thisboard.children[child], depth);
	}
    }
    else {
	let thisboard = db[curindex].clone();
	let predchild = main::updatepred(&mut *db, curindex);
	if thisboard.children.len() > 0 {
	    show_comparison(db, thisboard.children[predchild], depth);
	}
    }
}

// print a board's ratings (for debugging)
fn print_ratings(board: [[i8; 9]; 9]) {
    // calculate probabilities
    let mut scope_probs: [[(f32, f32); 3]; 3] = [[(0.0, 0.0); 3]; 3];
    for scope in 0..9 {
	let brd_str = main::board_to_str(&main::get_slice(board, scope));
	let swinner = main::small_winner(main::get_slice(board, scope));
	if swinner == -1 {
	    scope_probs[scope as usize / 3 as usize][scope as usize % 3 as usize] = (0.0, 1.0);
	}
	else if swinner == 1 {
	    scope_probs[scope as usize / 3 as usize][scope as usize % 3 as usize] = (1.0, 0.0);
	}
	else {
	    scope_probs[scope as usize / 3 as usize][scope as usize % 3 as usize] = *main::PROBS.get(&brd_str).unwrap();
	}
    }
    println!();
    for row in 0..3 {
	if row != 0 {
	    println!("{}", "-".repeat(23));
	}
	println!("       |       |       ");
	for col in 0..3 {
	    print!(" ");
	    let val = scope_probs[row][col].0;
	    print!("{:.3}", val);
	    print!(" |");
	}
	println!();
	for col in 0..3 {
	    print!(" ");
	    let val = scope_probs[row][col].1;
	    print!("{:.3}", val);
	    print!(" |");
	}
	println!();
    }
    println!();
    // rate big board
    // rows
    for row in 0..3 {
	let mut row_prob: (f32, f32) = (1.0, 1.0);
	for col in 0..3 {
	    row_prob = (row_prob.0 * scope_probs[row][col].0, row_prob.1 * scope_probs[row][col].1);
	}
	println!("row {}: h: {:.3}  c: {:.3}", row, row_prob.0, row_prob.1);
    }
    // cols
    for col in 0..3 {
	let mut col_prob: (f32, f32) = (1.0, 1.0);
	for row in 0..3 {
	    col_prob = (col_prob.0 * scope_probs[row][col].0, col_prob.1 * scope_probs[row][col].1);
	}
	println!("col {}: h: {:.3}  c: {:.3}", col, col_prob.0, col_prob.1);
    }
    // diagonal 1
    {
	let mut diag_prob: (f32, f32) = (1.0, 1.0);
	for diag in 0..3 {
	    diag_prob = (diag_prob.0 * scope_probs[diag][diag].0, diag_prob.1 * scope_probs[diag][diag].1);
	}
	println!("diag 1: h: {:.3}  c: {:.3}", diag_prob.0, diag_prob.1);
    }
    // diagonal 2
    {
	let mut diag_prob: (f32, f32) = (1.0, 1.0);
	for diag in 0..3 {
	    diag_prob = (diag_prob.0 * scope_probs[2 - diag][diag].0, diag_prob.1 * scope_probs[2 - diag][diag].1);
	}
	println!("diag 2: h: {:.3}  c: {:.3}", diag_prob.0, diag_prob.1);
    }
}
