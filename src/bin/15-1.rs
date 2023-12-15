use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
	let iter = io::stdin()
		.bytes()
		.map(|r| r.unwrap())
		.filter(|&b| b != b'\n');

	let mut hash: u16 = 0;
	let mut sum: u64 = 0;

	for b in iter {
		if b == b',' {
			sum += hash as u64;
			hash = 0;
		} else {
			hash += b as u16;
			hash *= 17;
			hash %= 256;
		}
	}

	sum += hash as u64;
	dbg!(sum);

	Ok(())
}
