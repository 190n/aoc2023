use std::{collections::VecDeque, fmt::Debug, io};

#[derive(Clone, Copy, PartialEq, Eq)]
enum DigState {
	Ground,
	Dug,
	Marked,
}

use DigState::*;

struct Digger {
	x: usize,
	y: usize,
	dug: Vec<Vec<DigState>>,
}

impl Digger {
	pub fn new() -> Digger {
		Digger {
			x: 0,
			y: 0,
			dug: vec![vec![Dug]],
		}
	}

	fn right(&mut self) -> () {
		if self.x + 1 >= self.dug[0].len() {
			for row in self.dug.iter_mut() {
				row.push(Ground);
			}
		}
		self.x += 1;
		self.dug[self.y][self.x] = Dug;
	}

	fn down(&mut self) -> () {
		if self.y + 1 >= self.dug.len() {
			self.dug.push(vec![Ground; self.dug[0].len()]);
		}
		self.y += 1;
		self.dug[self.y][self.x] = Dug;
	}

	fn left(&mut self) -> () {
		if self.x == 0 {
			for row in self.dug.iter_mut() {
				row.insert(0, Ground);
			}
		} else {
			self.x -= 1;
		}
		self.dug[self.y][self.x] = Dug;
	}

	fn up(&mut self) -> () {
		if self.y == 0 {
			self.dug.insert(0, vec![Ground; self.dug[0].len()]);
		} else {
			self.y -= 1;
		}
		self.dug[self.y][self.x] = Dug;
	}

	fn adjacent<'a>(
		&self,
		(x, y): (usize, usize),
		buf: &'a mut [(usize, usize); 4],
	) -> &'a [(usize, usize)] {
		let mut count: usize = 0;
		let width = self.dug[0].len();
		let height = self.dug.len();

		if x > 0 {
			buf[count] = (x - 1, y);
			count += 1;
		}
		if y > 0 {
			buf[count] = (x, y - 1);
			count += 1;
		}
		if x < width - 1 {
			buf[count] = (x + 1, y);
			count += 1;
		}
		if y < height - 1 {
			buf[count] = (x, y + 1);
			count += 1;
		}

		&buf[0..count]
	}

	/// count the number of enclosed tiles
	fn fill(&mut self) -> usize {
		let (width, height) = (self.dug[0].len(), self.dug.len());
		// expand the grid so that we can fill from a corner
		self.dug.insert(0, vec![Ground; width + 2]);
		self.dug.push(vec![Ground; width + 2]);
		for row in self.dug.iter_mut().skip(1).take(height) {
			row.insert(0, Ground);
			row.push(Ground);
		}

		self.fill_from((0, 0));

		// count "marked" tiles that are outside
		let num_marked = self
			.dug
			.iter()
			.flat_map(|row| row.iter())
			.filter(|&&pos| pos == Marked)
			.count();
		return ((width + 2) * (height + 2)) - num_marked;
	}

	fn fill_from(&mut self, (x, y): (usize, usize)) -> () {
		let mut q = VecDeque::<(usize, usize)>::new();
		q.push_front((x, y));
		let mut buf = [(0usize, 0usize); 4];

		while let Some((x, y)) = q.pop_front() {
			if self.dug[y][x] != Ground {
				continue;
			}
			self.dug[y][x] = Marked;
			for &(new_x, new_y) in self.adjacent((x, y), &mut buf) {
				if self.dug[new_y][new_x] != Ground {
					continue;
				}
				q.push_back((new_x, new_y));
			}
		}
	}
}

impl Debug for Digger {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (y, row) in self.dug.iter().enumerate() {
			for (x, dug) in row.iter().enumerate() {
				if x == self.x && y == self.y {
					write!(f, "x")?;
				} else {
					write!(
						f,
						"{}",
						match *dug {
							Ground => '.',
							Dug => '#',
							Marked => '=',
						}
					)?;
				}
			}
			if y != self.dug.len() - 1 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	let mut d = Digger::new();

	for line in iter {
		let mut split = line.split(' ');
		let dir = split.next().expect("no direction provided");
		let distance: usize = split
			.next()
			.expect("no distance provided")
			.parse()
			.expect("invalid distance");

		for _ in 0..distance {
			match dir {
				"R" => d.right(),
				"L" => d.left(),
				"U" => d.up(),
				"D" => d.down(),
				_ => panic!("invalid direction"),
			}
		}
	}

	dbg!(d.fill());

	Ok(())
}
