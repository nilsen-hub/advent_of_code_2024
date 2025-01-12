use std::{collections::{HashMap, HashSet}, fs::read_to_string, time::Instant};

type Field = Vec<Vec<char>>;

#[derive(Debug, Clone)]
enum Direction{
    North,
    South,
    East,
    West,
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coords{
    x: isize, 
    y: isize,
}

impl std::ops::Add<(isize, isize)> for Coords {
  type Output = Coords;

  fn add(self, rhs: (isize, isize)) -> Self::Output {
    Self::Output {
        x: self.x + rhs.0,
        y: self.y + rhs.1,
    }    
  }
}
impl std::ops::Add<Coords> for Coords {
    type Output = Coords;
  
    fn add(self, rhs: Coords) -> Self::Output {
      Self::Output {
          x: self.x + rhs.y,
          y: self.x + rhs.y,
      }    
    }
}
impl Coords{
    const NORTH: Coords = Coords{
        x:0,
        y:-1,
    };

    const SOUTH: Coords = Coords{
        x:0,
        y:1,
    };
    const EAST: Coords = Coords{
        x:1,
        y:0,
    };
    const WEST: Coords = Coords{
        x:-1,
        y:0,
    };

    fn from(t0:isize, t1:isize) -> Coords{
        return Coords{
            x: t0,
            y: t1,
        };

    }
}
#[derive(Debug, Clone, Copy)]
struct Node{
    coords: Coords,
    distance: usize,
}
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) -> Solver {
        let lines = self.input.lines();
        let mut field: Field = Vec::with_capacity(150);
        for line in lines{
            field.push(line.chars().collect());
        }
        for line in &field{
            for c in line{
                print!("{c}");
            }
            println!("");
        }
        println!("");
        let mut end = Coords::from(0,0);
        
        for (idx, c) in field[1].iter().enumerate(){
            if *c == 'S'{
                end = Coords::from(idx as isize, 1);
            }
        }
        let start = Coords::from(end.b, end.a);
        return Solver{
            maze: Maze{
                field,
                field_graph: HashMap::new(),
                start,
                end
            },
            rudolph: Rudolph {
                position: start,
                direction: Direction::East,
                seen_positions: HashSet::with_capacity(500),
                points: 0,
            },
        };
    }
}
#[derive(Debug, Clone)]
struct Maze{
    field: Field,
    field_graph: HashMap<Coords,Vec<Node>>,
    start: Coords,
    end: Coords,
}
impl Maze{
    fn make_graph(&mut self){
        let mut position = self.start;
        let mut visited:HashSet<Coords> = HashSet::with_capacity(500);
        let start = self.get_connected_nodes(position, visited);

        self.field_graph.insert(position, start.0);

    }
    fn get_connected_nodes(&self, start_pos:Coords, visited:HashSet<Coords>) -> (Vec<Node>, HashSet<Coords>){
        let mut current_pos = start_pos;
        let mut visited = visited;
        let directions = [
            Coords::NORTH, 
            Coords::SOUTH, 
            Coords::EAST, 
            Coords::WEST,
            ];
        
        let mut nodes: Vec<Node> = Vec::with_capacity(5);
        for direction in directions.iter(){
            let mut steps = 0;
            'outer: loop{
                visited.insert(current_pos);
                current_pos = current_pos + *direction;
                if self.field[current_pos.y as usize][current_pos.x as usize] == '#'{
                    break;
                }
                steps += 1;
                for next in directions{
                    let check = current_pos + next;
                    
                    if visited.contains(&check) || next == *direction{
                        continue;
                    }
                    if self.field[check.y as usize][check.x as usize] == '.'{
                        let node = Node{
                            coords: current_pos,
                            distance: steps,   
                        };
                        nodes.push(node);
                        break 'outer;
                    } 
                }
            }

        }
        return (nodes, visited);
    }
}
#[derive(Debug, Clone)]
struct Rudolph{
    position: Coords,
    seen_positions: HashSet<Coords>,
    direction: Direction,
    points: usize,
}
#[derive(Debug, Clone)]
struct Solver{
    maze: Maze,
    rudolph: Rudolph,
}
impl Solver{
    fn solve(&mut self,) -> usize {
        return self.rudolph.points;
    }
    fn rudolph_turn(&mut self){

    }
    fn rudolph_move_to_next_junction(&mut self){

    }
    //fn find_next_junction(&self, mut pos:Coords) -> Option<Coords>{
    //    
    //    match self.rudolph.direction{
    //        North=>,
    //        South=>,
    //        East=>,
    //        West=>,
    //    }
    //}
}
fn main() {
    let path = "./data/test_1";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    babbage(input);
}
fn babbage(input: InputData){
    let now = Instant::now();
    let mut solver = input.parse();
    //println!("Add tuple experiment: {:?}", tup_add);
    println!("The answer is: {}", solver.solve());
    println!("babbage runtime: {}", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_examples() {
    }
}
