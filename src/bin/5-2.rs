#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::io::{self};

#[derive(Debug)]
struct Mapping {
	dst_start: u64,
	src_start: u64,
	len: u64,
}

#[derive(Debug)]
struct Mapper {
	mappings: Vec<Mapping>,
}

struct SeedRange {
	start: u64,
	len: u64,
}

impl Mapper {
	pub fn build(lines: &mut impl Iterator<Item = String>) -> Mapper {
		let mut map = Mapper {
			mappings: Vec::new(),
		};
		for l in lines {
			if l.is_empty() {
				return map;
			}

			let [dst_start, src_start, len] = l
				.split(' ')
				.map(|t| t.parse::<u64>().unwrap())
				.array_chunks::<3>()
				.next()
				.unwrap();
			map.mappings.push(Mapping {
				dst_start,
				src_start,
				len,
			});
		}
		map
	}

	pub fn map(&self, input: u64) -> u64 {
		for map in &self.mappings {
			if input >= map.src_start {
				let offset = input - map.src_start;
				if offset < map.len {
					return map.dst_start + offset;
				}
			}
		}
		return input;
	}

	pub fn map_range(&self, start: u64, len: u64) -> Vec<(u64, u64)> {
		let mut output = Vec::<(u64, u64)>::new();
		for map in &self.mappings {
			if start >= map.src_start && start + len <= map.src_start + map.len {
				// totally contained
				output.push((map.dst_start + (start - map.src_start), len));
			} else if start >= map.src_start && start < map.src_start + map.len {
				// input goes higher than range of mapping
				// input       (5, 6, 7, 8, 9, 10, 11, 12)
				// map   (3, 4, 5, 6, 7, 8) -> (20, 21, 22, 23, 24, 25)
				// output should be (22, 23, 24, 25)
				dbg!(map, start, len);
				let common_len = len - (map.src_start + map.len);
				output.push((map.dst_start + map.len - common_len, common_len));
			} else if start + len >= map.src_start && start + len < map.src_start + map.len {
				// input starts before mapping
				// input (1, 2, 3, 4, 5, 6)
				// map         (3, 4, 5, 6, 7, 8) -> (20, 21, 22, 23, 24, 25)
				// output should be (20, 21, 22, 23)
				let common_len = start + len - map.src_start;
				output.push((map.dst_start, common_len))
			}
		}
		return output;
	}

	pub fn map_many(mappers: &[&Mapper], input: u64) -> u64 {
		let mut current = input;
		for m in mappers {
			current = m.map(current);
		}
		return current;
	}

	pub fn map_many_range(mappers: &[&Mapper], start: u64, len: u64) -> Vec<(u64, u64)> {
		let mut current = vec![(start, len)];
		for m in mappers {
			current = current
				.iter()
				.map(|(start, len)| m.map_range(*start, *len))
				.fold(Vec::new(), |mut acc, e| {
					acc.extend(e);
					acc
				});
		}
		return current;
	}
}

fn main() -> io::Result<()> {
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let seeds_line = iter.next().unwrap();
	let seeds: Vec<SeedRange> = seeds_line
		.split("seeds: ")
		.nth(1)
		.unwrap()
		.split(' ')
		.map(|t| t.parse::<u64>().unwrap())
		.array_chunks::<2>()
		.map(|a| SeedRange {
			start: a[0],
			len: a[1],
		})
		.collect();

	// empty
	let _ = iter.next().unwrap();
	// seed-to-soil map:
	assert!(iter.next().unwrap() == "seed-to-soil map:");
	let seed_to_soil = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "soil-to-fertilizer map:");
	let soil_to_fertilizer = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "fertilizer-to-water map:");
	let fertilizer_to_water = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "water-to-light map:");
	let water_to_light = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "light-to-temperature map:");
	let light_to_temperature = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "temperature-to-humidity map:");
	let temperature_to_humidity = Mapper::build(&mut iter);
	assert!(iter.next().unwrap() == "humidity-to-location map:");
	let humidity_to_location = Mapper::build(&mut iter);

	let all_maps = [
		&seed_to_soil,
		&soil_to_fertilizer,
		&fertilizer_to_water,
		&water_to_light,
		&light_to_temperature,
		&temperature_to_humidity,
		&humidity_to_location,
	];

	let lowest_location = seeds
		.iter()
		.map(|seed_range| {
			eprintln!("processing {} seeds", seed_range.len);
			let range = (seed_range.start)..(seed_range.start + seed_range.len);
			range
				.into_iter()
				.map(|s| Mapper::map_many(&all_maps, s))
				.min()
				.unwrap()
		})
		.min()
		.unwrap();
	dbg!(lowest_location);

	Ok(())
}
