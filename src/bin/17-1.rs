use std::collections::{BinaryHeap, HashSet};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

use Direction::*;

impl Direction {
	pub fn offset(&self) -> (isize, isize) {
		match *self {
			Left => (-1, 0),
			Right => (1, 0),
			Up => (0, -1),
			Down => (0, 1),
		}
	}

	pub fn apply(&self, (x, y): (usize, usize), (w, h): (usize, usize)) -> Option<(usize, usize)> {
		let (off_x, off_y) = self.offset();
		let (new_x, new_y) = (x as isize + off_x, y as isize + off_y);
		if new_x < 0 || new_y < 0 || new_x as usize >= w || new_y as usize >= h {
			None
		} else {
			Some((new_x as usize, new_y as usize))
		}
	}

	pub fn turns(&self) -> [Direction; 2] {
		match *self {
			Left | Right => [Up, Down],
			Up | Down => [Left, Right],
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AStarState {
	position: (usize, usize),
	dimensions: (usize, usize),
	direction: Direction,
	time_straight: u8,
	backward_cost: u32,
}

impl AStarState {
	fn heuristic(&self) -> u32 {
		// manhattan distance
		let goal_x = self.dimensions.0 - 1;
		let goal_y = self.dimensions.1 - 1;
		let diff_x = (self.x() as isize - goal_x as isize).abs() as u32;
		let diff_y = (self.y() as isize - goal_y as isize).abs() as u32;
		return diff_x + diff_y;
	}

	fn x(&self) -> usize {
		self.position.0
	}

	fn y(&self) -> usize {
		self.position.1
	}

	fn is_goal(&self) -> bool {
		self.position == (self.dimensions.0 - 1, self.dimensions.1 - 1)
	}

	fn cost_function(&self) -> u32 {
		self.backward_cost + self.heuristic()
	}

	fn start_states(dimensions: (usize, usize), heat_loss: &Vec<Vec<u8>>) -> [AStarState; 2] {
		[
			AStarState {
				position: (1, 0),
				dimensions,
				direction: Right,
				time_straight: 1,
				backward_cost: heat_loss[0][1] as u32,
			},
			AStarState {
				position: (0, 1),
				dimensions,
				direction: Down,
				time_straight: 1,
				backward_cost: heat_loss[1][0] as u32,
			},
		]
	}

	fn successors<'a>(
		&self,
		heat_loss: &Vec<Vec<u8>>,
		buf: &'a mut [AStarState; 3],
	) -> &'a mut [AStarState] {
		let mut count: usize = 0;
		if self.time_straight < 3 {
			if let Some(new_pos) = self.direction.apply(self.position, self.dimensions) {
				buf[count] = AStarState {
					direction: self.direction,
					position: new_pos,
					time_straight: self.time_straight + 1,
					backward_cost: self.backward_cost + heat_loss[new_pos.1][new_pos.0] as u32,
					dimensions: self.dimensions,
				};
				count += 1;
			}
		}

		for new_dir in self.direction.turns() {
			if let Some(new_pos) = new_dir.apply(self.position, self.dimensions) {
				buf[count] = AStarState {
					direction: new_dir,
					position: new_pos,
					time_straight: 1,
					backward_cost: self.backward_cost + heat_loss[new_pos.1][new_pos.0] as u32,
					dimensions: self.dimensions,
				};
				count += 1;
			}
		}

		&mut buf[0..count]
	}
}

impl PartialOrd for AStarState {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for AStarState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// reverse so that we get a min heap not a max heap
		self.cost_function().cmp(&other.cost_function()).reverse()
	}
}

impl Default for AStarState {
	fn default() -> Self {
		AStarState {
			position: (0, 0),
			dimensions: (0, 0),
			direction: Left,
			time_straight: 0,
			backward_cost: 0,
		}
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());

	let mut heat_loss: Vec<Vec<u8>> = Vec::new();
	for line in iter {
		let mut row: Vec<u8> = Vec::new();
		for c in line.chars() {
			row.push(c.to_digit(10).unwrap() as u8);
		}
		heat_loss.push(row);
	}

	let width = heat_loss[0].len();
	let height = heat_loss.len();

	let mut fringe = BinaryHeap::<AStarState>::new();
	for s in AStarState::start_states((width, height), &heat_loss) {
		fringe.push(s);
	}

	let mut visited: HashSet<((usize, usize), Direction, u8)> = HashSet::new();

	while let Some(state) = fringe.pop() {
		if visited.contains(&(state.position, state.direction, state.time_straight)) {
			continue;
		}

		visited.insert((state.position, state.direction, state.time_straight));

		if state.is_goal() {
			dbg!(state.backward_cost);
			return Ok(());
		}

		let mut successors_buf: [AStarState; 3] = Default::default();
		for s in state.successors(&heat_loss, &mut successors_buf) {
			let s_val = std::mem::take(s);
			fringe.push(s_val);
		}
	}

	Ok(())
}
