use std::vec::Vec;

fn main() {
    let mut tocheck: Vec<Board> = Vec::new();
    struct Board<'a> {
	brd: [[i8; 9]; 9],
	scope: u8,
	player: i8,
	movenum: u8,
	parent: Option<&'a Board<'a>>,
	child: Option<&'a Board<'a>>,
	// children: [Option<&'a Board<'a>>; 9],
    }
    let mut starter = Board {
	brd: [[0; 9]; 9],
	scope: 4,
	player: 1,
	movenum: 0,
	parent: None,
	child: None,
	// children: [None; 9],
    };
    let child = Board {
	brd: starter.brd.clone(),
	scope: 0,
	player: -1,
	movenum: 1,
	parent: Some(&starter),
	child: None,
	// children: [None; 9],
    };
    starter.child = Some(&child);
    // tocheck.push(child);
    for entry in 0..3 {
	println!("{}", tocheck[entry].movenum);
    }
}
