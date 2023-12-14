use std::io;

fn count_different(c1: impl Iterator<Item = bool>, c2: impl Iterator<Item = bool>) -> usize {
	c1.zip(c2)
		.map(|(v1, v2)| if v1 == v2 { 0 } else { 1 })
		.sum()
}

fn find_reflection(grid: &[Vec<bool>]) -> usize {
	for column in 0..(grid[0].len() - 1) {
		let left_columns = (0..=column)
			.rev()
			.map(|x| grid.iter().map(move |row| row[x]));
		let right_columns =
			((column + 1)..(grid[0].len())).map(|x| grid.iter().map(move |row| row[x]));

		if left_columns
			.zip(right_columns)
			.map(|(c1, c2)| count_different(c1, c2))
			.sum::<usize>()
			== 1
		{
			return column + 1;
		}
	}

	for row in 0..(grid.len() - 1) {
		let top_rows = (0..=row).rev().map(|y| &grid[y]);
		let bottom_rows = ((row + 1)..(grid.len())).map(|y| &grid[y]);

		if top_rows
			.zip(bottom_rows)
			.map(|(c1, c2)| count_different(c1.iter().map(|r| *r), c2.iter().map(|r| *r)))
			.sum::<usize>()
			== 1
		{
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
