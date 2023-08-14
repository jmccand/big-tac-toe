use std::vec::Vec;

fn main() {
    #[derive(Copy, Clone)]
    struct Board {
	brd: [[i8; 9]; 9],
	scope: u8,
	player: i8,
	movenum: u8,
	parent: Option<usize>,
	children: [Option<usize>; 9],
    }
    let mut starter = Board {
	brd: [[0; 9]; 9],
	scope: 4,
	player: 1,
	movenum: 0,
	children: [None; 9],
	parent: None,
    };
    let mut db: Vec<Board> = Vec::new();
    db.push(starter);
    fn tryall (db: &mut Vec<Board>, index: usize) {
	let mut b = db[index];
	for row in 0..3 {
	    for col in 0..3 {
		let p: u8 = (row * 3 + col) as u8;
		let mut newbrd = Board {
		    brd: b.brd.clone(),
		    scope: p,
		    player: b.player * -1,
		    movenum: b.movenum + 1,
		    children: [None; 9],
		    parent: Some(index),
		};
		b.children[p as usize] = Some(db.len() as usize);
		db.push(newbrd);
	    }
	}
    };
    let mut curin = 0;
    while db.len() > curin {
	tryall(&mut db, curin);
	curin += 1;
	if db.len() <= curin {
	    break;
	}
    }
}
