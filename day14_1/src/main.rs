use std::{fs::read_to_string, time::Instant};

type RoomSize = (isize, isize);
type Coords = (isize, isize);

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
    fn place_robot(&mut self, robot:&Robot){
        let mut index = 0;
        loop{
            if self.quads[index].detect_robot(robot){
                self.quads[index].robots += 1;
                break;
            }
            index += 1;
            if index == 4{
                break;
            }
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
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> isize{
    let mut acc = 1;
    let steps: isize = 100;
    let room_size: RoomSize = (101, 103); 
    let mut room = Room::default();
    room.construct_room(&room_size);
    for data in full_data{
        let mut robot = do_the_robot(data);
        robot.move_robot(&room_size, &steps);
        room.place_robot(&robot);
    }
    for quad in room.quads{
        if quad.robots > 0{
            acc *= quad.robots;
        }
    }

    acc
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
