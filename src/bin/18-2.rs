use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

use Direction::*;

impl Direction {
	fn offset(&self) -> (i64, i64) {
		match *self {
			Up => (0, -1),
			Down => (0, 1),
			Left => (-1, 0),
			Right => (1, 0),
		}
	}
}

impl TryFrom<u8> for Direction {
	type Error = u8;
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'0' => Ok(Right),
			b'1' => Ok(Down),
			b'2' => Ok(Left),
			b'3' => Ok(Up),
			x => Err(x),
		}
	}
}

struct Digger {
	x: i64,
	y: i64,
	area: i64,
	last_direction: Option<Direction>,
}

impl Digger {
	pub fn new() -> Digger {
		Digger {
			x: 0,
			y: 0,
			area: 0,
			last_direction: None,
		}
	}

	pub fn dig(&mut self, dir: Direction, distance: i64) -> () {
		let (off_x, off_y) = dir.offset();
		let (new_x, new_y) = (
			self.x + (distance + 1) * off_x,
			self.y + (distance + 1) * off_y,
		);

		self.area += ((self.y + new_y) * (self.x - new_x)) / 2;

		self.x = new_x;
		self.y = new_y;
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	let mut d = Digger::new();

	for line in iter {
		let mut split = line.split(' ');
		let command = split.nth(2).unwrap();
		let hex_dist = &command[2..7];
		let dist: i64 = i64::from_str_radix(hex_dist, 16).unwrap();
		let dir = Direction::try_from(command.as_bytes()[7]).unwrap();

		d.dig(dir, dist);
		dbg!(d.x, d.y);
	}

	dbg!(d.area);

	Ok(())
}
