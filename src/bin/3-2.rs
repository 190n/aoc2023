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

fn parse_number_around(grid: &Vec<Vec<u8>>, y: isize, x: isize) -> u64 {
	let mut min_x = x;
	let mut max_x = x;
	while get_char_or_dot(grid, y, min_x - 1).is_ascii_digit() {
		min_x -= 1;
	}
	while get_char_or_dot(grid, y, max_x + 1).is_ascii_digit() {
		max_x += 1;
	}

	let slice = &grid[y as usize][min_x as usize..(max_x + 1) as usize];
	return std::str::from_utf8(slice).unwrap().parse().unwrap();
}

fn product_adjacent_numbers(grid: &Vec<Vec<u8>>, y: isize, x: isize) -> u64 {
	let mut product: u64 = 1;
	let mut count_nums = 0;

	if get_char_or_dot(grid, y - 1, x).is_ascii_digit() {
		count_nums += 1;
		product *= parse_number_around(grid, y - 1, x);
	} else {
		if get_char_or_dot(grid, y - 1, x - 1).is_ascii_digit() {
			count_nums += 1;
			product *= parse_number_around(grid, y - 1, x - 1);
		}
		if get_char_or_dot(grid, y - 1, x + 1).is_ascii_digit() {
			count_nums += 1;
			product *= parse_number_around(grid, y - 1, x + 1);
		}
	}

	if get_char_or_dot(grid, y + 1, x).is_ascii_digit() {
		count_nums += 1;
		product *= parse_number_around(grid, y + 1, x);
	} else {
		if get_char_or_dot(grid, y + 1, x - 1).is_ascii_digit() {
			count_nums += 1;
			product *= parse_number_around(grid, y + 1, x - 1);
		}
		if get_char_or_dot(grid, y + 1, x + 1).is_ascii_digit() {
			count_nums += 1;
			product *= parse_number_around(grid, y + 1, x + 1);
		}
	}

	if get_char_or_dot(grid, y, x - 1).is_ascii_digit() {
		count_nums += 1;
		product *= parse_number_around(grid, y, x - 1);
	}
	if get_char_or_dot(grid, y, x + 1).is_ascii_digit() {
		count_nums += 1;
		product *= parse_number_around(grid, y, x + 1);
	}

	return if count_nums == 2 { product } else { 0 };
}

fn main() -> io::Result<()> {
	let mut lines: Vec<Vec<u8>> = Vec::new();
	for line in std::io::stdin().lines() {
		lines.push(line?.as_bytes().to_owned());
	}

	let mut sum = 0u64;

	for y in 0..lines.len() {
		for x in 0..lines[y].len() {
			if lines[y][x] == b'*' {
				sum += product_adjacent_numbers(&lines, y as isize, x as isize);
			}
		}
	}

	dbg!(sum);
	Ok(())
}
