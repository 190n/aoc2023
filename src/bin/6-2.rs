use std::io;

fn calculate_distance(total_time: u64, time_held: u64) -> u64 {
	total_time * time_held - (time_held * time_held)
}

fn main() -> io::Result<()> {
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let times_line = iter.next().unwrap();
	assert!(times_line.starts_with("Time:"));
	let time: u64 = times_line
		.split(':')
		.nth(1)
		.unwrap()
		.split_ascii_whitespace()
		.fold(String::new(), |mut acc, part| {
			acc += part;
			return acc;
		})
		.parse()
		.unwrap();

	let distances_line = iter.next().unwrap();
	assert!(distances_line.starts_with("Distance:"));
	let distance: u64 = distances_line
		.split(':')
		.nth(1)
		.unwrap()
		.split_ascii_whitespace()
		.fold(String::new(), |mut acc, part| {
			acc += part;
			return acc;
		})
		.parse()
		.unwrap();

	let mut range = (time / 2)..=(time / 2);
	while calculate_distance(time, *range.start() - 1) > distance {
		range = (range.start() - 1)..=(*range.end());
	}
	while calculate_distance(time, *range.end() + 1) > distance {
		range = (*range.start())..=(range.end() + 1);
	}
	let answer = range.count();

	dbg!(answer);
	Ok(())
}
