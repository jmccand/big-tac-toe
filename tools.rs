use std::env;
use std::fs;
use std::process::Command;
mod main;


fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let rawboard = parse_board(contents);
    let starter = main::Board {
	brd: rawboard,
	scope: 1,
	player: -1,
	movenum: 0,
	parent: None,
	children: Vec::new(),
	prediction: Some(main::rate_board(rawboard)),
    };
    main::print_board(&starter);
    println!("original board rating: {}", main::rate_board(rawboard));
    unsafe{main::DB.push(starter);}
    println!("building tree");
    main::buildtree();
    loop {
	let mut child = Command::new("sleep").arg("1").spawn().unwrap();
	let _result = child.wait().unwrap();
	if unsafe{main::DB[main::DB.len() - 1].movenum} > 6 {
	    break;
	}
    }
    println!("computer move: {}", main::getcpmove(unsafe{&mut main::DB}, 0, Some(4)));
    show_comparison(unsafe{&mut main::DB}, 0, 4);
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

fn show_comparison(mut db: &mut Vec<main::Board>, curindex: usize, depth: u8) {
    if db[curindex].movenum == depth {
	main::print_board(&db[curindex]);
	println!("Rating: {}", main::rate_board(db[curindex].brd));
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
	show_comparison(db, thisboard.children[predchild], depth);
    }
}
