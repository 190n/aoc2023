#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(map_try_insert)]

use std::{
	collections::HashMap,
	io::{self},
};

fn main() -> io::Result<()> {
	let mut cards: u32 = 0;
	let mut copies: HashMap<u32, u32> = HashMap::new();
	let mut card: u32 = 1;

	for line in std::io::stdin().lines() {
		let _ = copies.try_insert(card, 1);
		let this_card_copies = *copies.get(&card).unwrap();

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

		let mut num_to_copy: u32 = 0;
		for n in have.split(' ').filter(|n| !n.is_empty()) {
			let n: u32 = n.parse().unwrap();
			if winning.contains(&n) {
				num_to_copy += 1;
			}
		}

		for i in 0..num_to_copy {
			let copying_card = card + i + 1;
			if let Some(update_copies) = copies.get_mut(&copying_card) {
				*update_copies += this_card_copies;
			} else {
				copies.insert(copying_card, 1 + this_card_copies);
			}
		}

		card += 1;
		cards += this_card_copies;
	}

	dbg!(cards);
	Ok(())
}
