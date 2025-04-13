use std::{collections::VecDeque, fs::read_to_string, time::Instant};
type Floor = Vec<Vec<char>>;
type Coords = (usize, usize); // coords are (X,Y)
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) -> WareHouse {
        let mut floor: Floor = Vec::with_capacity(50);
        let mut iter = self.input.lines();
        let mut position: Coords = (0, 0);
        loop {
            let line = match iter.next() {
                Some(line) => line,
                None => panic!("the should be some floor or empty line here"),
            };
            if line.starts_with('#') {
                floor.push(line.chars().collect());
            } else {
                break;
            }
        }
        let move_list = match iter.next() {
            Some(moves) => moves.chars().collect(),
            None => panic!("There should be some moves here"),
        };
        floor = self.expand_floor(floor);
        for (idy, line) in floor.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == '@' {
                    position = (idx, idy);
                    break;
                }
            }
        }
        let robot = Robot {
            position,
            move_list,
        };
        let output = WareHouse { floor, robot };

        output
    }
    fn expand_floor(&self, floor: Floor) -> Floor {
        let mut output: Floor = Vec::new();
        for line in floor {
            let mut new_line = Vec::new();
            for tile in line {
                match tile {
                    '#' => new_line.append(&mut vec!['#', '#']),
                    'O' => new_line.append(&mut vec!['[', ']']),
                    '.' => new_line.append(&mut vec!['.', '.']),
                    '@' => new_line.append(&mut vec!['@', '.']),
                    _ => panic!("invalid match in floor expander: {}", tile),
                }
            }
            output.push(new_line);
        }
        output
    }
}
#[derive(Debug, Clone, Default)]
struct WareHouse {
    floor: Floor,
    robot: Robot,
}
impl WareHouse {
    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for (idy, line) in self.floor.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == '[' {
                    sum += (idy * 100) + idx;
                }
            }
        }
        sum
    }
    fn do_the_robot(&mut self) {
        for direction in self.robot.move_list.clone() {
            let current_tile = self.robot.position;
            match direction {
                '^' | 'v' => {
                    match self.get_vertical_moves(
                        &direction,
                        &current_tile,
                        VecDeque::new(),
                        VecDeque::new(),
                    ) {
                        Some(mut moves) => {
                            self.process_vertical_moves(&mut moves, &direction);
                        }
                        None => continue,
                    };
                }
                '>' | '<' => match self.get_horizontal_moves(&direction, &current_tile, Vec::new())
                {
                    Some(mut moves) => {
                        self.process_horizontal_moves(&mut moves);
                    }
                    None => continue,
                },
                _ => panic!("do the robot paniced!"),
            }
        }
    }
    fn get_next_tile(&self, direction: &char, current_tile: &Coords) -> Coords {
        let (x, y) = *current_tile;
        return match *direction {
            '^' => (x, y - 1), // North
            'v' => (x, y + 1), // South
            '>' => (x + 1, y), // East
            '<' => (x - 1, y), // West
            _ => panic!("not a valid character"),
        };
    }
    fn get_horizontal_moves(
        &self,
        direction: &char,
        current_tile: &Coords,
        moves: Vec<Coords>,
    ) -> Option<Vec<Coords>> {
        let mut moves = moves;
        let next = self.get_next_tile(&direction, &current_tile);
        match self.floor[next.1][next.0] {
            '#' => return None,
            '[' | ']' => {
                moves.push(next);
                match self.get_horizontal_moves(direction, &next, moves) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            '.' => {
                moves.push(next);
                return Some(moves);
            }
            _ => panic!("Thats nowhere to be found in this room"),
        }
    }
    fn get_vertical_moves(
        &self,
        direction: &char,
        current_tile: &Coords,
        moves: VecDeque<Coords>,
        to_check: VecDeque<Coords>,
    ) -> Option<VecDeque<Coords>> {
        let mut moves = moves;
        let mut to_check = to_check;
        let next = self.get_next_tile(&direction, &current_tile);
        match self.floor[next.1][next.0] {
            '#' => return None,
            '[' => {
                moves.push_back(next);
                to_check.push_back((next.0 + 1, next.1));
                match self.get_vertical_moves(&direction, &next, moves, to_check) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            ']' => {
                moves.push_back(next);
                to_check.push_back((next.0 - 1, next.1));
                match self.get_vertical_moves(&direction, &next, moves, to_check) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            '.' => {
                match to_check.pop_front() {
                    Some(next) => {
                        moves.push_back(next);
                        match self.get_vertical_moves(&direction, &next, moves, to_check) {
                            Some(moves) => return Some(moves),
                            None => return None,
                        };
                    }
                    None => return Some(moves),
                };
            }
            _ => panic!(
                "get vertical moves really shit the bed.. next tile is: {}",
                self.floor[next.1][next.0]
            ),
        }
    }
    fn process_vertical_moves(&mut self, moves: &VecDeque<Coords>, direction: &char) {
        let mut moves: Vec<(usize, usize)> = moves.clone().into_iter().collect();
        moves.sort();
        moves.dedup();
        moves.sort_by_key(|y| y.1);
        let mut moves: VecDeque<Coords> = moves.into_iter().collect();
        loop {
            match direction {
                '^' => match moves.pop_front() {
                    Some(from) => {
                        let to = (from.0, from.1 - 1);
                        self.make_move(to, from);
                    }
                    None => {
                        let from = self.robot.position;
                        let to = (from.0, from.1 - 1);
                        self.make_move(to, from);
                        self.robot.position = to;
                        break;
                    }
                },
                'v' => match moves.pop_back() {
                    Some(from) => {
                        let to = (from.0, from.1 + 1);
                        self.make_move(to, from);
                    }
                    None => {
                        let from = self.robot.position;
                        let to = (from.0, from.1 + 1);
                        self.make_move(to, from);
                        self.robot.position = to;
                        break;
                    }
                },
                _ => panic!("process vertical moves made a fool of itself.."),
            };
        }
    }
    fn process_horizontal_moves(&mut self, moves: &mut Vec<Coords>) {
        loop {
            let to = moves.pop().unwrap();
            match moves.last() {
                Some(from) => self.make_move(to, *from),
                None => {
                    self.make_move(to, self.robot.position);
                    self.robot.position = to;
                    break;
                }
            };
        }
    }
    fn make_move(&mut self, to: Coords, from: Coords) {
        let to_value = self.floor[to.1][to.0];
        self.floor[to.1][to.0] = self.floor[from.1][from.0];
        self.floor[from.1][from.0] = to_value;
    }
}

#[derive(Debug, Clone, Default)]
struct Robot {
    position: Coords,
    move_list: VecDeque<char>,
}
fn main() {
    let path = "./data/data";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    let answer = babbage(input);
    println!("The answer is: {}", answer);
}
fn babbage(input: InputData) -> usize {
    let now = Instant::now();
    let mut warehouse = input.parse();
    warehouse.do_the_robot();
    println!("babbage runtime: {}", now.elapsed().as_micros());
    return warehouse.sum_gps();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_are_parsed() {
        let to_match: Vec<char> = vec!['<', '^', '^', '>'];
        let path = "./data/test_parse";
        let input = InputData {
            input: match read_to_string(path) {
                Ok(file) => file,
                Err(_) => panic!("File should be here"),
            },
        };
        let moves = input.parse().robot.move_list;
        assert_eq!(moves, to_match)
    }
    #[test]
    fn next_tile_is_correct() {
        let to_match = (9, 3);
        let path = "./data/test_s";
        let mut input = InputData {
            input: match read_to_string(path) {
                Ok(file) => file,
                Err(_) => panic!("File should be here"),
            },
        };
        let direction = input.parse().robot.move_list.pop_front().unwrap();
        let current_tile = &input.parse().robot.position;
        let next_tile = input.parse().get_next_tile(&direction, &current_tile);
        assert_eq!(next_tile, to_match)
    }
    #[test]
    fn passes_example() {
        let to_match = 9021;
        let path = "./data/test";
        let mut input = InputData {
            input: match read_to_string(path) {
                Ok(file) => file,
                Err(_) => panic!("File should be here"),
            },
        };
        let result = babbage(input);
        assert_eq!(to_match, result);
    }
}
