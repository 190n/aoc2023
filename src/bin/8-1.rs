use std::collections::HashMap;
use std::fmt::Debug;
use std::io;

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

fn main() -> io::Result<()> {
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let mut nodes: HashMap<[u8; 3], Node> = HashMap::new();

	let instructions = iter.next().unwrap();
	let _ = iter.next().unwrap();

	for line in iter {
		let node = Node::parse(&line);
		nodes.insert(node.name, node);
	}

	let mut current = nodes.get(&[b'A', b'A', b'A']).unwrap();
	let mut steps: u32 = 0;

	loop {
		for c in instructions.chars() {
			match c {
				'L' => current = nodes.get(&current.left).unwrap(),
				'R' => current = nodes.get(&current.right).unwrap(),
				_ => {},
			}
			steps += 1;
			if current.name == [b'Z', b'Z', b'Z'] {
				dbg!(steps);
				return Ok(());
			}
		}
	}
}
