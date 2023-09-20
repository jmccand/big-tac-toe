use std::io::{stdin,stdout,Write};
use std::fs::File;
use std::vec::Vec;
use std::thread;

#[derive(Clone, Debug)]
pub struct Board {
    pub brd: [[i8; 9]; 9],
    pub scope: u8,
    pub player: i8,
    pub movenum: u8,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub prediction: Option<f32>,
}

pub static mut DB: Vec<Board> = Vec::new();
pub static mut CURINDEX: usize = 0;

impl Board {
    fn obselete(&self, db: &Vec<Board>, curindex: usize) -> bool {
	let ogboard = &db[curindex];
	let mut thisboard = self;
	while thisboard.movenum > ogboard.movenum {
	    thisboard = &db[thisboard.parent.unwrap()];
	}
	return thisboard.brd != ogboard.brd;
    }
}

pub fn updatepred(db: &mut Vec<Board>, curindex: usize) -> usize {
    let cpboard = db[curindex].clone();
    if cpboard.children.len() > 0 {
	if cpboard.player == 1 {
	    // get max rating from children
	    let mut maxrate: Option<f32> = None;
	    let mut maxindex: usize = 0;
	    for i in 0..cpboard.children.len() {
		if maxrate == None || db[cpboard.children[i]].prediction > maxrate {
		    maxrate = db[cpboard.children[i]].prediction;
		    maxindex = i as usize;
		}
	    }
	    db[curindex].prediction = maxrate;
	    return maxindex;
	}
	else {
	    // get min rating from children
	    let mut minrate: Option<f32> = None;
	    let mut minindex: usize = 0;
	    let onwon = small_winner(get_slice(cpboard.brd, cpboard.scope)) != 0;
	    let mut all_loop = true;
	    for i in 0..cpboard.children.len() {
		let child = &db[cpboard.children[i]];
		let nextwon = small_winner(get_slice(child.brd, child.scope)) != 0;
		if !(onwon && loopstuck(&db, curindex, i) && (!nextwon)) {
		    all_loop = false;
		    break;
		}
	    }
	    for i in 0..cpboard.children.len() {
		let child = &db[cpboard.children[i]];
		let nextwon = small_winner(get_slice(child.brd, child.scope)) != 0;
		if (all_loop || !(onwon && loopstuck(&db, curindex, i) && (!nextwon))) && (minrate == None || child.prediction > minrate) {
		    minrate = db[cpboard.children[i]].prediction;
		    minindex = i as usize;
		}
	    }
	    db[curindex].prediction = minrate;
	    return minindex;
	}
    }
    return 0;
}

fn write_board(board: [[i8; 9]; 9], outfile: &mut File) {
    outfile.write(b"\n[").expect("failed to write file");
    for row in 0..9 {
	outfile.write(b"[").expect("failed to write file");
	for col in 0..9 {
	    outfile.write(
		match board[row][col] {
		    -2 => b"-2",
		    -1 => b"-1",
		    0 => b"0",
		    1 => b"1",
		    2 => b"2",
		    _ => b".",
		}
	    ).expect("failed to write file");
	    outfile.write(b", ").expect("failed to write file");
	}
	outfile.write(b"],\n").expect("failed to write file");
    }
    outfile.write(b"]").expect("failed to write file");
}

fn main() {
    print!("Welcome to Big Tac Toe! Enter the number of players: ");
    let s = input();
    if s == "1" {
	let starter = Board {
	    brd: [[0; 9]; 9],
	    scope: 1,
	    player: 1,
	    movenum: 0,
	    children: Vec::new(),
	    parent: None,
	    prediction: None,
	};
	println!("Welcome to 1 player Big Tac Toe!");
	play(starter);
    }
    else if s == "2" {
	println!("Welcome to 2 player Big Tac Toe!");
	twoplayer();
    }
}

