use std::{io, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
	Operational,
	Damaged,
	Unknown,
}

use Condition::*;

#[derive(Debug)]
struct Row {
	conditions: Vec<Condition>,
	damaged_groups: Vec<usize>,
}

impl FromStr for Row {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut row = Row {
			conditions: Vec::new(),
			damaged_groups: Vec::new(),
		};
		let mut parts = s.split(' ');
		let conditions_part = if let Some(p) = parts.next() {
			p
		} else {
			return Err("empty");
		};
		let groups_part = if let Some(p) = parts.next() {
			p
		} else {
			return Err("too short");
		};

		for c in conditions_part.chars() {
			match c {
				'#' => row.conditions.push(Damaged),
				'.' => row.conditions.push(Operational),
				'?' => row.conditions.push(Unknown),
				_ => return Err("invalid symbol"),
			}
		}

		for n in groups_part.split(',') {
			row.damaged_groups
				.push(n.parse().map_err(|_| "invalid number")?);
		}

		Ok(row)
	}
}

fn indent(amount: u32) -> () {
	for _ in 0..amount {
		eprint!("    ");
	}
}

impl Row {
	fn assign_operational(
		conditions: &[Condition],
		current_damaged_group: usize,
		damaged_groups: &[usize],
	) -> usize {
		if current_damaged_group == 0 {
			return Row::count_arrangements_internal(
				&conditions[1..],
				damaged_groups[1],
				&damaged_groups[1..],
			);
		} else {
			return Row::count_arrangements_internal(
				&conditions[1..],
				current_damaged_group,
				damaged_groups,
			);
		}
	}

	fn assign_damaged(
		conditions: &[Condition],
		current_damaged_group: usize,
		damaged_groups: &[usize],
	) -> usize {
		if current_damaged_group == 0 {
			return 0;
		}
	}

	fn count_arrangements_internal(
		conditions: &[Condition],
		current_damaged_group: usize,
		damaged_groups: &[usize],
	) -> usize {
		if conditions.len() == 1 && damaged_groups.len() == 1 && current_damaged_group == 0 {
			return 1;
		} else if conditions.len() == 1 || damaged_groups.len() == 1 {
			return 0;
		} else {
			return match conditions[0] {
				Operational => {
					Row::assign_operational(conditions, current_damaged_group, damaged_groups)
				},
				Damaged => Row::assign_damaged(conditions, current_damaged_group, damaged_groups),
				Unknown => {
					Row::assign_operational(conditions, current_damaged_group, damaged_groups)
						+ Row::assign_damaged(conditions, current_damaged_group, damaged_groups)
				},
			};
		}
	}

	pub fn count_arrangements(&self) -> usize {
		return Row::count_arrangements_internal(
			&self.conditions,
			self.damaged_groups[0],
			&self.damaged_groups,
		);
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	for line in iter {
		let row = Row::from_str(&line).unwrap();
		let count = row.count_arrangements();
		eprintln!("========");
		println!("count = {count}");
		eprintln!("========");
	}

	Ok(())
}
