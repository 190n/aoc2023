#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::io::{self};

#[derive(Debug)]
struct Mapping {
	dst_start: u32,
	src_start: u32,
	len: u32,
}

#[derive(Debug)]
struct Mapper {
	mappings: Vec<Mapping>,
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
				.map(|t| t.parse::<u32>().unwrap())
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

	pub fn map(&self, input: u32) -> u32 {
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

	pub fn map_many(mappers: &[&Mapper], input: u32) -> u32 {
		let mut current = input;
		for m in mappers {
			current = m.map(current);
		}
		return current;
	}
}

fn main() -> io::Result<()> {
	let mut sum: u32 = 0;
	let mut iter = io::stdin().lines().map(|l| l.unwrap());

	let seeds_line = iter.next().unwrap();
	let seeds: Vec<u32> = seeds_line
		.split("seeds: ")
		.nth(1)
		.unwrap()
		.split(' ')
		.map(|t| t.parse::<u32>().unwrap())
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

	let all_maps = &[
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
		.map(|s| Mapper::map_many(all_maps, *s))
		.min()
		.unwrap();
	dbg!(lowest_location);

	Ok(())
}
