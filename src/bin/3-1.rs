#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::io::{self};

fn get_char_or_dot(grid: &Vec<Vec<u8>>, y: isize, x: isize) -> u8 {
	if y < 0 || (y as usize) >= grid.len() {
		return b'.';
	} else if x < 0 || (x as usize) >= grid[y as usize].len() {
		return b'.';
	} else {
		return grid[y as usize][x as usize];
	}
}

fn has_adjacent_symbols(grid: &Vec<Vec<u8>>, y: usize, x_range: (usize, usize)) -> bool {
	let y = y as isize;
	let x_range = (x_range.0 as isize, x_range.1 as isize);

	for x in (x_range.0 - 1)..(x_range.1 + 1) {
		if get_char_or_dot(grid, y - 1, x) != b'.' {
			return true;
		} else if get_char_or_dot(grid, y + 1, x) != b'.' {
			return true;
		}
	}

	if get_char_or_dot(grid, y, x_range.0 - 1) != b'.' {
		return true;
	} else if get_char_or_dot(grid, y, x_range.1) != b'.' {
		return true;
	}

	return false;
}

fn main() -> io::Result<()> {
	let mut lines: Vec<Vec<u8>> = Vec::new();
	for line in std::io::stdin().lines() {
		lines.push(line?.as_bytes().to_owned());
	}

	let mut sum = 0u64;

	for y in 0..lines.len() {
		let mut x = 0;
		while x < lines[y].len() {
			if lines[y][x].is_ascii_digit() {
				let slice_start = x;
				while x < lines[y].len() && lines[y][x].is_ascii_digit() {
					x += 1;
				}

				if has_adjacent_symbols(&lines, y, (slice_start, x)) {
					let slice = &lines[y][slice_start..x];
					let part_num: u64 = std::str::from_utf8(slice).unwrap().parse().unwrap();
					sum += part_num;
				}

				continue;
			}
			x += 1;
		}
	}

	dbg!(sum);
	Ok(())
}
