use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs::read_to_string,
    path, thread,
    time::{Duration, Instant},
    usize,
};

type Field = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
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

impl Coords {
    const NORTH: Coords = Coords { x: 0, y: -1 };
    const SOUTH: Coords = Coords { x: 0, y: 1 };
    const EAST: Coords = Coords { x: 1, y: 0 };
    const WEST: Coords = Coords { x: -1, y: 0 };

    fn from(t0: isize, t1: isize) -> Coords {
        return Coords { x: t0, y: t1 };
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    coords: Coords,
    dist_fr_neigh: usize, // Distance from neighbor
    dist_fr_start: usize, // Distance from start tile
    direction: Direction,
    path: Vec<Vec<Coords>>,
    visited: HashSet<Coords>,
}
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) -> Solver {
        let lines = self.input.lines();
        let mut field: Field = Vec::with_capacity(150);
        for line in lines {
            field.push(line.chars().collect());
        }
        let mut end = Coords::from(0, 0);

        for (idx, c) in field[1].iter().enumerate() {
            if *c == 'E' {
                end = Coords::from(idx as isize, 1);
            }
        }
        let start = Coords::from(end.y, end.x);
        return Solver {
            maze: Maze {
                field,
                field_graph: HashMap::new(),
                start,
                end,
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
struct Maze {
    field: Field,
    field_graph: HashMap<Coords, Vec<Node>>,
    start: Coords,
    end: Coords,
}

impl Maze {
    fn make_graph(&mut self) {
        let position = self.start;

        let to_explore = self.get_connected_nodes(position);
        self.field_graph.insert(position, to_explore.clone());

        self.node_crawler(to_explore);
        //println!("graph contains {} nodes", self.field_graph.len());
        //self.graph_printer();
    }
    fn graph_printer(&self) {
        let mut field_copy = self.field.clone();
        for (key, _nodes) in &self.field_graph {
            field_copy[key.y as usize][key.x as usize] = '+';
        }
        for line in &field_copy {
            for c in line {
                print!("{c}");
            }
            println!("");
        }
    }
    fn node_crawler(&mut self, mut to_explore: Vec<Node>) {
        loop {
            let node = match to_explore.pop() {
                Some(node) => node,
                None => return,
            };
            if self.field_graph.contains_key(&node.coords) {
                continue;
            }
            let mut nodes = self.get_connected_nodes(node.coords);
            self.field_graph.insert(node.coords, nodes.clone());
            to_explore.append(&mut nodes);
        }
    }
    fn get_connected_nodes(&self, start_pos: Coords) -> Vec<Node> {
        let mut visited: HashSet<Coords> = HashSet::new();
        visited.insert(start_pos);
        let directions = [Coords::NORTH, Coords::SOUTH, Coords::EAST, Coords::WEST];

        let mut nodes: Vec<Node> = Vec::with_capacity(5);
        for direction in directions {
            let mut current_pos = start_pos;
            let mut steps = 0;
            'outer: loop {
                current_pos = current_pos + direction;
                if self.field[current_pos.y as usize][current_pos.x as usize] == '#' {
                    break;
                }
                visited.insert(current_pos);
                steps += 1;
                if self.field[current_pos.y as usize][current_pos.x as usize] == 'E' {
                    let node = Node {
                        coords: current_pos,
                        dist_fr_neigh: steps,
                        dist_fr_start: usize::MAX,
                        direction: Direction::East,
                        path: Vec::new(),
                        visited: HashSet::new(),
                    };
                    nodes.push(node);
                    continue;
                }
                for next in directions {
                    let check = current_pos + next;
                    if visited.contains(&check) || next == direction {
                        continue;
                    }
                    if self.field[check.y as usize][check.x as usize] == '.' {
                        let node = Node {
                            coords: current_pos,
                            dist_fr_neigh: steps,
                            dist_fr_start: usize::MAX,
                            direction: Direction::East,
                            path: Vec::new(),
                            visited: HashSet::new(),
                        };
                        nodes.push(node);
                        break 'outer;
                    }
                }
            }
        }
        //nodes.sort_by_key(|node| node.dist_fr_neigh);
        //println!("nodes: {:?}", nodes);
        return nodes;
    }
}
#[derive(Debug, Clone)]
struct Rudolph {
    position: Coords,
    direction: Direction,
    points: usize,
    priority: usize,
    path: HashSet<Coords>,
}
#[derive(Debug, Clone)]
struct Solver {
    maze: Maze,
    rudolph: Rudolph,
    to_check: BTreeMap<usize, VecDeque<Rudolph>>,
}
impl Solver {
    fn solve(&mut self) -> usize {
        self.maze.make_graph();
        let mut frontier: HashMap<Coords, Node> = HashMap::new();
        frontier.insert(
            self.maze.start,
            Node {
                coords: self.maze.start,
                dist_fr_neigh: 0,
                dist_fr_start: 0,
                direction: Direction::East,
                path: Vec::new(),
                visited: HashSet::new(),
            },
        );
        //bootstrap vector

        let mut collector: HashMap<Coords, Node> = HashMap::new();
        let mut candidates: BTreeMap<usize, Vec<Node>> = BTreeMap::new();
        loop {
            if frontier.len() == 0 {
                let truth: (usize, Vec<Node>) = match candidates.pop_first() {
                    Some(vector) => vector,
                    None => (Default::default(), Default::default()),
                };
                println!("steps: {} length: {}", truth.0, truth.1.len());
                let mut to_print: HashSet<Coords> = HashSet::new();
                for node in truth.1 {
                    for vec in node.path {
                        to_print.extend(self.path_reconstructor(vec));
                    }
                }

                self.maze_printer(to_print.clone());
                println!("AMOUNT OF STUFF: {}", to_print.len());
                break;
            }
            for (_coords, mut node) in frontier {
                if node.dist_fr_start > 75000000 {
                    continue;
                }
                if node.path.len() == 0 {
                    node.path.push(Vec::from([node.coords]));
                } else {
                    node.path[0].push(node.coords);
                }

                node.visited.insert(node.coords.clone());
                let connected_nodes = self.maze.field_graph.get(&node.coords).unwrap().clone();
                for mut target in connected_nodes {
                    target.visited = node.visited.clone();
                    if target.visited.contains(&target.coords) {
                        continue;
                    }

                    target.direction = self.turn_detector(&node, target.coords);
                    target.dist_fr_start = target.dist_fr_neigh + node.dist_fr_start;

                    if target.direction != node.direction {
                        target.dist_fr_start += 1000;
                    }

                    target.path = node.path.clone();

                    if target.coords == self.maze.end {
                        println!("candidate discovered!");
                        target.path[0].push(target.coords);
                        candidates
                            .entry(target.dist_fr_start)
                            .and_modify(|vec| {
                                if !vec.contains(&target) {
                                    vec.push(target.clone())
                                }
                            })
                            .or_insert(Vec::from([target.clone()]));
                    }
                    if collector.contains_key(&target.coords) {
                        let mut prev = collector.remove(&target.coords).unwrap();
                        prev.path[0].push(prev.coords);
                        target.visited.extend(prev.visited);
                        for el in prev.path {
                            target.path.push(el);
                        }
                        //match prev.dist_fr_start.cmp(&target.dist_fr_start) {
                        //    Ordering::Less => {
                        //        collector.insert(prev.coords, prev);
                        //        continue;
                        //    }
                        //    Ordering::Equal => {
                        //        prev.path[0].push(prev.coords);
                        //        target.visited.extend(prev.visited);
                        //        for el in prev.path {
                        //            target.path.push(el);
                        //        }
                        //    }
                        //    Ordering::Greater => (),
                        //};
                    }

                    collector.insert(target.coords, target);
                }
            }
            frontier = collector.clone();
            //println!("frontier.len {}", frontier.len());
            //if counter > 100000 {
            //    self.maze_printer(frontier.clone());
            //    counter = 0;
            //}

            //thread::sleep(dur);
            collector.clear();
        }
        0
    }
    fn path_reconstructor(&self, node: Vec<Coords>) -> HashSet<Coords> {
        let mut source = node.clone();
        let mut reconstructed: HashSet<Coords> = HashSet::with_capacity(1000);
        let mut current = source.pop().unwrap();
        loop {
            reconstructed.insert(current);
            let next = match source.pop() {
                Some(node) => node,
                None => break,
            };

            let comp = current - next;
            if comp.x == 0 {
                if comp.y.is_negative() {
                    while current.y - next.y != 0 {
                        current.y += 1;
                        reconstructed.insert(current);
                    }
                    continue;
                } else {
                    while current.y - next.y != 0 {
                        current.y -= 1;
                        reconstructed.insert(current);
                    }
                    continue;
                }
            } else {
                if comp.x.is_negative() {
                    while current.x - next.x != 0 {
                        current.x += 1;
                        reconstructed.insert(current);
                    }
                    continue;
                } else {
                    while current.x - next.x != 0 {
                        current.x -= 1;
                        reconstructed.insert(current);
                    }
                    continue;
                }
            }
        }
        reconstructed
    }
    fn maze_mutator(&mut self, path: &HashSet<Coords>) {
        for tile in path {
            self.maze.field[tile.y as usize][tile.x as usize] = '*';
        }
    }
    fn maze_printer(&self, frontier: HashSet<Coords>) {
        let mut field = self.maze.field.clone();
        for tile in frontier {
            field[tile.y as usize][tile.x as usize] = '*';
        }
        for line in &field {
            for tile in line {
                if *tile == '*' {
                    print!("\x1b[0;32m*\x1b[0m");
                    continue;
                }
                if *tile == '.' {
                    print!(" ");
                    continue;
                }
                print!("{tile}");
            }
            println!("");
        }
        println!("");
    }
    //fn path_print(&self, node: &Node) {
    //    println!("");
    //    let mut maze = self.maze.field.clone();
    //    for pos in &node.path {
    //        maze[pos.y as usize][pos.x as usize] = 'O';
    //    }
    //    for line in maze {
    //        for c in line {
    //            print!("{c}");
    //        }
    //        println!("");
    //    }
    //    println!("");
    //}
    fn turn_detector(&self, node: &Node, next_pos: Coords) -> Direction {
        let dir_indicator = node.coords - next_pos;
        use Direction as D;
        match node.direction {
            D::North | D::South => {
                if dir_indicator.x == 0 {
                    return node.direction;
                }
                if dir_indicator.x.is_negative() {
                    return D::East;
                } else {
                    return D::West;
                }
            }
            D::East | D::West => {
                if dir_indicator.y == 0 {
                    return node.direction;
                }
                if dir_indicator.y.is_negative() {
                    return D::South;
                } else {
                    return D::North;
                }
            }
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
fn babbage(input: InputData) {
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
    fn passes_examples() {}
}
