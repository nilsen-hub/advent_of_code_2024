use std::{collections::VecDeque, fs::read_to_string, time::Instant};
type Floor = Vec<Vec<char>>;
type Coords = (usize, usize);
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
        let moves = match iter.next() {
            Some(moves) => moves.chars().collect(),
            None => panic!("There should be some moves here"),
        };
        for (idy, line) in floor.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == '@' {
                    position = (idx, idy);
                    break;
                }
            }
        }
        let robot = Robot { position, moves };
        let output = WareHouse { floor, robot };

        output
    }
}
#[derive(Debug, Clone, Default)]
struct WareHouse {
    floor: Floor,
    robot: Robot,
}
impl WareHouse {
    fn get_next_tile(&self, direction: &char, current_tile: &Coords) -> Coords {
        let (x, y) = current_tile.clone();
        return match *direction {
            '^' => (x, y - 1), // North
            'v' => (x, y + 1), // South
            '>' => (x + 1, y), // East
            '<' => (x - 1, y), // West
            _ => panic!("not a valid character"),
        };
    }
    fn check_move(
        &self,
        direction: &char,
        current_tile: &Coords,
        moves: Vec<Coords>,
    ) -> Option<Vec<Coords>> {
        let mut moves = moves;
        let next = self.get_next_tile(&direction, &current_tile);
        match self.floor[next.1][next.0] {
            '#' => return None,
            'O' => {
                moves.push(next);
                self.check_move(direction, &next, moves)
            }
            '.' => {
                moves.push(next);
                return Some(moves);
            }
            _ => panic!("Thats nowhere to be found in this room"),
        }
    }
    fn make_move(&mut self, moves: Vec<Coords>) {}
}
#[derive(Debug, Clone, Default)]
struct Robot {
    position: Coords,
    moves: VecDeque<char>,
}
fn main() {
    let now = Instant::now();
    //let path = "./data/data";
    let path = "./data/test_s";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    let answer = babbage(input);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(input: InputData) -> isize {
    let mut acc = 0;
    let mut warehouse = input.parse();
    for line in warehouse.floor.clone() {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    print!("moves: ");
    for mov in warehouse.robot.moves.clone() {
        print!("{}", mov);
    }
    println!("");
    let current_tile = warehouse.robot.position;
    let direction = warehouse.robot.moves.pop_front().unwrap();
    println!(
        "current posistion: {:?} next tile: {:?}",
        current_tile,
        warehouse.get_next_tile(&direction, &current_tile)
    );

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