pub fn play(starter: Board) {
    let mut game_history = File::create("game_history.txt").expect("failed to open file");
    // thread that builds the decision tree
    unsafe {DB.push(starter);}
    buildtree();
    // thread that takes user input and gets best computer move
    while winner(unsafe {DB[CURINDEX].brd}) == 0 {
	let board = unsafe{&DB[CURINDEX].clone()};
	// println!("This board has {} children", board.children.len());
	write_board(board.brd, &mut game_history);
	if board.player == 1 {
	    // println!("Board rating: {}", rate_board(board));
	    print_board(&board);
	    println!("Board rating: {}. Computer can see {} moves ahead.", rate_board(board.brd), unsafe{DB.last().unwrap().movenum - DB[CURINDEX].movenum});
	    print!("You are on board number {}. Please enter a number, 0-8 (inclusive) for where you want to place your X: ", board.scope);
	    let up = input();
	    let truep = up.parse::<u8>().unwrap();
	    let board = unsafe{&DB[CURINDEX].clone()};
	    if truep < 9 && get(board.brd, board.scope, truep) == 0 {
		unsafe{CURINDEX = domove(board, truep);}
	    }
	}
	else {
	    unsafe{CURINDEX = domove(board, getcpmove(&mut DB, CURINDEX, Some(DB.last().unwrap().movenum - 1)));}
	}
    }
    print_board(unsafe{&DB[CURINDEX]});
    if winner(unsafe{DB[CURINDEX].brd}) == 1 {
	print!("You ");
    }
    else {
	print!("The computer ");
    }
    println!("won. Thank you for playing!");
}

pub fn buildtree() {
    thread::spawn(|| {
	fn tryall(database: &mut Vec<Board>, index: usize) {
	    if winner(database[index].brd) == 0 {
		let myslice = get_slice(database[index].brd, database[index].scope);
		if is_full(myslice) {
		    for row in 0..9 {
			for col in 0..9 {
			    if database[index].brd[row][col] == 0 {
				let p: u8 = ((row % 3) * 3 + col % 3) as u8;
				let mut newbrd = Board {
				    brd: database[index].brd.clone(),
				    scope: p,
				    player: database[index].player * -1,
				    movenum: database[index].movenum + 1,
				    children: Vec::new(),
				    parent: Some(index),
				    prediction: None,
				};
				place(&mut newbrd.brd, database[index].player, ((row / 3) * 3 + (col / 3)) as u8, p);
				newbrd.prediction = Some(rate_board(newbrd.brd));
				let dblength = database.len() as usize;
				database[index].children.push(dblength);
				database.push(newbrd);
			    }
			}
		    }
		}
		else {
		    for row in 0..3 {
			for col in 0..3 {
			    if myslice[row][col] == 0 {
				let p: u8 = (row * 3 + col) as u8;
				let mut newbrd = Board {
				    brd: database[index].brd.clone(),
				    scope: p,
				    player: database[index].player * -1,
				    movenum: database[index].movenum + 1,
				    children: Vec::new(),
				    parent: Some(index),
				    prediction: None,
				};
				place(&mut newbrd.brd, database[index].player, database[index].scope, p);
				newbrd.prediction = Some(rate_board(newbrd.brd));
				let dblength = database.len() as usize;
				database[index].children.push(dblength);
				database.push(newbrd.clone());
			    }
			}
		    }
		}
	    }
	}
	let mut curin = 0;
	while unsafe {DB.len()} > curin {
	    let thisboard = unsafe{DB[curin].clone()};
	    if !thisboard.obselete(unsafe{&DB}, unsafe{CURINDEX}) && thisboard.children.len() == 0 {
		tryall(unsafe {&mut DB}, curin);
	    }
	    curin += 1;
	    // println!("{}", curin);
	}
    });
}    

fn twoplayer() {
    let mut board: [[i8; 9]; 9] = [[0; 9]; 9];
    let mut scope = 4;
    let mut player = 1;
    while winner(board) == 0 {
	raw_print_board(board);
	if player == 1 {
	    print!("Player 1");
	}
	else {
	    print!("Player 2");
	}
	print!(", you are on board number {}. Please enter a number, 0-8 (inclusive) for where you want to place your ", scope);
	if player == 1 {
	    print!("X: ");
	}
	else {
	    print!("O: ");
	}
	let s = input();
	let p = s.parse::<u8>().unwrap();
	if p < 9 {
	    place(&mut board, player, scope, p);
	    player *= -1;
	    scope = p;
	}
    }
    raw_print_board(board);
    if winner(board) == 1 {
	print!("Player 1 ");
    }
    else {
	print!("Player 2 ");
    }
    println!("won. Thank you for playing!");
}    

