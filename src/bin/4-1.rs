#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::io::{self};

fn main() -> io::Result<()> {
	let mut sum: u32 = 0;

	for line in std::io::stdin().lines() {
		let line = line?;
		let numbers = line.split(':').nth(1).unwrap();
		let mut sep = numbers.split('|');
		let winning: Vec<u32> = sep
			.next()
			.unwrap()
			.split(' ')
			.filter(|n| !n.is_empty())
			.map(|n| n.parse::<u32>().unwrap())
			.collect();
		let have = sep.next().unwrap();

		let mut value: u32 = 0;

		for n in have.split(' ').filter(|n| !n.is_empty()) {
			let n: u32 = n.parse().unwrap();
			if winning.contains(&n) {
				value = if value == 0 { 1 } else { 2 * value };
			}
		}

		sum += value;
	}

	dbg!(sum);
	Ok(())
}
