use std::{collections::{BTreeMap, HashMap, HashSet, VecDeque}, fs::read_to_string, thread::current, time::Instant};

type Field = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq)]
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
impl std::ops::Sub<(isize, isize)> for Coords {
    type Output = Coords;
  
    fn sub(self, rhs: (isize, isize)) -> Self::Output {
      Self::Output {
          x: self.x - rhs.0,
          y: self.y - rhs.1,
      }    
    }
  }
impl std::ops::Add<Coords> for Coords {
    type Output = Coords;
  
    fn add(self, rhs: Coords) -> Self::Output {
      Self::Output {
          x: self.x + rhs.x,
          y: self.y + rhs.y,
      }    
    }
}
impl std::ops::Sub<Coords> for Coords {
    type Output = Coords;
  
    fn sub(self, rhs: Coords) -> Self::Output {
      Self::Output {
          x: self.x - rhs.x,
          y: self.y - rhs.y,
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
        let mut end = Coords::from(0,0);
        
        for (idx, c) in field[1].iter().enumerate(){
            if *c == 'E'{
                end = Coords::from(idx as isize, 1);
            }
        }
        let start = Coords::from(end.y, end.x);
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
                points: 0,
                priority: 0,
                path: HashSet::new(),
            },
            to_check: BTreeMap::new(),
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
        let position = self.start;
        
        let to_explore = self.get_connected_nodes(position);
        self.field_graph.insert(position, to_explore.clone());
        
        self.node_crawler(to_explore);
        println!("graph contains {} nodes", self.field_graph.len());
        self.graph_printer();
    }
    fn graph_printer(&self){
        let mut field_copy = self.field.clone();
        for (key, _nodes) in &self.field_graph{
            field_copy[key.y as usize][key.x as usize] = '+';
        }
        for line in &field_copy{
            for c in line{
                print!("{c}");
            }
            println!("");
        }
    }
    fn node_crawler(&mut self, mut to_explore: Vec<Node>){
        loop{
            let node = match to_explore.pop() {
                Some(node) => node,
                None => return,
            };
            if self.field_graph.contains_key(&node.coords){
                continue;
            }
            let mut nodes = self.get_connected_nodes(node.coords);
            self.field_graph.insert(node.coords, nodes.clone());
            to_explore.append(&mut nodes);
        }

    }
    //fn deprec_node_crawler(&mut self, mut to_explore: Vec<Node>){
    //    // recursive function, must be bootstrapped with one valid node vector
    //    let node = match to_explore.pop() {
    //        Some(node) => node,
    //        None => return,
    //    };
//
    //    if self.field_graph.contains_key(&node.coords){
    //        return self.node_crawler(to_explore);
    //    }
//
    //    let mut nodes = self.get_connected_nodes(node.coords);
    //    self.field_graph.insert(node.coords, nodes.clone());
    //    to_explore.append(&mut nodes);
//
    //    return self.node_crawler(to_explore);
    //}
    fn get_connected_nodes(&self, start_pos:Coords,) -> Vec<Node>{
        let mut visited:HashSet<Coords> = HashSet::new();
        visited.insert(start_pos);
        let directions = [
            Coords::NORTH, 
            Coords::SOUTH, 
            Coords::EAST, 
            Coords::WEST,
            ];
        
        let mut nodes: Vec<Node> = Vec::with_capacity(5);
        for direction in directions{
            let mut current_pos = start_pos;
            let mut steps = 0;
            'outer: loop{
                current_pos = current_pos + direction;
                if self.field[current_pos.y as usize][current_pos.x as usize] == '#'{
                    break;
                }
                visited.insert(current_pos);
                steps += 1;
                for next in directions{
                    let check = current_pos + next;
                    if visited.contains(&check) || next == direction{
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
        //nodes.sort_by_key(|node|node.distance);
        return nodes;
    }
}
#[derive(Debug, Clone,)]
struct Rudolph{
    position: Coords,
    direction: Direction,
    points: usize,
    priority: usize,
    path: HashSet<Coords>,
}
#[derive(Debug, Clone)]
struct Solver{
    maze: Maze,
    rudolph: Rudolph,
    to_check: BTreeMap<usize, VecDeque<Rudolph>>
}
impl Solver{
    fn solve(&mut self) -> usize{
        self.maze.make_graph();
        let mut counter: usize = 0;
        self.to_check.insert(0, VecDeque::from([self.rudolph.clone()]));
        'outer: loop{
            let vector_rudolph = self.to_check.pop_first().unwrap();
            
            for rudolph in vector_rudolph.1{  
                self.rudolph = rudolph;          
                if counter % 10000000 == 0{
                println!("counter: {}, rudolph vectors: {}", counter, self.to_check.len());
                self.path_print();
                }
                //println!("iterations: {}", counter);
                self.rudolph.path.insert(self.rudolph.position);
                if self.rudolph.position == self.maze.end{
                    break 'outer;
                }
                let connected_nodes = self.maze.field_graph.get(&self.rudolph.position).unwrap();
                for node in connected_nodes{
                    if self.rudolph.path.contains(&node.coords){
                        continue;
                    }
                    let mut move_points = self.rudolph.points + node.distance;
                    let move_direction = self.turn_detector(&self.rudolph, node.coords);
                    if move_direction != self.rudolph.direction{
                        move_points += 1000;
                    }
                    let priority = move_points + self.rudolph.position.x.abs_diff(node.coords.x) * 100 + self.rudolph.position.x.abs_diff(node.coords.x) * 100;
                    let rudolph = Rudolph{
                        position: node.coords,
                        direction: move_direction.clone(),
                        points: move_points,
                        priority,
                        path: self.rudolph.path.clone(),   
                        };
                    self.to_check.entry(self.rudolph.points)
                        .and_modify(|vec|vec.push_back(rudolph.clone()))
                        .or_insert(VecDeque::from([rudolph]));
                }

                counter += 1;
            }
        }
        self.path_print();
        return self.rudolph.points;
    }
    //fn solve_deprec(&mut self,) -> usize {
    //    self.maze.make_graph();
    //    let mut rudolph_collection: BTreeMap<usize, Vec<Rudolph>> = BTreeMap::new();
    //    rudolph_collection.insert(self.rudolph.points, Vec::from([self.rudolph.clone()]));
    //    let mut counter: usize = 0;
    //    //let mut visited: HashSet<Coords> = HashSet::new();
    //    loop{
    //        counter += 1;
    //        //if counter % 10000 == 0{
    //            //println!("{} iterations counted, there are currently {} rudolphs running around", counter, rudolph_collection.len());
    //        //}
    //        
    //        //if rudolph_collection.len() > 10000{
    //        //    rudolph_collection.truncate(9900);    
    //        //}
    //        let mut rudolph_vec = match rudolph_collection.pop_first().1{
    //            Some(vec) => vec,
    //            None => self.panic_print(current),
    //        };
    //        let mut current = rudolph_vec.pop().unwrap();
    //        if rudolph_vec.len() > 0{
    //            rudolph_collection.insert(rudolph_vec[0].points, rudolph_vec);
    //        }
    //        //println!("Rudolph is at: {:?} living rudolphs: {}", current.position, rudolph_collection.len());
    //        //if current.path.contains(&current.position){
    //        //    continue;
    //        //}
    //        //if visited.contains(&current.position){
    //        //    continue;
    //        //}
    //        //visited.insert(current.position);
//
    //        current.path.insert(current.position);
//
    //        if current.position == self.maze.end{
    //            self.rudolph = current;
    //            break;
    //        }
//
    //        let connected_nodes = match self.maze.field_graph.get(&current.position){
    //            Some(nodes) => nodes,
    //            None => continue,
    //        };
    //        // println!("current position: {:?} connected nodes: {:?}", &current.position, &connected_nodes);
    //        for node in connected_nodes{
    //            if current.path.contains(&node.coords){
    //                continue;
    //            }
//
    //            let mut move_points = current.points + node.distance;
    //            let move_direction = self.turn_detector(&current, node.coords);
//
    //            if move_direction != current.direction{
    //                move_points += 1000;
    //            }
    //            rudolph_collection.entry(current.points).
    //                and_modify(|vec|vec.push(Rudolph{
    //                position: node.coords,
    //                direction: move_direction.clone(),
    //                points: move_points,
    //                path: current.path.clone(),   
    //                })).
    //                    or_insert(Vec::from([Rudolph{
    //                        position: node.coords,
    //                        direction: move_direction,
    //                        points: move_points,
    //                        path: current.path.clone(),   
    //                        }]));   
    //        }
    //        
    //    }
    //    for pos in &self.rudolph.path{
    //        self.maze.field[pos.y as usize][pos.x as usize] = 'O';
    //    }
    //    for line in &self.maze.field{
    //        for c in line{
    //            print!("{c}");
    //        }
    //        println!("");
    //    }
    //    return self.rudolph.points;
    //}
    fn path_print(&mut self){
        for pos in &self.rudolph.path{
            self.maze.field[pos.y as usize][pos.x as usize] = 'O';
        }
        for line in &self.maze.field{
            for c in line{
                print!("{c}");
            }
            println!("");
        }    
    }
    fn turn_detector(&self, rudolph: &Rudolph, next_pos:Coords)-> Direction{
        let dir_indicator = rudolph.position - next_pos;
        match rudolph.direction{
            Direction::North =>{if dir_indicator.x == 0{
                return Direction::North;
                }
                if dir_indicator.x.is_negative(){
                    return Direction::East;
                }
                return Direction::West;
            },
            Direction::South =>{if dir_indicator.x == 0{
                return Direction::South;
                }
                if dir_indicator.x.is_negative(){
                    return Direction::East;
                }
                return Direction::West;
            },
            Direction::East =>{if dir_indicator.y == 0{
                return Direction::East;
                }
                if dir_indicator.y.is_negative(){
                    return Direction::South;
                }
                return Direction::North;
            },
            Direction::West =>{if dir_indicator.y == 0{
                return Direction::East;
                }
                if dir_indicator.y.is_negative(){
                    return Direction::South;
                }
                return Direction::North;
            },
        }
    }
}
fn main() {
    let path = "./data/data";
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
