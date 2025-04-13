use std::{fs::read_to_string, time::Instant};

type RoomSize = (isize, isize);
type Coords = (isize, isize);
type Field = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Robot {
    position: Coords,
    speed: Coords,
}

impl Robot {
    fn move_robot(&mut self, room_size: &RoomSize, steps: &isize) {
        let x_mod = room_size.0;
        let y_mod = room_size.1;
        self.position.0 = (self.position.0 + (self.speed.0 * steps)).rem_euclid(x_mod);
        self.position.1 = (self.position.1 + (self.speed.1 * steps)).rem_euclid(y_mod);
    }
}
#[derive(Debug, Clone)]
struct Display {
    robots: Vec<Robot>,
    screen: Field,
    possible_tree: bool,
    line_to_check: usize,
    room_size: RoomSize,
}
impl Display {
    fn clear(&mut self) {
        let x = self.screen[0].len();
        let y = self.screen.len();
        let scan_line: Vec<char> = vec![' '; x];
        self.screen = vec![scan_line; y];
    }
    fn get_next_frame(&mut self, increment: isize) {
        let room_size = self.room_size;
        let mut index = 0;
        let mut star_counter: Vec<(isize, isize)> = vec![(0, 0); room_size.1 as usize];
        loop {
            self.robots[index].move_robot(&room_size, &increment);
            let pos = self.robots[index].position.1;
            star_counter[pos as usize].0 = pos;
            star_counter[pos as usize].1 += 1;
            index += 1;
            if index == self.robots.len() {
                star_counter.sort_by_key(|tup| tup.1);
                let last = star_counter.last().unwrap();
                if last.1 >= 30 {
                    self.possible_tree = true;
                    self.line_to_check = last.0 as usize;
                }
                break;
            }
        }
    }
    fn frame_buffer(&mut self) {
        self.clear();
        for robot in &self.robots {
            self.screen[robot.position.1 as usize][robot.position.0 as usize] = '*';
        }
    }
    fn draw(&mut self) {
        print!("{esc}c", esc = 27 as char); //clears terminal
        for (index, line) in self.screen.iter().enumerate() {
            for c in line {
                print!("{c}");
            }
            if index == 70 {
                break;
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> isize {
    let room_size: RoomSize = (101, 103);
    let mut counter = 0;
    let mut display = build_display(full_data, room_size);
    let mut increment = 1;

    loop {
        if display.possible_tree {
            increment = display.room_size.1;
            display.frame_buffer();
            if line_checker(&display.screen[display.line_to_check]) {
                display.draw();
                break;
            }
            display.possible_tree = false;
        }
        counter += increment;
        display.get_next_frame(increment);
    }
    counter
}
fn line_checker(line: &Vec<char>) -> bool {
    let mut window = line.windows(30);
    'outer: loop {
        let view = match window.next() {
            Some(view) => view,
            None => return false,
        };
        for el in view {
            if *el != '*' {
                continue 'outer;
            }
        }
        return true;
    }
}
fn build_display(full_data: Vec<String>, room_size: RoomSize) -> Display {
    let mut robots: Vec<Robot> = Vec::with_capacity(500);
    let scan_line: Vec<char> = vec![' '; room_size.0 as usize];
    let screen = vec![scan_line; room_size.1 as usize];
    for data in full_data {
        robots.push(do_the_robot(data));
    }

    let output = Display {
        robots,
        screen,
        possible_tree: false,
        line_to_check: 0,
        room_size,
    };

    output
}
fn do_the_robot(data: String) -> Robot {
    let data = data;
    let mut elements = data.split_whitespace();
    let position = get_coords(elements.next().unwrap());
    let speed = get_coords(elements.next().unwrap());
    let robot = Robot { position, speed };
    robot
}
fn get_coords(input: &str) -> Coords {
    let as_chars = input.chars();
    let mut coords: Coords = Coords::default();
    let mut num: Vec<char> = Vec::new();
    for c in as_chars {
        if c.is_numeric() || c == '-' {
            num.push(c);
        }
        if c == ',' {
            let as_string: String = num.iter().collect();
            coords.0 = as_string.parse().unwrap();
            num.clear();
        }
    }
    let as_string: String = num.iter().collect();
    coords.1 = as_string.parse().unwrap();

    coords
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
