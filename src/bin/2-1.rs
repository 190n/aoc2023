#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::io::{self};

fn main() -> io::Result<()> {
	let mut sum_game_ids: u32 = 0;

	for line in std::io::stdin().lines() {
		let line = line?;

		let [game, reveals] = line.split(": ").array_chunks().next().unwrap();
		let game_id: u32 = game[5..].parse().unwrap();

		let mut max_red: u32 = 0;
		let mut max_green: u32 = 0;
		let mut max_blue: u32 = 0;

		for reveal in reveals.split("; ") {
			for num_and_color in reveal.split(", ") {
				let [num, color] = num_and_color.split(' ').array_chunks().next().unwrap();
				let num: u32 = num.parse().unwrap();

				match color {
					"red" => max_red = max_red.max(num),
					"green" => max_green = max_green.max(num),
					"blue" => max_blue = max_blue.max(num),
					_ => {},
				}
			}
		}

		if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
			sum_game_ids += game_id;
		}
	}

	println!("{sum_game_ids}");
	Ok(())
}
