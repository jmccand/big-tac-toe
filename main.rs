use std::io::{stdin,stdout,Write};
use std::fs::File;
// use std::io::prelude::*;

fn test() {
    let mut board: [[i8; 9]; 9] = [[2, -2, 0, 2, 2, -1, 0, 1, 0, ],
				   [1, 1, 1, 2, -1, 2, -2, 1, 0, ],
				   [2, -2, 0, -1, -2, 2, 0, 1, 0, ],
				   [-1, -1, -1, -1, 1, -1, -1, -1, 0, ],
				   [1, 2, -1, -1, 1, 0, 1, 0, 1, ],
				   [2, 1, 2, -1, 0, 0, -1, 0, 0, ],
				   [-1, 1, 2, -1, -1, 0, 0, -1, 0, ],
				   [-1, 0, 0, -1, 0, -1, -1, 0, 0, ],
				   [-1, 1, 0, 1, 1, 0, 0, 0, 0, ],
    ];
    println!("Rating: {}", rate_board(board));
    place(&mut board, 1, 6, 4);
    print_board(board);
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
    if false {
	test();
	return;
    }
    let mut game_history = File::create("game_history.txt").expect("failed to open file");
    print!("Welcome to Big Tac Toe! Enter the number of players: ");
    let s = input();
    if s == "1" {
	let mut board: [[i8; 9]; 9] = [[0; 9]; 9];
	let mut scope = 4;
	let mut player = 1;
	println!("Welcome to 1 player Big Tac Toe!");
	while winner(board) == 0 {
	    write_board(board, &mut game_history);
	    if player == 1 {
		// println!("Board rating: {}", rate_board(board));
		print_board(board);
		if is_full(get_slice(board, scope)) {
		    print!("You are on board number {}, but because it is full you can go anywhere you'd like. Please enter a number, 0-8 (inclusive) for where you want to set the scope (which quadrant): ", scope);
		    let nscope = input();
		    let intscope = nscope.parse::<i8>().unwrap();
		    if intscope >= 0 && intscope < 9 {
			scope = intscope;
		    }
		    else {
			continue;
		    }
		}
		print!("You are on board number {}. Please enter a number, 0-8 (inclusive) for where you want to place your X: ", scope);
		let s = input();
		let p = s.parse::<i8>().unwrap();
		if p >= 0 && p < 9 && get(board, scope, p) == 0 {
		    place(&mut board, player, scope, p);
		    player *= -1;
		    scope = p;
		}
		print!("Finished your turn! Time for the computer!");
	    }
	    else {
		let p: i8;
		if is_full(get_slice(board, scope)) {
		    let enc = cpturn(0, board, scope);
		    scope = (enc / 10) as i8;
		    p = (enc - ((scope * 10) as i32)) as i8;
		}
		else {
		    p = cpturn(0, board, scope) as i8;
		}
		print!("Computer done thinking! Time to place!");
		place(&mut board, player, scope, p);
		player *= -1;
		scope = p;
	    }
	}
	print_board(board);
	if winner(board) == 1 {
	    print!("You ");
	}
	else {
	    print!("The computer ");
	}
	println!("won. Thank you for playing!");
    }
    else if s == "2" {
	let mut board: [[i8; 9]; 9] = [[0; 9]; 9];
	let mut scope = 4;
	let mut player = 1;
	println!("Welcome to 2 player Big Tac Toe!");
	while winner(board) == 0 {
	    print_board(board);
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
	    let p = s.parse::<i8>().unwrap();
	    if p >= 0 && p < 9 {
		place(&mut board, player, scope, p);
		player *= -1;
		scope = p;
	    }
	}
	print_board(board);
	if winner(board) == 1 {
	    print!("Player 1 ");
	}
	else {
	    print!("Player 2 ");
	}
	println!("won. Thank you for playing!");
    }
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

fn place(board: &mut [[i8; 9]; 9], player: i8, scope: i8, p: i8) {
    let mut p_round: i8 = p / 3;
    p_round = p_round as i8;
    let mut scope_round: i8 = scope / 3;
    scope_round *= 3;
    let r: i8 = scope_round + p_round;
    let c: i8 = 3 * (scope % 3) + (p % 3);
    let slice = get_slice(*board, scope);
    let ru: usize = r as usize;
    let cu: usize = c as usize;
    if ru > 8 {
	println!("ru: {}, r: {}, scope: {}, p_round: {}, p: {}", ru, r, scope_round, p_round, p);
    }
    if cu > 8 {
	println!("cu: {}, c: {}, scope: {}, p: {}", cu, c, scope, p);
    }
    if small_winner(slice) == 0 {
	board[ru][cu] = player;
    }
    else {
	board[ru][cu] = player * 2;
    }
}

fn get(board: [[i8; 9]; 9], scope: i8, p: i8) -> i8 {
    let mut p_round: i8 = p / 3;
    p_round = p_round as i8;
    let mut scope_round = scope / 3;
    scope_round = scope_round as i8;
    scope_round *= 3;
    let r: i8 = scope_round + p_round;
    let c: i8 = 3 * (scope % 3) + (p % 3);
    return board[r as usize][c as usize];
}

fn get_slice(board: [[i8; 9]; 9], scope: i8) -> [[i8; 3]; 3] {
    let mut slice = [[0; 3]; 3];
    for row in 0..3 {
	for col in 0..3 {
	    slice[row][col] = get(board, scope, (3*row + col) as i8);
	}
    }
    return slice;
}

fn print_board(board: [[i8; 9]; 9]) {
    println!();
    for row in 0..9 {
	if row % 3 == 0 && row != 0 {
	    println!("{}", "-".repeat(23));
	}
	for col in 0..9 {
	    if col % 3 == 0 && col != 0 {
		print!(" |")
	    }
	    let scope: i8 = ((row / 3) as i8) * 3 + (col / 3) as i8;
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
		    if board[(((row / 3) as i8) * 3 + 1) as usize][(((col / 3) as i8) * 3 + 1) as usize] == 0 {
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
	    let slice = get_slice(board, (b_row*3 + b_col) as i8);
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

// old cpturn function using floats and averaging human player scores
// fn cpturn(ahead: i32, board: [[i8; 9]; 9], scope: i8) -> f32 {
//   // if ahead < 3 {
//   //   println!("AHEAD: {}", ahead);
//   //   print_board(board);
//   // }
//   // if ahead == 8 {
//   //   println!("AHEAD: {}", ahead);
//   //   print_board(board);
//   //   return 0.0;
//   // }
//   let mut win_ps: [f32; 9] = [-1.0; 9];
//   for row in 0..3 {
//     for col in 0..3 {
//       if board[(((scope / 3) as i8) * 3 + row) as usize][(3 * (scope % 3) + col) as usize] == 0 {
//         let mut cp_board = board.clone();
//         cp_board[(((scope / 3) as i8) * 3 + row) as usize][(3 * (scope % 3) + col) as usize] = -1;
//         if winner(cp_board) == 0 {
//           // opponent's move
//           let oscope = 3 * row + col;
//           let mut win2_ps: [f32; 9] = [-1.0; 9];
//           for orow in 0..3 {
//             for ocol in 0..3 {
//               if cp_board[(((oscope / 3) as i8) * 3 + orow) as usize][(3 * (oscope % 3) + ocol) as usize] == 0 {
//                 let mut cp2_board = cp_board.clone();
//                 cp2_board[(((oscope / 3) as i8) * 3 + orow) as usize][(3 * (oscope % 3) + ocol) as usize] = 1;
//                 win2_ps[(3 * orow + ocol) as usize] = cpturn(ahead + 1, cp2_board, 3 * orow + ocol);
//               }
//             }
//           }
//           let mut ototal: f32 = 0.0;
//           let mut opos = 0;
//           for win2_p in win2_ps {
//             if win2_p != -1.0 {
//               ototal += win2_p;
//               opos += 1;
//             }
//           }
//           win_ps[(3 * row + col) as usize] = ototal / (opos as f32);
//         }
//         else if winner(cp_board) == 1 {
//           return 0.0;
//         }
//         else {
//           // println!("WINNER");
//           // print_board(cp_board);
//           return 9.0;
//         }
//       }
//     }
//   }
//   if ahead == 0 {
//     let mut max_index = 0;
//     for (index, win_p) in win_ps.iter().enumerate() {
//       if *win_p > win_ps[max_index] {
//         max_index = index;
//       }
//     }
//     return max_index as f32;
//   }
//   else {
//     let mut max_value = 0.0;
//     for win_p in win_ps {
//       if win_p > max_value {
//         max_value = win_p;
//       }
//     }
//     return max_value as f32;
//   }
// }

// old cpturn function (not oldest, v2) that uses ints and returns the player who wins
// fn cpturn(ahead: i32, board: [[i8; 9]; 9], scope: i8) -> i8 {
//   // if ahead < 3 {
//   //   println!("AHEAD: {}", ahead);
//   //   print_board(board);
//   // }
//   if ahead == 3 {
//     // println!("AHEAD: {}", ahead);
//     // print_board(board);
//     return 0;
//   }
//   let mut winners: [i8; 9] = [0; 9];
//   for row in 0..3 {
//     for col in 0..3 {
//       if board[(((scope / 3) as i8) * 3 + row) as usize][(3 * (scope % 3) + col) as usize] == 0 {
//         let mut cp_board = board.clone();
//         cp_board[(((scope / 3) as i8) * 3 + row) as usize][(3 * (scope % 3) + col) as usize] = -1;
//         if winner(cp_board) == 0 {
//           // opponent's move
//           let oscope = 3 * row + col;
//           let mut owinners: [i8; 9] = [0; 9];
//           for orow in 0..3 {
//             for ocol in 0..3 {
//               if cp_board[(((oscope / 3) as i8) * 3 + orow) as usize][(3 * (oscope % 3) + ocol) as usize] == 0 {
//                 let mut cp2_board = cp_board.clone();
//                 cp2_board[(((oscope / 3) as i8) * 3 + orow) as usize][(3 * (oscope % 3) + ocol) as usize] = 1;
//                 owinners[(3 * orow + ocol) as usize] = cpturn(ahead + 1, cp2_board, 3 * orow + ocol);
//               }
//             }
//           }
//           let mut omax: i8 = 0;
//           for owin in owinners {
//             if owin > omax {
//               omax = owin;
//             }
//           }
//           winners[(3 * row + col) as usize] = omax;
//         }
//         else {
//           // println!("WINNER");
//           // print_board(cp_board);
//           return winner(cp_board);
//         }
//       }
//     }
//   }
//   if ahead == 0 {
//     let mut max_index = 0;
//     for (index, win_p) in winners.iter().enumerate() {
//       if *win_p > winners[max_index] {
//         max_index = index;
//       }
//     }
//     return max_index as i8;
//   }
//   else {
//     let mut max_value = 0;
//     for win_p in winners {
//       if win_p > max_value {
//         max_value = win_p;
//       }
//     }
//     return max_value;
//   }
// }

fn rate_board(board: [[i8; 9]; 9]) -> i32 {
    let mut total = 0;
    let mut ratings: [[i8; 3]; 3] = [[0; 3]; 3];
    for b_row in 0..3 {
	for b_col in 0..3 {
	    let slice = get_slice(board, (3*b_row + b_col) as i8);
	    ratings[b_row][b_col] = small_winner(slice);
	    total += rate_small(slice);
	}
    }
    let overall_rating = rate_small(ratings);
    if overall_rating < 0 {
	total -= overall_rating * overall_rating;
    }
    else {
	total += overall_rating * overall_rating;
    }
    return total;
}

fn rate_small(board: [[i8; 3]; 3]) -> i32 {
    let mut total: i32 = 0;
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
	if counts[0] == 0 || counts[1] == 0 {
	    total += counts[0] * counts[0];
	    total -= counts[1] * counts[1];
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
	if counts[0] == 0 || counts[1] == 0 {
	    total += counts[0] * counts[0];
	    total -= counts[1] * counts[1];
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
	if counts[0] == 0 || counts[1] == 0 {
	    total += counts[0] * counts[0];
	    total -= counts[1] * counts[1];
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
	if counts[0] == 0 || counts[1] == 0 {
	    total += counts[0] * counts[0];
	    total -= counts[1] * counts[1];
	}
    }
    return total;
}

fn possible_moves(board: [[i8; 3]; 3]) -> i32 {
    let mut possible: i32 = 0;
    for row in 0..3 {
	for col in 0..3 {
	    if board[row][col] == 0 {
		possible += 1;
	    }
	}
    }
    return possible;
}

fn cpturn(ahead: i32, board: [[i8; 9]; 9], scope: i8) -> i32 {
    if ahead == 3 {
	// println!("AHEAD: {}, RATING: {}", ahead, rate_board(board));
	// print_board(board);
	return rate_board(board);
    }
    else if rate_board(board) < -30 {
	return rate_board(board);
    }
    else if possible_moves(get_slice(board, scope)) > 5 && ahead > 1 {
	return rate_board(board);
    }
    if is_full(get_slice(board, scope)) {
	let mut ratings: [i32; 81] = [10000; 81];
	for nscope in 0..9 {
	    for row in 0..3 {
		for col in 0..3 {
		    let p: i8 = 3*row + col;
		    if get(board, nscope, p) == 0 {
			let mut cp_board = board.clone();
			place(&mut cp_board, -1, nscope, p);
			// opponent's move
			let mut oratings: [i32; 9] = [-10000; 9];
			for orow in 0..3 {
			    for ocol in 0..3 {
				let op: i8 = 3*orow + ocol;
				if get(cp_board, p, op) == 0 {
				    let mut cp2_board = cp_board.clone();
				    place(&mut cp2_board, 1, p, op);
				    oratings[(3 * orow + ocol) as usize] = cpturn(ahead + 1, cp2_board, 3 * orow + ocol);
				}
			    }
			}
			let mut omax = -10000;
			for owin in oratings {
			    if owin > omax {
				omax = owin;
			    }
			}
			ratings[((3 * row + col) + nscope * 9) as usize] = omax;
		    }
		}
	    }
	}
	if ahead == 0 {
	    let mut min_index = 0;
	    for (index, win_p) in ratings.iter().enumerate() {
		if *win_p < ratings[min_index] {
		    min_index = index;
		}
	    }
	    // combine scope and p value
	    let nscope = (min_index / 9) as i8;
	    let p_val = (min_index - nscope as usize) as i8;
	    return (nscope * 10 + p_val) as i32;
	}
	else {
	    let mut min_value = 10000;
	    for win_p in ratings {
		if win_p < min_value {
		    min_value = win_p;
		}
	    }
	    return min_value;
	}
    }
    else {
	let mut ratings: [i32; 9] = [10000; 9];
	for row in 0..3 {
	    for col in 0..3 {
		let p: i8 = 3*row + col;
		if get(board, scope, p) == 0 {
		    let mut cp_board = board.clone();
		    place(&mut cp_board, -1, scope, p);
		    // opponent's move
		    let mut oratings: [i32; 9] = [-10000; 9];
		    for orow in 0..3 {
			for ocol in 0..3 {
			    let op: i8 = 3*orow + ocol;
			    if get(cp_board, p, op) == 0 {
				let mut cp2_board = cp_board.clone();
				place(&mut cp2_board, 1, p, op);
				oratings[(3 * orow + ocol) as usize] = cpturn(ahead + 1, cp2_board, 3 * orow + ocol);
			    }
			}
		    }
		    let mut omax = -10000;
		    for owin in oratings {
			if owin > omax {
			    omax = owin;
			}
		    }
		    ratings[(3 * row + col) as usize] = omax;
		}
	    }
	}
	if ahead == 0 {
	    let mut min_index: i32 = 0;
	    // if ahead == 0 {
	    //   println!();
	    // }
	    for (index, win_p) in ratings.iter().enumerate() {
		// if ahead == 0 {
		//   print!("{} ", win_p);
		// }
		if *win_p < ratings[min_index as usize] {
		    min_index = index as i32;
		}
	    }
	    // if ahead == 0 {
	    //   println!();
	    // }
	    print!("Returning {}", min_index);
	    return min_index;
	}
	else {
	    let mut min_value = 10000;
	    if ahead == 0 {
		println!();
	    }
	    for win_p in ratings {
		if ahead == 0 {
		    print!("{} ", win_p);
		}
		if win_p < min_value {
		    min_value = win_p;
		}
	    }
	    if ahead == 0 {
		println!();
	    }
	    return min_value;
	}
    }
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
