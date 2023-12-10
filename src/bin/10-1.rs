use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
	Vert,
	Horiz,
	Ne,
	Nw,
	Sw,
	Se,
	Ground,
	Start,
}

use Pipe::*;

#[derive(Debug)]
enum PipeError {
	InvalidCharacter(char),
}

impl TryFrom<char> for Pipe {
	type Error = PipeError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'|' => Ok(Vert),
			'-' => Ok(Horiz),
			'L' => Ok(Ne),
			'J' => Ok(Nw),
			'7' => Ok(Sw),
			'F' => Ok(Se),
			'.' => Ok(Ground),
			'S' => Ok(Start),
			_ => Err(PipeError::InvalidCharacter(value)),
		}
	}
}

impl Pipe {
	pub fn connects(&self, off_x: isize, off_y: isize) -> bool {
		match *self {
			Ground => false,
			Vert => off_x == 0,
			Horiz => off_y == 0,
			Ne => match (off_x, off_y) {
				(0, -1) => true,
				(1, 0) => true,
				_ => false,
			},
			Nw => match (off_x, off_y) {
				(0, -1) => true,
				(-1, 0) => true,
				_ => false,
			},
			Sw => match (off_x, off_y) {
				(0, 1) => true,
				(-1, 0) => true,
				_ => false,
			},
			Se => match (off_x, off_y) {
				(0, 1) => true,
				(1, 0) => true,
				_ => false,
			},
			Start => false,
		}
	}
}

#[derive(Debug)]
struct Grid {
	grid: Vec<Vec<Pipe>>,
	distances: Vec<Vec<i32>>,
	start: (usize, usize),
}

impl Grid {
	fn parse(lines: impl Iterator<Item = String>) -> Result<Grid, PipeError> {
		let mut g = Grid {
			grid: Vec::new(),
			distances: Vec::new(),
			start: (0, 0),
		};
		for (y, l) in lines.enumerate() {
			let mut row: Vec<Pipe> = Vec::new();
			for (x, c) in l.chars().enumerate() {
				let pos = Pipe::try_from(c)?;
				row.push(pos);
				if pos == Start {
					g.start = (x, y);
				}
			}
			g.grid.push(row);
			g.distances.push(vec![-1; g.grid[0].len()]);
		}
		Ok(g)
	}

	fn reachable<'a>(
		&self,
		(x, y): (usize, usize),
		buf: &'a mut [(usize, usize); 4],
	) -> &'a [(usize, usize)] {
		let mut count: usize = 0;
		let (sx, sy) = (x as isize, y as isize);
		let current_pipe = self.grid[y][x];

		for off_y in [-1, 0, 1] {
			for off_x in [-1, 0, 1] {
				let (curr_x, curr_y) = (sx + off_x, sy + off_y);
				if curr_x < 0
					|| curr_y < 0 || curr_x >= self.grid[0].len() as isize
					|| curr_y >= self.grid.len() as isize
				{
					continue;
				}
				if off_x == 0 && off_y == 0 {
					continue;
				}
				let (curr_x, curr_y) = (curr_x as usize, curr_y as usize);
				if self.grid[curr_y][curr_x] == Ground {
					continue;
				}
				if current_pipe == Start {
					let new_pipe = self.grid[curr_y][curr_x];
					if !new_pipe.connects(-off_x, -off_y) {
						continue;
					}
				} else if !current_pipe.connects(off_x, off_y) {
					continue;
				}
				buf[count] = (curr_x, curr_y);
				count += 1;
			}
		}
		&buf[0..count]
	}

	fn explore(&mut self, (x, y): (usize, usize), distance: i32) -> () {
		self.distances[y][x] = distance;
		let current_pipe = self.grid[y][x];
		let mut buf = [(0usize, 0usize); 4];

		for &(new_x, new_y) in self.reachable((x, y), &mut buf) {
			if current_pipe != Start && self.distances[new_y][new_x] >= 0 {
				if self.distances[new_y][new_x] < distance + 1 {
					continue;
				}
			}

			self.explore((new_x, new_y), distance + 1);
		}
	}

	fn find_max_distance(&self) -> i32 {
		let mut current_pos = self.start;
		let mut buf = [(0usize, 0usize); 4];
		loop {
			if let Some(&new_pos) =
				self.reachable(current_pos, &mut buf)
					.iter()
					.max_by(|&&(x1, y1), &&(x2, y2)| {
						self.distances[y1][x1].cmp(&self.distances[y2][x2])
					}) {
				if self.distances[current_pos.1][current_pos.0]
					> self.distances[new_pos.1][new_pos.0]
				{
					return self.distances[current_pos.1][current_pos.0];
				} else {
					current_pos = new_pos;
				}
			}
		}
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut grid = Grid::parse(iter).unwrap();
	grid.explore(grid.start, 0);
	// dbg!(grid);

	// for row in &grid.distances {
	// 	for d in row {
	// 		print!("{:4}", d);
	// 	}
	// 	print!("\n");
	// }

	dbg!(grid.find_max_distance());

	Ok(())
}
