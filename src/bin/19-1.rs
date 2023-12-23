use std::{collections::HashMap, io, str::FromStr};

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
		value: u32,
		destination: Destination,
	},
	Unconditional(Destination),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Part {
	x: u32,
	m: u32,
	a: u32,
	s: u32,
}

impl Part {
	fn get(&self, property: Property) -> u32 {
		match property {
			Property::X => self.x,
			Property::M => self.m,
			Property::A => self.a,
			Property::S => self.s,
		}
	}
}

impl FromStr for Part {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut numbers = s
			.split(&['=', ',', '}'])
			.filter_map(|s| s.parse::<u32>().ok());
		let x = numbers.next().ok_or("invalid part")?;
		let m = numbers.next().ok_or("invalid part")?;
		let a = numbers.next().ok_or("invalid part")?;
		let s = numbers.next().ok_or("invalid part")?;
		Ok(Part { x, m, a, s })
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
				let value: u32 = rule_desc[2..colon_index]
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
	fn accept(
		&self,
		other_workflows: &HashMap<String, Workflow>,
		part: &Part,
	) -> Result<bool, &'static str> {
		for r in self.rules.iter() {
			match r {
				Rule::Unconditional(d) => match d {
					Destination::Accept => return Ok(true),
					Destination::Reject => return Ok(false),
					Destination::Workflow(name) => {
						return other_workflows
							.get(name)
							.ok_or("destination workflow not found")?
							.accept(other_workflows, part)
					},
				},
				Rule::Compare {
					property,
					comparison,
					value,
					destination,
				} => {
					let part_value = part.get(*property);
					let passes = match comparison {
						Comparison::Greater => part_value > *value,
						Comparison::Less => part_value < *value,
					};

					if passes {
						match destination {
							Destination::Accept => return Ok(true),
							Destination::Reject => return Ok(false),
							Destination::Workflow(name) => {
								return other_workflows
									.get(name)
									.ok_or("destination workflow not found")?
									.accept(other_workflows, part)
							},
						}
					}
				},
			}
		}
		Err("reached end without termination")
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

	let mut sum: u32 = 0;
	let start_workflow = workflows.get("in").unwrap();

	while let Some(line) = iter.next() {
		let p: Part = line.parse().unwrap();
		if start_workflow.accept(&workflows, &p) == Ok(true) {
			sum += p.x + p.m + p.a + p.s;
		}
	}

	dbg!(sum);

	Ok(())
}
