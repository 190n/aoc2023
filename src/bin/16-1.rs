use std::{fmt::Debug, io};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Empty,
	BackMirror,
	FwdMirror,
	HSplit,
	VSplit,
}

use Tile::*;

impl TryFrom<char> for Tile {
	type Error = char;
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'.' => Ok(Empty),
			'\\' => Ok(BackMirror),
			'/' => Ok(FwdMirror),
			'-' => Ok(HSplit),
			'|' => Ok(VSplit),
			c => Err(c),
		}
	}
}

impl Debug for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Empty => '.',
				BackMirror => '\\',
				FwdMirror => '/',
				HSplit => '-',
				VSplit => '|',
			}
		)
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
	Left = 1,
	Right = 2,
	Up = 4,
	Down = 8,
}

use Direction::*;

impl Direction {
	pub fn apply(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
		match *self {
			Left => {
				if x == 0 {
					None
				} else {
					Some((x - 1, y))
				}
			},
			Right => {
				if x == w - 1 {
					None
				} else {
					Some((x + 1, y))
				}
			},
			Up => {
				if y == 0 {
					None
				} else {
					Some((x, y - 1))
				}
			},
			Down => {
				if y == h - 1 {
					None
				} else {
					Some((x, y + 1))
				}
			},
		}
	}
}

fn send_beam(
	machine: &Vec<Vec<Tile>>,
	mut x: usize,
	mut y: usize,
	mut dir: Direction,
	energized: &mut Vec<Vec<u8>>,
) -> () {
	let w = machine[0].len();
	let h = machine.len();

	loop {
		if energized[y][x] & dir as u8 != 0 {
			// we have been at this tile in this direction before
			return;
		}

		energized[y][x] |= dir as u8;

		match machine[y][x] {
			BackMirror => {
				dir = match dir {
					Left => Up,
					Right => Down,
					Up => Left,
					Down => Right,
				}
			},
			FwdMirror => {
				dir = match dir {
					Left => Down,
					Right => Up,
					Up => Right,
					Down => Left,
				};
			},
			HSplit => {
				if dir == Up || dir == Down {
					// send both new beams, then return
					send_beam(machine, x, y, Left, energized);
					send_beam(machine, x, y, Right, energized);
					return;
				}
			},
			VSplit => {
				if dir == Left || dir == Right {
					send_beam(machine, x, y, Up, energized);
					send_beam(machine, x, y, Down, energized);
					return;
				}
			},
			Empty => {},
		}

		if let Some((new_x, new_y)) = dir.apply(x, y, w, h) {
			x = new_x;
			y = new_y;
		} else {
			// beam left grid
			return;
		}
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut machine: Vec<Vec<Tile>> = Vec::new();

	for line in iter {
		let mut row: Vec<Tile> = Vec::new();
		for c in line.chars() {
			row.push(Tile::try_from(c).unwrap());
		}
		machine.push(row);
	}

	let mut energized = vec![vec![0u8; machine[0].len()]; machine.len()];
	send_beam(&machine, 0, 0, Right, &mut energized);

	let num_energized = energized
		.iter()
		.flat_map(|r| r.iter())
		.filter(|&&d| d != 0)
		.count();
	dbg!(num_energized);

	Ok(())
}
