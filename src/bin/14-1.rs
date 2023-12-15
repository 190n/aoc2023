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

	roll_north(&mut grid);
	dbg!(count_load(&grid));

	Ok(())
}
