use std::io;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
	label: String,
	focal_length: u8,
}

fn main() -> io::Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

	for step in input.split(',').map(|s| s.trim_end()) {
		let mut hash: usize = 0;

		let label = step.split(&['-', '=']).nth(0).unwrap();
		for b in label.bytes() {
			hash += b as usize;
			hash *= 17;
			hash %= 256;
		}

		if step.ends_with('-') {
			// remove given lens
			if let Some(index) = boxes[hash].iter().position(|l| l.label == label) {
				boxes[hash].remove(index);
			}
		} else {
			let focal_length = *step.as_bytes().last().unwrap() - b'0';
			if let Some(lens) = boxes[hash].iter_mut().find(|l| l.label == label) {
				lens.focal_length = focal_length;
			} else {
				boxes[hash].push(Lens {
					label: label.to_string(),
					focal_length,
				});
			}
		}
	}

	let mut sum: usize = 0;
	for (i, contents) in boxes.iter().enumerate() {
		for (j, lens) in contents.iter().enumerate() {
			let focusing_power = (i + 1) * (j + 1) * lens.focal_length as usize;
			sum += focusing_power;
		}
	}

	dbg!(sum);

	Ok(())
}
