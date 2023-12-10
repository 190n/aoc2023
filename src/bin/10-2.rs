use std::{collections::VecDeque, io};

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
	outside: Vec<Vec<bool>>,
}

impl Grid {
	fn parse(lines: impl Iterator<Item = String>) -> Result<Grid, PipeError> {
		let mut g = Grid {
			grid: Vec::new(),
			distances: Vec::new(),
			start: (0, 0),
			outside: Vec::new(),
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
			for _ in 0..3 {
				g.outside.push(vec![false; 3 * g.grid[0].len()]);
			}
		}
		Ok(g)
	}

	fn adjacent<'a, T>(
		grid: &Vec<Vec<T>>,
		(x, y): (usize, usize),
		buf: &'a mut [(usize, usize); 4],
	) -> &'a [(usize, usize)] {
		let mut count: usize = 0;
		let (sx, sy) = (x as isize, y as isize);

		for off_y in [-1, 0, 1] {
			for off_x in [-1, 0, 1] {
				let (curr_x, curr_y) = (sx + off_x, sy + off_y);
				if curr_x < 0
					|| curr_y < 0 || curr_x >= grid[0].len() as isize
					|| curr_y >= grid.len() as isize
				{
					continue;
				}
				if off_x == 0 && off_y == 0 {
					continue;
				}
				if off_x != 0 && off_y != 0 {
					continue;
				}
				buf[count] = (curr_x as usize, curr_y as usize);
				count += 1;
			}
		}
		&buf[0..count]
	}

	fn reachable<'a>(
		grid: &Vec<Vec<Pipe>>,
		(x, y): (usize, usize),
		buf: &'a mut [(usize, usize); 4],
	) -> &'a [(usize, usize)] {
		let mut count: usize = 0;
		let current_pipe = grid[y][x];
		let mut buf2 = [(0usize, 0usize); 4];

		for &(curr_x, curr_y) in Grid::adjacent(grid, (x, y), &mut buf2) {
			let off_x = (curr_x as isize) - (x as isize);
			let off_y = (curr_y as isize) - (y as isize);
			if grid[curr_y][curr_x] == Ground {
				continue;
			}
			if current_pipe == Start {
				let new_pipe = grid[curr_y][curr_x];
				if !new_pipe.connects(-off_x, -off_y) {
					continue;
				}
			} else if !current_pipe.connects(off_x, off_y) {
				continue;
			}
			buf[count] = (curr_x, curr_y);
			count += 1;
		}
		&buf[0..count]
	}

	fn explore(&mut self, (x, y): (usize, usize), distance: i32) -> () {
		self.distances[y][x] = distance;
		let current_pipe = self.grid[y][x];
		let mut buf = [(0usize, 0usize); 4];

		for &(new_x, new_y) in Grid::reachable(&self.grid, (x, y), &mut buf) {
			if current_pipe != Start && self.distances[new_y][new_x] >= 0 {
				if self.distances[new_y][new_x] < distance + 1 {
					continue;
				}
			}

			self.explore((new_x, new_y), distance + 1);
		}
	}

	fn to_blocks(&self) -> Vec<Vec<bool>> {
		let mut out = vec![vec![false; 3 * self.grid[0].len()]; 3 * self.grid.len()];

		for (y, row) in self.grid.iter().enumerate() {
			for (x, pipe) in row.iter().enumerate() {
				if self.distances[y][x] < 0 {
					continue;
				}

				let pixels: [[bool; 3]; 3] = match *pipe {
					Vert => [
						[false, true, false],
						[false, true, false],
						[false, true, false],
					],
					Horiz => [
						[false, false, false],
						[true, true, true],
						[false, false, false],
					],
					Ne => [
						[false, true, false],
						[false, true, true],
						[false, false, false],
					],
					Nw => [
						[false, true, false],
						[true, true, false],
						[false, false, false],
					],
					Sw => [
						[false, false, false],
						[true, true, false],
						[false, true, false],
					],
					Se => [
						[false, false, false],
						[false, true, true],
						[false, true, false],
					],
					Ground => [[false; 3]; 3],
					Start => [
						[false, true, false],
						[true, true, true],
						[false, true, false],
					],
				};

				for (i, p_row) in pixels.iter().enumerate() {
					out[3 * y + i][(3 * x)..(3 * x + 3)].copy_from_slice(p_row);
				}
			}
		}

		return out;
	}

	fn mark_outside(&mut self, blocks: &Vec<Vec<bool>>) -> () {
		let outer_ring_iter = (0..self.distances.len())
			.flat_map(|y| [(0usize, y), (self.distances[0].len() - 1, y)].into_iter())
			.chain(
				(1..(self.distances[0].len() - 1))
					.flat_map(|x| [(x, 0usize), (x, self.distances.len() - 1)].into_iter()),
			);

		for (x, y) in outer_ring_iter {
			if self.distances[y][x] < 0 {
				Grid::mark_outside_from((3 * x + 1, 3 * y + 1), blocks, &mut self.outside);
			}
		}
	}

	fn mark_outside_from(
		(x, y): (usize, usize),
		blocks: &Vec<Vec<bool>>,
		outside: &mut Vec<Vec<bool>>,
	) -> () {
		let mut q = VecDeque::<(usize, usize)>::new();
		q.push_front((x, y));
		let mut buf = [(0usize, 0usize); 4];

		while let Some((x, y)) = q.pop_front() {
			outside[y][x] = true;
			for &(new_x, new_y) in Grid::adjacent(blocks, (x, y), &mut buf) {
				if blocks[new_y][new_x] || outside[new_y][new_x] || q.contains(&(x, y)) {
					// skip tiles that are on the loop
					continue;
				}

				q.push_back((new_x, new_y));
			}
		}
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut grid = Grid::parse(iter).unwrap();
	grid.explore(grid.start, 0);

	grid.mark_outside(&grid.to_blocks());

	let mut num_inside: u64 = 0;

	for (y, row) in grid.grid.iter().enumerate() {
		for (x, _) in row.iter().enumerate() {
			if !grid.outside[3 * y + 1][3 * x + 1] && grid.distances[y][x] < 0 {
				num_inside += 1;
			}
		}
	}

	dbg!(num_inside);

	Ok(())
}
