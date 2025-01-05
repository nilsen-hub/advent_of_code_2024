use std::{fs::read_to_string, time::Instant, time::Duration, thread};

type RoomSize = (isize, isize);
type Coords = (isize, isize);
type Field = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Robot {
    position: Coords,
    speed: Coords,
}
impl Robot {
    fn move_robot(&mut self, room_size: &RoomSize, steps:&isize){
         let x_mod = room_size.0;
         let y_mod = room_size.1;
         self.position.0 = (self.position.0 + (self.speed.0 * steps)).rem_euclid(x_mod);
         self.position.1 = (self.position.1 + (self.speed.1 * steps)).rem_euclid(y_mod);
    }    
}
#[derive(Debug, Clone, Default, Copy)]
struct Room {
    size: RoomSize,
    quads: [Quadrant;4],
}
impl Room{
    fn construct_room(&mut self, size: &RoomSize){
        self.size = *size;
        let mut id: usize = 0;
 
        loop{
            self.quads[id].get_quad(id, size);
            id += 1;
            if id == 4{
                break;
            }
        }
    }
}
#[derive(Debug, Clone)]
struct Display{
    robots: Vec<Robot>,
    room: Room,
    screen: Field
}
impl Display{
    fn clear(&mut self){
        let x = self.screen[0].len();
        let y = self.screen.len();
        let scan_line:Vec<char> = vec![' ';x];
        self.screen = vec![scan_line;y];
    }
    fn get_next_frame(&mut self){
        let room_size = (self.screen[0].len() as isize, self.screen.len() as isize);
        let mut index = 0;
        loop{
            self.robots[index].move_robot(&room_size, &1);
            index += 1;
            if index == self.robots.len(){
                break;
            }
        }
    }
    fn prepare_buffer(&mut self){
        self.clear();
        for robot in &self.robots{
            self.screen[robot.position.1 as usize][robot.position.0 as usize ] = '*';
        }
    }
    fn draw(&mut self){
        print!("{esc}c", esc = 27 as char); //clears terminal
        for line in &self.screen{
            for c in line{
                print!("{c}");
            }
            println!("");
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]
struct Quadrant {
    perimeter: (Coords, Coords), // upper left corner, lower right corner
    robots: isize,
}
impl Quadrant {
    fn get_quad(&mut self, id: usize, size: &RoomSize){
        match id {
            0 => self.perimeter = ((0, 0),(size.0 / 2 - 1, size.1 / 2 - 1)),
            1 => self.perimeter = ((size.0 / 2 + 1, 0),(size.0 - 1, size.1 / 2 - 1)),
            2 => self.perimeter = ((0, size.1 / 2 + 1),(size.0 / 2 - 1, size.1 - 1)),
            3 => self.perimeter = ((size.0 / 2 + 1, size.1 / 2 + 1),(size.0 - 1, size.1 - 1)),
            _ => panic!("should never happen"),
        };
    }
    fn detect_robot(&mut self, robot:&Robot) -> bool {
        let range_x = self.perimeter.0.0..=self.perimeter.1.0;
        let range_y = self.perimeter.0.1..=self.perimeter.1.1;
        if range_x.contains(&robot.position.0) && range_y.contains(&robot.position.1){
            return true
        }
        false 
    }
    
}
fn main() {
    let now = Instant::now();
    //let path = "./data/data";
    let path = "./data/test";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> isize{
    let mut acc = 1;
    let room_size: RoomSize = (11, 7); //test
    //let room_size: RoomSize = (101, 103); //not test
    let mut display = build_display(full_data, room_size);
    //let sleep_time = Duration::from_millis(250);
    for frame in 0..10000000000000{
        display.prepare_buffer();
        // for test only
        if display.screen[0][5] == '*' && display.screen[6][5] == '*' && display.screen[5][0] == '*' && display.screen[5][10] == '*'  {
            display.draw();
            acc = frame;
            break
        }
        display.get_next_frame();
    }


    acc
}
fn build_display(full_data: Vec<String>, room_size:RoomSize) -> Display {
    let mut room = Room::default();
    room.construct_room(&room_size);
    let mut robots: Vec<Robot> = Vec::with_capacity(500);
    let scan_line:Vec<char> = vec![' ';room_size.0 as usize];
    let screen = vec![scan_line;room_size.1 as usize];
    for data in full_data{
        robots.push(do_the_robot(data));
    }
    let output = Display{
        robots,
        room,
        screen,
    };

    output
}
fn do_the_robot(data: String) -> Robot{
    let data = data;
    let mut elements = data.split_whitespace();
    let position = get_coords(elements.next().unwrap());
    let speed = get_coords(elements.next().unwrap());
    let robot = Robot{
        position,
        speed,
    };
    robot
}
fn get_coords(input: &str) -> Coords{
    let as_chars = input.chars();
    let mut coords:Coords = Coords::default();
    let mut num:Vec<char> = Vec::new(); 
    for c in as_chars{
        if c.is_numeric() || c == '-'{
            num.push(c);
        }
        if c == ','{
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
