#![feature(array_windows)]
use std::io;

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut sum: i64 = 0;

	for line in iter {
		let readings: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
		let mut difference_tracks: Vec<Vec<i64>> = vec![readings];

		// calculate differences from input
		while !difference_tracks.last().unwrap().iter().all(|&d| d == 0) {
			let new_differences: Vec<i64> = difference_tracks
				.last()
				.unwrap()
				.array_windows::<2>()
				.map(|[a, b]| b - a)
				.collect();
			difference_tracks.push(new_differences);
		}

		// propagate upward
		difference_tracks.last_mut().unwrap().push(0);
		for i in (1..difference_tracks.len()).rev() {
			let upper = &difference_tracks[i - 1];
			let lower = &difference_tracks[i];
			let new_term = lower.last().unwrap() + upper.last().unwrap();
			difference_tracks[i - 1].push(new_term);
		}
		sum += difference_tracks[0].last().unwrap();
	}

	dbg!(sum);
	Ok(())
}