fn input() -> String {
    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
	s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
	s.pop();
    }
    return s;
}

fn place(board: &mut [[i8; 9]; 9], player: i8, scope: u8, p: u8) {
    let p_round: u8 = p / 3;
    let mut scope_round: u8 = scope / 3;
    scope_round *= 3;
    let r: u8 = scope_round + p_round;
    let c: u8 = 3 * (scope % 3) + (p % 3);
    let slice = get_slice(*board, scope);
    let ru: usize = r as usize;
    let cu: usize = c as usize;
    if small_winner(slice) == 0 {
	board[ru][cu] = player;
    }
    else {
	board[ru][cu] = player * 2;
    }
}

fn get(board: [[i8; 9]; 9], scope: u8, p: u8) -> i8 {
    let p_round: u8 = p / 3;
    let mut scope_round: u8 = scope / 3;
    scope_round *= 3;
    let r: u8 = scope_round + p_round;
    let c: u8 = 3 * (scope % 3) + (p % 3);
    return board[r as usize][c as usize];
}

fn get_slice(board: [[i8; 9]; 9], scope: u8) -> [[i8; 3]; 3] {
    let mut slice = [[0; 3]; 3];
    for row in 0..3 {
	for col in 0..3 {
	    slice[row][col] = get(board, scope, (3*row + col) as u8);
	}
    }
    return slice;
}

fn raw_print_board(board: [[i8; 9]; 9]) {
    println!();
    for row in 0..9 {
	if row % 3 == 0 && row != 0 {
	    println!("{}", "-".repeat(23));
	}
	for col in 0..9 {
	    if col % 3 == 0 && col != 0 {
		print!(" |")
	    }
	    let scope: u8 = ((row / 3) as u8) * 3 + (col / 3) as u8;
	    let b_winner = small_winner(get_slice(board, scope));
	    let value = board[row][col];
	    if b_winner == 0 {
		if value == 1 || value == 2 {
		    print!(" X");
		}
		else if value == -1 || value == -2 {
		    print!(" O");
		}
		else {
		    print!(" _");
		}
	    }
	    else {
		if row > 0 && col > 0 && (row - 1) % 3 == 0 && (col - 1) % 3 == 0 {
		    if board[row][col] == 0 {
			print!(" _");
		    }
		    else {
			if b_winner == 1 {
			    print!(" X");
			}
			else {
			    print!(" O");
			}
		    }
		}
		else {
		    if board[(((row / 3) as u8) * 3 + 1) as usize][(((col / 3) as u8) * 3 + 1) as usize] == 0 {
			if value == 0 {
			    print!(" _");
			}
			else {
			    if value == 1 {
				print!(" X");
			    }
			    else {
				print!(" O");
			    }
			}
		    }
		    else {
			if value == 0 {
			    print!(" _");
			}
			else {
			    print!(" *");
			}
		    }
		}
	    }
	}
	println!();
    }
    println!();
}

pub fn print_board(board: &Board) {
    println!();
    for row in 0..9 {
	if row % 3 == 0 && row != 0 {
	    println!("{}", "-".repeat(23));
	}
	for col in 0..9 {
	    if col % 3 == 0 && col != 0 {
		print!(" |")
	    }
	    let scope: u8 = ((row / 3) as u8) * 3 + (col / 3) as u8;
	    let b_winner = small_winner(get_slice(board.brd, scope));
	    let value = board.brd[row][col];
	    if b_winner == 0 {
		if value == 1 || value == 2 {
		    print!(" X");
		}
		else if value == -1 || value == -2 {
		    print!(" O");
		}
		else {
		    print!(" _");
		}
	    }
	    else {
		if row > 0 && col > 0 && (row - 1) % 3 == 0 && (col - 1) % 3 == 0 {
		    if board.brd[row][col] == 0 {
			print!(" _");
		    }
		    else {
			if b_winner == 1 {
			    print!(" X");
			}
			else {
			    print!(" O");
			}
		    }
		}
		else {
		    if board.brd[(((row / 3) as u8) * 3 + 1) as usize][(((col / 3) as u8) * 3 + 1) as usize] == 0 {
			if value == 0 {
			    print!(" _");
			}
			else {
			    if value == 1 {
				print!(" X");
			    }
			    else {
				print!(" O");
			    }
			}
		    }
		    else {
			if value == 0 {
			    print!(" _");
			}
			else {
			    print!(" *");
			}
		    }
		}
	    }
	}
	println!();
    }
    println!();
}

