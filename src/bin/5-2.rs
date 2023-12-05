#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::{
	io::{self},
	sync::Arc,
	thread::JoinHandle,
};

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

#[derive(Copy, Clone)]
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

	pub fn map_many(mappers: &[Mapper], input: u64) -> u64 {
		let mut current = input;
		for m in mappers {
			current = m.map(current);
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

	let all_maps = Arc::new([
		seed_to_soil,
		soil_to_fertilizer,
		fertilizer_to_water,
		water_to_light,
		light_to_temperature,
		temperature_to_humidity,
		humidity_to_location,
	]);

	const CHUNK_SIZE: u64 = 4096;
	const NUM_THREADS: u16 = 16;

	let chunks: Arc<Vec<SeedRange>> = Arc::new(
		seeds
			.iter()
			.flat_map(|sr| {
				(0..(sr.len / CHUNK_SIZE)).map(|chunk| {
					let start = sr.start + CHUNK_SIZE * chunk;
					SeedRange {
						start,
						len: CHUNK_SIZE.min(sr.start + sr.len - start),
					}
				})
			})
			.collect(),
	);

	let lowest_location = (0..NUM_THREADS)
		.map(|i| {
			let all_maps_ref = all_maps.clone();
			let chunks_ref = chunks.clone();
			std::thread::spawn(move || {
				((i as usize)..chunks_ref.len())
					.step_by(NUM_THREADS as usize)
					.flat_map(|j| {
						let sr = chunks_ref[j];
						let range = (sr.start)..(sr.start + sr.len);
						range.map(|seed| Mapper::map_many(all_maps_ref.as_ref(), seed))
					})
					.min()
					.unwrap()
			})
		})
		.collect::<Vec<JoinHandle<u64>>>()
		.into_iter()
		.map(|t| t.join().unwrap())
		.min()
		.unwrap();

	dbg!(lowest_location);

	Ok(())
}
