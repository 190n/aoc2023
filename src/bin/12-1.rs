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
	fn count_arrangements_internal(
		conditions: &[Condition],
		damaged_groups: &[usize],
		depth: u32,
	) -> usize {
		if damaged_groups.len() == 0 {
			return 1;
		}

		indent(depth);
		for &c in conditions {
			eprint!(
				"{}",
				match c {
					Operational => ".",
					Damaged => "#",
					Unknown => "?",
				}
			);
		}
		eprint!("\n");

		assert!(conditions[0] == Operational);
		assert!(conditions[conditions.len() - 1] == Operational);
		let mut n: usize = 0;
		let mut working_set: Vec<Condition> = conditions.to_owned();
		let group_len = damaged_groups[0];
		let mut satisfied = false;
		// look for all places to put the first damaged group
		for i in 1..(working_set.len() - 1) {
			if satisfied {
				return n;
			}

			match working_set[i] {
				Operational => {},
				Damaged => {
					if working_set[i - 1] != Operational {
						continue;
					}
					indent(depth);
					eprintln!("looking for match starting at {i}");
					let mut j = i;
					while working_set[j + 1] == Damaged {
						j += 1;
					}
					if j - i + 1 == group_len {
						indent(depth);
						eprintln!("{group_len}-long group is matched by {i} thru {j}");
						// group is already satisfied
						n += Row::count_arrangements_internal(
							&working_set[(j + 1)..],
							&damaged_groups[1..],
							depth + 1,
						);
					}
					satisfied = true;
				},
				Unknown => {
					let mut span = i..(i + 1);
					while working_set[span.start - 1] != Operational {
						span = (span.start - 1)..(span.end);
					}
					while working_set[span.end] != Operational {
						span = (span.start)..(span.end + 1);
					}

					indent(depth);
					eprintln!("{} {} {}", span.start, i, span.end);

					if (i..span.end).len() < group_len {
						continue;
					}

					indent(depth);
					eprint!("found span {span:?}. new row = ");

					for j in (span.start)..i {
						working_set[j] = Operational;
					}
					for j in (span.start)..(span.start + group_len) {
						working_set[j] = Damaged;
					}
					working_set[i + group_len] = Operational;

					for (i, c) in working_set.iter().enumerate() {
						if i == span.start {
							eprint!("[");
						}
						eprint!(
							"{}",
							match c {
								Operational => ".",
								Damaged => "#",
								Unknown => "?",
							}
						);
						if i == span.end - 1 {
							eprint!("]");
						}
					}
					eprint!("\n");
					// dbg!(i, span.start, span.end);
					indent(depth);
					eprintln!("recursive call starting from index {}", i + group_len);
					n += Row::count_arrangements_internal(
						&working_set[(i + group_len)..],
						&damaged_groups[1..],
						depth + 1,
					);
					working_set.copy_from_slice(conditions);
				},
			}
		}
		indent(depth);
		eprintln!("RETURN: {n}");
		return n;
	}

	pub fn count_arrangements(&self) -> usize {
		// pad the conditions with working springs so end conditions are easier
		let mut padded_conditions = Vec::<Condition>::with_capacity(self.conditions.len() + 2);
		padded_conditions.push(Operational);
		padded_conditions.extend(&self.conditions);
		padded_conditions.push(Operational);
		Row::count_arrangements_internal(&padded_conditions, &self.damaged_groups, 0)
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
