use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

type Coordinates = (isize, isize);
type Board = Vec<Vec<Point>>;

enum ParserError {
	NoStartFound,
	NoTestData,
	InvalidBoard,
}

impl std::fmt::Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::NoTestData => write!(f, "No test data found"),
			Self::NoStartFound => write!(f, "No Guard in board"),
			Self::InvalidBoard => write!(f, "Board contains illegal characters"),
		}
	}
}

impl std::fmt::Debug for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NoStartFound => write!(f, "NoStartFound"),
			Self::NoTestData => write!(f, "NoTestData"),
			Self::InvalidBoard => write!(f, "InvalidBoard"),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
	Up,
	Down,
	Right,
	Left,
}

impl Direction {
	fn get_next(&self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
	y: usize,
	x: usize,
	symbol: Symbols,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Symbols {
	Guard,
	Ground,
	Obstacle,
}

struct Guard {
	board: Board,
	start: Point,
	steps: usize,
	visited: HashSet<Point>,
	direction: Direction,
}

impl FromStr for Guard {
	type Err = ParserError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Err(ParserError::NoTestData);
		}

		let mut board: Vec<Vec<Point>> = Vec::with_capacity(100);
		let mut direction: Option<Direction> = None;
		let mut start: Option<Point> = None;

		for (y, line) in s.lines().enumerate() {
			let mut tmp = vec![];
			for (x, char) in line.chars().enumerate() {
				let symbol = match char {
					'#' => Symbols::Obstacle,
					'.' => Symbols::Ground,
					'^' | 'V' | '>' | '<' => Symbols::Guard,
					_ => return Err(ParserError::InvalidBoard),
				};

				tmp.push(Point { x, y, symbol });

				if char == '.' || char == '#' {
					continue;
				}

				direction = match char {
					'^' => Some(Direction::Up),
					'V' => Some(Direction::Down),
					'>' => Some(Direction::Right),
					'<' => Some(Direction::Left),
					_ => return Err(ParserError::InvalidBoard),
				};

				start = Some(Point { x, y, symbol });
			}

			board.push(tmp);
		}

		let start = match start {
			Some(start) => start,
			None => return Err(ParserError::NoStartFound),
		};

		let direction = match direction {
			Some(d) => d,
			None => return Err(ParserError::InvalidBoard),
		};

		Ok(Self {
			board,
			start,
			steps: 0,
			visited: HashSet::new(),
			direction,
		})
	}
}

impl Guard {
	fn test_for_looping_obstacle(&self) -> usize {
		use Direction as d;

		let cu: Coordinates = (-1, 0);
		let cd: Coordinates = (1, 0);
		let cl: Coordinates = (0, -1);
		let cr: Coordinates = (0, 1);
		let oob = self.board.len();

		self.visited
			.iter()
			.filter_map(|obstacle| {
				let mut seen: HashSet<(Direction, Point)> = HashSet::new();
				let mut current = self.board[self.start.y][self.start.x];
				let mut current_direction = self.direction;

				loop {
					let Point { x, y, symbol: _ } = current;

					let (ny, nx) = match current_direction {
						d::Up => cu,
						d::Down => cd,
						d::Left => cl,
						d::Right => cr,
					};

					let yi = y as isize;
					let xi = x as isize;

					let ny = (yi + ny) as usize;
					let nx = (xi + nx) as usize;

					if ny >= oob || nx >= oob {
						break None;
					}

					let next = self.board[ny][nx];
					let next_is_obstacle = next.y == obstacle.y && next.x == obstacle.x;

					if next.symbol == Symbols::Obstacle || next_is_obstacle {
						current_direction = current_direction.get_next();

						if seen.contains(&(current_direction, current)) {
							return Some(true);
						}

						seen.insert((current_direction, current));
						continue;
					}

					current = next;
				}
			})
			.count()
	}

	fn walk_path(&mut self) -> Self {
		use Direction as d;
		let cu: Coordinates = (-1, 0);
		let cd: Coordinates = (1, 0);
		let cl: Coordinates = (0, -1);
		let cr: Coordinates = (0, 1);

		let mut current = self.board[self.start.y][self.start.x];
		let mut current_direction = self.direction;
		let oob = self.board.len();
		self.visited.insert(current);

		loop {
			let Point { x, y, symbol: _ } = current;

			let (ny, nx) = match current_direction {
				d::Up => cu,
				d::Down => cd,
				d::Left => cl,
				d::Right => cr,
			};

			let yi = y as isize;
			let xi = x as isize;

			let ny = (yi + ny) as usize;
			let nx = (xi + nx) as usize;

			if ny >= oob || nx >= oob {
				break;
			}

			let next = self.board[ny][nx];

			if next.symbol == Symbols::Obstacle {
				current_direction = current_direction.get_next();
				continue;
			}

			self.steps += 1;

			current = next;
			self.visited.insert(next);
		}

		Self {
			visited: self.visited.clone(),
			direction: self.direction,
			start: self.start,
			steps: self.steps,
			board: self.board.clone(),
		}
	}
}

pub fn solve_part_1() {
	let now = std::time::Instant::now();
	let input = fs::read_to_string("input/day6.txt").expect("day6 input exists");
	let mut guard = match Guard::from_str(&input) {
		Ok(guard) => guard,
		Err(e) => panic!("{e}"),
	};

	let result = guard.walk_path().visited.len();
	println!("D6-1 runtime: {}", now.elapsed().as_micros());
	println!("D6-1 result: {result}");
	let _ = fs::write("output/day6.part1.txt", result.to_string());
}

pub fn solve_part_2() {
	let now = std::time::Instant::now();
	let input = fs::read_to_string("input/day6.txt").expect("day6 input exists");
	let mut guard = match Guard::from_str(&input) {
		Ok(guard) => guard,
		Err(e) => panic!("{e}"),
	};

	let result = guard.walk_path().test_for_looping_obstacle();
	println!("D6-2 runtime: {}", now.elapsed().as_micros());
	println!("D6-2 result: {result}");
	//let _ = fs::write("output/day6.part1.txt", result.to_string());
}
fn main(){
    solve_part_1();
    solve_part_2();

}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_parses() {
		let input = fs::read_to_string("input/day6.example.txt").expect("day6 test data exists");

		let expected_start = Point {
			y: 6,
			x: 4,
			symbol: Symbols::Guard,
		};

		let guard = Guard::from_str(&input);
		assert!(guard.is_ok());

		let guard = guard.unwrap();

		assert!(!guard.board.is_empty());
		assert_eq!(expected_start, guard.start);
		assert_eq!(guard.steps, 0);
		assert_eq!(guard.visited.len(), 0);
	}

	#[test]
	fn it_walks_example() {
		let input = fs::read_to_string("input/day6.example.txt").expect("day6 test data exists");
		let mut guard = Guard::from_str(&input).unwrap();
		let unique_route = guard.walk_path().visited.len();
		let expected_result = 41;

		assert_eq!(expected_result, unique_route);
	}

	#[test]
	fn it_places_obstacles() {
		let input = fs::read_to_string("input/day6.example.txt").expect("day6 test data exists");
		let result = Guard::from_str(&input)
			.unwrap()
			.walk_path()
			.test_for_looping_obstacle();
		let expected_result = 6;

		assert_eq!(expected_result, result);
	}
}
