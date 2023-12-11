use std::io;

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	// let mut space: Vec<Vec<bool>> = Vec::new();
	let mut galaxies: Vec<(i64, i64)> = Vec::new();
	let mut width: usize = 0;
	let mut height: usize = 0;
	for (y, l) in iter.enumerate() {
		height += 1;
		for (x, c) in l.chars().enumerate() {
			if c == '#' {
				galaxies.push((x as i64, y as i64));
			}
			if height == 1 {
				width += 1;
			}
		}
	}

	const NEW_ROWS: i64 = 1_000_000 - 1;

	// duplicate rows
	let mut y: i64 = 0;
	while y < (height as i64) {
		if galaxies.iter().all(|&(_, gy)| gy != y) {
			for (_, gy) in galaxies.iter_mut() {
				if *gy > y {
					*gy += NEW_ROWS;
				}
			}
			y += NEW_ROWS;
			height += NEW_ROWS as usize;
		}
		y += 1;
	}

	// duplicate columns
	let mut x: i64 = 0;
	while x < (width as i64) {
		if galaxies.iter().all(|&(gx, _)| gx != x) {
			for (gx, _) in galaxies.iter_mut() {
				if *gx > x {
					*gx += NEW_ROWS;
				}
			}
			x += NEW_ROWS + 1;
			width += NEW_ROWS as usize;
		}
		x += 1;
	}

	let mut sum_distances: i64 = 0;

	for (i, &(x1, y1)) in galaxies.iter().enumerate() {
		for &(x2, y2) in &galaxies[i + 1..] {
			let distance = (x2 - x1).abs() + (y2 - y1).abs();
			sum_distances += distance;
		}
	}

	dbg!(sum_distances);
	Ok(())
}
