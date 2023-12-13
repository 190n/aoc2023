use std::io;

fn find_reflection(grid: &[Vec<bool>]) -> usize {
	for column in 0..(grid[0].len() - 1) {
		let left_columns = (0..=column)
			.rev()
			.map(|x| grid.iter().map(move |row| row[x]));
		let right_columns =
			((column + 1)..(grid[0].len())).map(|x| grid.iter().map(move |row| row[x]));

		if left_columns
			.zip(right_columns)
			.all(|(c1, c2)| c1.zip(c2).all(|(v1, v2)| v1 == v2))
		{
			return column + 1;
		}
	}

	for row in 0..(grid.len() - 1) {
		let top_rows = (0..=row).rev().map(|y| &grid[y]);
		let bottom_rows = ((row + 1)..(grid.len())).map(|y| &grid[y]);

		if top_rows.zip(bottom_rows).all(|(c1, c2)| c1 == c2) {
			return 100 * (row + 1);
		}
	}
	unreachable!();
}

fn main() -> io::Result<()> {
	let iter = io::stdin()
		.lines()
		.map(|l| l.unwrap())
		.chain(["".to_string()].into_iter());

	let mut current_grid: Vec<Vec<bool>> = Vec::new();
	let mut sum: usize = 0;

	for line in iter {
		if line.is_empty() {
			let current_sum = find_reflection(&current_grid);
			dbg!(current_sum);
			sum += current_sum;
			current_grid = Vec::new();
		} else {
			let mut row: Vec<bool> = Vec::new();
			for c in line.chars() {
				row.push(match c {
					'#' => true,
					'.' => false,
					_ => panic!("wrong character"),
				});
			}
			current_grid.push(row);
		}
	}

	dbg!(sum);

	Ok(())
}