fn winner(board: [[i8; 9]; 9]) -> i8 {
    let mut winners: [[i8; 3]; 3] = [[0; 3]; 3];
    for b_row in 0..3 {
	for b_col in 0..3 {
	    let slice = get_slice(board, (b_row*3 + b_col) as u8);
	    winners[b_row][b_col] = small_winner(slice);
	}
    }
    return small_winner(winners);
}

fn small_winner(board: [[i8; 3]; 3]) -> i8 {
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

fn rate_board(board: [[i8; 9]; 9]) -> f32 {
    let mut cprobs: [[f32; 3]; 3] = [[0.0; 3]; 3];
    let mut hprobs: [[f32; 3]; 3] = [[0.0; 3]; 3];
    for row in 0..3 {
	for col in 0..3 {
	    let slice = get_slice(board, (row * 3 + col) as u8);
	    (cprobs[row][col], hprobs[row][col]) = winprob(prob_convert(slice));
	}
    }
    let cprob = winprob(cprobs).0;
    let hprob = winprob(hprobs).0;
    if cprob == 1.0 {
	return cprob;
    }
    else {
	return cprob * (1.0 - hprob);
    }
}

fn winprob(pboard: [[f32; 3]; 3]) -> (f32, f32) {
    let mut cprobs: Vec<(usize, usize, usize, f32)> = Vec::new();
    let mut hprobs: Vec<(usize, usize, usize, f32)> = Vec::new();

    // rows
    for row in 0..3 {
	cprobs.push((0 + row * 3, 1 + row * 3, 2 + row * 3, pboard[row][0] * pboard[row][1] * pboard[row][2]));
	hprobs.push((0 + row * 3, 1 + row * 3, 2 + row * 3, (1.0 - pboard[row][0]) * (1.0 - pboard[row][1]) * (1.0 - pboard[row][2])));
    }
    // cols
    for col in 0..3 {
	cprobs.push((col, col + 3, col + 6, pboard[0][col] * pboard[1][col] * pboard[2][col]));
	hprobs.push((col, col + 3, col + 6, (1.0 - pboard[0][col]) * (1.0 - pboard[1][col]) * (1.0 - pboard[2][col])));
    }
    // diag 1
    cprobs.push((0, 4, 7, pboard[0][0] * pboard[1][1] * pboard[2][2]));
    hprobs.push((0, 4, 7, (1.0 - pboard[0][0]) * (1.0 - pboard[1][1]) * (1.0 - pboard[2][2])));

    // diag 2
    cprobs.push((2, 4, 6, pboard[0][2] * pboard[1][1] * pboard[2][0]));
    hprobs.push((2, 4, 6, (1.0 - pboard[0][2]) * (1.0 - pboard[1][1]) * (1.0 - pboard[2][0])));

    let mut ctotal: f32 = 0.0;
    while cprobs.len() > 0 {
	let (mindex, mval) = maxenum(&cprobs);
	ctotal += mval;
	filtervec(cprobs[mindex].clone(), &mut cprobs);
    }
    
    let mut htotal: f32 = 0.0;
    while hprobs.len() > 0 {
	let (mindex, mval) = maxenum(&hprobs);
	htotal += mval;
	filtervec(hprobs[mindex].clone(), &mut hprobs);
    }
    
    return (ctotal, htotal);
}

fn maxenum(v: &Vec<(usize, usize, usize, f32)>) -> (usize, f32) {
    let mut maxi = 0;
    let mut maxv = 0.0;
    for (index, &tup) in v.iter().enumerate() {
	if tup.3 > maxv {
	    maxv = tup.3;
	    maxi = index;
	}
    }
    return (maxi, maxv);
}

fn intersect(tup1: (usize, usize, usize, f32), tup2: (usize, usize, usize, f32)) -> bool {
    return tup1.0 == tup2.0 || tup1.0 == tup2.1 || tup1.0 == tup2.2 ||
	tup1.1 == tup2.0 || tup1.1 == tup2.1 || tup1.1 == tup2.2 ||
	tup1.2 == tup2.0 || tup2.0 == tup2.1 || tup2.0 == tup2.2;
}

fn filtervec(tup: (usize, usize, usize, f32), v: &mut Vec<(usize, usize, usize, f32)>) {
    let mut index: usize = 0;
    while index < v.len() {
	if intersect(tup, v[index]) {
	    v.remove(index);
	}
	else {
	    index += 1;
	}
    }
}

fn prob_convert(board: [[i8; 3]; 3]) -> [[f32; 3]; 3] {
    let mut toreturn: [[f32; 3]; 3] = [[0.0; 3]; 3];
    for row in 0..3 {
	for col in 0..3 {
	    toreturn[row][col] = match board[row][col] {
		1  => 0.0,
		0 => 0.5,
		-1 => 1.0,
		_ => -1.0,
	    }
	}
    }
    return toreturn;
}

fn is_full(board: [[i8; 3]; 3]) -> bool {
    for row in 0..3 {
	for col in 0..3 {
	    if board[row][col] == 0 {
		return false;
	    }
	}
    }
    return true;
}

fn domove(board: &Board, pindex: u8) -> usize {
    if board.player == 1 {
	let mut veccount: usize = 0;
	for row in 0..3 {
	    for col in 0..3 {
		if row * 3 + col < pindex {
		    if get(board.brd, board.scope, (row * 3 + col) as u8) == 0 {
			veccount += 1;
		    }
		}
	    }
	}
	return board.children[veccount];
    }
    else {
	return board.children[pindex as usize];
    }
}

pub fn getcpmove(db: &mut Vec<Board>, curindex: usize, maxdepth: Option<u8>) -> u8 {
    for child in 0..db[curindex].children.len() {
	let newindex = db[curindex].children[child];
	calcmove(&mut *db, newindex, maxdepth);
    }
    let haswon = small_winner(get_slice(db[curindex].brd, db[curindex].scope)) != 0;
    let mut minindex: u8 = 0;
    let mut minrating: Option<f32> = None;
    let mut all_loop = true;
    for childnum in 0..db[curindex].children.len() {
	let child = &db[db[curindex].children[childnum]];
	let isloop = haswon && (loopstuck(&db, curindex, childnum));
	let nextwon = small_winner(get_slice(child.brd, child.scope)) != 0;
	if !(isloop && !nextwon) {
	    all_loop = false;
	}
    }
    for childnum in 0..db[curindex].children.len() {
	let child = &db[db[curindex].children[childnum]];
	let childrating = child.prediction.unwrap();
	// print!(", {}", childrating);
	let isloop = haswon && (loopstuck(&db, curindex, childnum));
	let nextwon = small_winner(get_slice(child.brd, child.scope)) != 0;
	if (all_loop || !(isloop && !nextwon)) && (minrating == None || childrating < minrating.unwrap()) {
	    minrating = Some(childrating);
	    minindex = childnum as u8;
	}
    }
    println!("Computer is moving towards a board with rating {}", minrating.unwrap());
    return minindex;
}

fn calcmove(db: &mut Vec<Board>, curindex: usize, maxdepth: Option<u8>) {
    if maxdepth.is_some() && db[curindex].movenum == maxdepth.unwrap() {
	return;
    }
    else {
	for child in 0..db[curindex].children.len() {
	    let newindex = db[curindex].children[child];
	    calcmove(&mut *db, newindex, maxdepth);
	}
	if !db[curindex].children.len() > 0 {
	    updatepred(&mut *db, curindex);
	}
    }
}

fn loopstuck(db: &Vec<Board>, curindex: usize, child: usize) -> bool {
    let stuckboard = db[curindex].scope;
    let grandchildren = &db[db[curindex].children[child]].children;
    for grandchild in 0..grandchildren.len() {
	if db[grandchildren[grandchild]].scope == stuckboard {
	    return true;
	}
    }
    return false;
}
