use std::{fmt::Debug, io};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridPos {
	Empty,
	Rounded,
	Cube,
}

use GridPos::*;

impl Debug for GridPos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Empty => '.',
				Rounded => 'O',
				Cube => '#',
			}
		)
	}
}

impl TryFrom<char> for GridPos {
	type Error = &'static str;
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'.' => Ok(Empty),
			'O' => Ok(Rounded),
			'#' => Ok(Cube),
			_ => Err("invalid character"),
		}
	}
}

fn roll_north(grid: &mut Vec<Vec<GridPos>>) -> () {
	let mut rolled = true;
	while rolled {
		rolled = false;
		for y in (1..grid.len()).rev() {
			for x in 0..grid[0].len() {
				if grid[y][x] != Rounded {
					continue;
				}
				if grid[y - 1][x] == Empty {
					grid[y - 1][x] = Rounded;
					grid[y][x] = Empty;
					rolled = true;
				}
			}
		}
	}
}

fn roll_west(grid: &mut Vec<Vec<GridPos>>) -> () {
	let mut rolled = true;
	while rolled {
		rolled = false;
		for x in (1..grid[0].len()).rev() {
			for y in 0..grid.len() {
				if grid[y][x] != Rounded {
					continue;
				}
				if grid[y][x - 1] == Empty {
					grid[y][x - 1] = Rounded;
					grid[y][x] = Empty;
					rolled = true;
				}
			}
		}
	}
}

fn roll_south(grid: &mut Vec<Vec<GridPos>>) -> () {
	let mut rolled = true;
	while rolled {
		rolled = false;
		for y in 0..(grid.len() - 1) {
			for x in 0..grid.len() {
				if grid[y][x] != Rounded {
					continue;
				}
				if grid[y + 1][x] == Empty {
					grid[y + 1][x] = Rounded;
					grid[y][x] = Empty;
					rolled = true;
				}
			}
		}
	}
}

fn roll_east(grid: &mut Vec<Vec<GridPos>>) -> () {
	let mut rolled = true;
	while rolled {
		rolled = false;
		for x in 0..(grid[0].len() - 1) {
			for y in 0..grid.len() {
				if grid[y][x] != Rounded {
					continue;
				}
				if grid[y][x + 1] == Empty {
					grid[y][x + 1] = Rounded;
					grid[y][x] = Empty;
					rolled = true;
				}
			}
		}
	}
}

fn spin_cycle(grid: &mut Vec<Vec<GridPos>>) -> () {
	roll_north(grid);
	roll_west(grid);
	roll_south(grid);
	roll_east(grid);
}

fn display_grid(grid: &Vec<Vec<GridPos>>) -> () {
	for row in grid {
		for rock in row {
			print!("{:?}", rock);
		}
		print!("\n");
	}
}

fn count_load(grid: &Vec<Vec<GridPos>>) -> usize {
	let mut load: usize = 0;
	for (y, row) in grid.iter().enumerate() {
		for rock in row.iter() {
			if *rock == Rounded {
				load += grid.len() - y;
			}
		}
	}
	return load;
}

fn split_pattern_from_end<'a>(
	slice: &'a [usize],
	pattern_length: usize,
	count: usize,
) -> &'a [usize] {
	&slice[slice.len() - pattern_length * (1 + count)..slice.len() - pattern_length * count]
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut grid: Vec<Vec<GridPos>> = Vec::new();

	for line in iter {
		let mut row: Vec<GridPos> = Vec::new();
		for c in line.chars() {
			row.push(GridPos::try_from(c).unwrap());
		}
		grid.push(row);
	}

	let mut loads: Vec<usize> = Vec::new();

	loop {
		spin_cycle(&mut grid);
		let load = count_load(&grid);
		loads.push(load);

		for pattern_length in 2..100 {
			if loads.len() < 3 * pattern_length {
				continue;
			}

			if split_pattern_from_end(&loads, pattern_length, 0)
				== split_pattern_from_end(&loads, pattern_length, 1)
				&& split_pattern_from_end(&loads, pattern_length, 1)
					== split_pattern_from_end(&loads, pattern_length, 2)
			{
				let pattern = split_pattern_from_end(&loads, pattern_length, 0);
				let offset = loads.len() % pattern_length;
				const TARGET_CYCLE: usize = 1_000_000_000;
				let load_after_billion = pattern[(TARGET_CYCLE - offset - 1) % pattern.len()];
				dbg!(load_after_billion);
				return Ok(());
			}
		}
	}
}
