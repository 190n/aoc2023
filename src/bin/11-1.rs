use std::io;

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	let mut space: Vec<Vec<bool>> = Vec::new();
	for l in iter {
		let mut row: Vec<bool> = Vec::new();
		for c in l.chars() {
			row.push(match c {
				'.' => false,
				'#' => true,
				_ => panic!("wrong input char"),
			});
		}
		space.push(row);
	}

	// duplicate rows
	let mut y: usize = 0;
	while y < space.len() {
		if space[y].iter().all(|&g| g == false) {
			space.insert(y, vec![false; space[0].len()]);
			y += 1;
		}
		y += 1;
	}

	// duplicate columns
	let mut x: usize = 0;
	while x < space[0].len() {
		if space.iter().all(|r| r[x] == false) {
			for row in &mut space {
				row.insert(x, false);
			}
			x += 1;
		}
		x += 1;
	}

	let mut galaxies: Vec<(i32, i32)> = Vec::new();
	for (y, row) in space.iter().enumerate() {
		for (x, &g) in row.iter().enumerate() {
			if g {
				galaxies.push((x as i32, y as i32));
			}
		}
	}

	let mut sum_distances: i32 = 0;

	for (i, &(x1, y1)) in galaxies.iter().enumerate() {
		for &(x2, y2) in &galaxies[i + 1..] {
			let distance = (x2 - x1).abs() + (y2 - y1).abs();
			sum_distances += distance;
		}
	}

	dbg!(sum_distances);
	Ok(())
}
