use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io;

#[derive(Eq, PartialEq, Hash)]
struct Node {
	name: [u8; 3],
	left: [u8; 3],
	right: [u8; 3],
}

impl Node {
	pub fn parse(line: &str) -> Node {
		let bytes = line.as_bytes();
		return Node {
			name: [bytes[0], bytes[1], bytes[2]],
			left: [bytes[7], bytes[8], bytes[9]],
			right: [bytes[12], bytes[13], bytes[14]],
		};
	}

	pub fn follow<'a>(
		&'a self,
		instructions: &'a str,
		start_index: usize,
		world: &'a HashMap<[u8; 3], Node>,
	) -> Follower<'a> {
		return Follower {
			instructions,
			world,
			current_node: self,
			current_index: start_index,
		};
	}
}

struct Follower<'a> {
	instructions: &'a str,
	current_node: &'a Node,
	current_index: usize,
	world: &'a HashMap<[u8; 3], Node>,
}

impl<'a> Iterator for Follower<'a> {
	type Item = (usize, &'a Node);

	fn next(&mut self) -> Option<Self::Item> {
		let inst = self.instructions.as_bytes()[self.current_index];
		self.current_index = (self.current_index + 1) % self.instructions.len();
		match inst {
			b'L' => self.current_node = self.world.get(&self.current_node.left).unwrap(),
			b'R' => self.current_node = self.world.get(&self.current_node.right).unwrap(),
			_ => {},
		}
		return Some((self.current_index, self.current_node));
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{name} = ({left}, {right})",
			name = std::str::from_utf8(&self.name).unwrap(),
			left = std::str::from_utf8(&self.left).unwrap(),
			right = std::str::from_utf8(&self.right).unwrap(),
		)
	}
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let t = b;
		b = a % b;
		a = t;
	}
	return a;
}

fn lcm(a: u64, b: u64) -> u64 {
	return a * b / gcd(a, b);
}

fn main() -> io::Result<()> {
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let mut nodes: HashMap<[u8; 3], Node> = HashMap::new();

	let instructions = iter.next().unwrap();
	let _ = iter.next().unwrap();

	for line in iter {
		let node = Node::parse(&line);
		nodes.insert(node.name, node);
	}

	let starts = nodes.values().filter(|n| n.name[2] == b'A');

	let mut loop_sizes: Vec<u64> = Vec::new();

	'start_nodes: for s in starts {
		let mut seen: HashSet<(usize, &Node)> = HashSet::new();
		for (i, n) in s.follow(&instructions, 0, &nodes) {
			if !seen.insert((i, n)) && n.name[2] == b'Z' {
				let mut loop_contents = vec![n];
				for (j, n2) in n.follow(&instructions, i, &nodes) {
					if n == n2 && j == i {
						loop_sizes.push(loop_contents.len() as u64);
						continue 'start_nodes;
					}
					loop_contents.push(n2);
				}
			}
		}
	}

	dbg!(loop_sizes.into_iter().reduce(lcm).unwrap());

	Ok(())
}
