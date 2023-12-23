use std::{collections::HashMap, io, ops::RangeInclusive, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Property {
	X,
	M,
	A,
	S,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Comparison {
	Greater,
	Less,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Destination {
	Accept,
	Reject,
	Workflow(String),
}

impl Destination {
	fn count_accepted(
		&self,
		workflows: &HashMap<String, Workflow>,
		ranges: Ranges,
	) -> Result<u64, &'static str> {
		match self {
			Destination::Accept => Ok(ranges.count()),
			Destination::Reject => Ok(0),
			Destination::Workflow(name) => workflows
				.get(name)
				.ok_or("workflow not found")?
				.count_accepted(workflows, ranges),
		}
	}
}

impl FromStr for Destination {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Destination::Accept),
			"R" => Ok(Destination::Reject),
			workflow_name => Ok(Destination::Workflow(workflow_name.to_string())),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Rule {
	Compare {
		property: Property,
		comparison: Comparison,
		value: u64,
		destination: Destination,
	},
	Unconditional(Destination),
}

impl Rule {
	fn count_accepted(
		&self,
		workflows: &HashMap<String, Workflow>,
		ranges: Ranges,
		next: &[Rule],
	) -> Result<u64, &'static str> {
		match self {
			Rule::Unconditional(dest) => return dest.count_accepted(workflows, ranges),
			Rule::Compare {
				property,
				comparison,
				value,
				destination,
			} => {
				let (if_true, if_false) = if *comparison == Comparison::Greater {
					// suppose condition is x > 500, and X range is currently 0..=1000
					// for "if true", we pass 501..=1000 into the target workflow
					// for "if false", we follow the rest of this workflow's rules with 0..=500
					let if_true = ranges.intersect(*property, (*value + 1)..=4000);
					let if_false = ranges.intersect(*property, 0..=(*value));
					(if_true, if_false)
				} else {
					let if_true = ranges.intersect(*property, 0..=(*value - 1));
					let if_false = ranges.intersect(*property, (*value)..=4000);
					(if_true, if_false)
				};
				return Ok(destination.count_accepted(workflows, if_true)?
					+ next[0].count_accepted(workflows, if_false, &next[1..])?);
			},
		}
	}
}

#[derive(Clone, Debug)]
struct Ranges {
	x: RangeInclusive<u64>,
	m: RangeInclusive<u64>,
	a: RangeInclusive<u64>,
	s: RangeInclusive<u64>,
}

impl Default for Ranges {
	fn default() -> Self {
		Ranges {
			x: 1..=4000,
			m: 1..=4000,
			a: 1..=4000,
			s: 1..=4000,
		}
	}
}

impl Ranges {
	fn intersect_one(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> RangeInclusive<u64> {
		// https://scicomp.stackexchange.com/a/26260
		if *b.start() > *a.end() || *a.start() > *b.end() {
			// empty
			return 1..=0;
		} else {
			let start = *a.start().max(b.start());
			let end = *a.end().min(b.end());
			return start..=end;
		}
	}

	fn intersect(&self, property: Property, other: RangeInclusive<u64>) -> Ranges {
		let mut new_ranges = self.clone();
		match property {
			Property::X => new_ranges.x = Ranges::intersect_one(&self.x, &other),
			Property::M => new_ranges.m = Ranges::intersect_one(&self.m, &other),
			Property::A => new_ranges.a = Ranges::intersect_one(&self.a, &other),
			Property::S => new_ranges.s = Ranges::intersect_one(&self.s, &other),
		}
		return new_ranges;
	}

	fn count(&self) -> u64 {
		let mut total: u64 = 1;
		for range in [&self.x, &self.m, &self.a, &self.s] {
			total *= *range.end() - *range.start() + 1;
		}
		return total;
	}
}

#[derive(Debug)]
struct Workflow {
	name: String,
	rules: Vec<Rule>,
}

impl FromStr for Workflow {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (name_ref, rest) = s.split_once('{').ok_or("empty input")?;
		let name = name_ref.to_string();
		let rules_string = &rest[0..rest.find('}').ok_or("no closing brace")?];
		let mut rules: Vec<Rule> = Vec::new();

		for rule_desc in rules_string.split(',') {
			if let Some(colon_index) = rule_desc.find(':') {
				// part after colon = destination
				let destination = Destination::from_str(&rule_desc[colon_index + 1..]).unwrap();
				let property = match &rule_desc[0..1] {
					"x" => Property::X,
					"m" => Property::M,
					"a" => Property::A,
					"s" => Property::S,
					_ => return Err("invalid property in comparison"),
				};
				let comparison = match &rule_desc[1..2] {
					">" => Comparison::Greater,
					"<" => Comparison::Less,
					_ => return Err("invalid comparison"),
				};
				let value: u64 = rule_desc[2..colon_index]
					.parse()
					.map_err(|_| "invalid number")?;
				rules.push(Rule::Compare {
					property,
					comparison,
					value,
					destination,
				});
			} else {
				// unconditional
				rules.push(Rule::Unconditional(rule_desc.parse().unwrap()));
			}
		}

		Ok(Workflow { name, rules })
	}
}

impl Workflow {
	fn count_accepted(
		&self,
		workflows: &HashMap<String, Workflow>,
		ranges: Ranges,
	) -> Result<u64, &'static str> {
		return self.rules[0].count_accepted(workflows, ranges, &self.rules[1..]);
	}
}

fn main() -> io::Result<()> {
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let mut workflows: HashMap<String, Workflow> = HashMap::new();

	while let Some(line) = iter.next() {
		if line.is_empty() {
			break;
		}

		let workflow = Workflow::from_str(&line).unwrap();
		workflows.insert(workflow.name.clone(), workflow);
	}

	let start_workflow = workflows.get("in").unwrap();
	let num_accepted = start_workflow
		.count_accepted(&workflows, Ranges::default())
		.unwrap();
	dbg!(num_accepted);

	Ok(())
}
