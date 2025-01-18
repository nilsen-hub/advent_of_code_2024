use std::{
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
    path: Vec<Coords>,
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
        let dur = Duration::from_millis(25);
        self.maze.make_graph();
        let mut counter = 0;
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
        let mut collector: HashMap<Coords, Node> = HashMap::new();
        let mut candidates: BTreeMap<usize, Vec<Node>> = BTreeMap::new();
        loop {
            if frontier.len() == 0 {
                let mut truth: (usize, Vec<Node>) = match candidates.pop_first() {
                    Some(vector) => vector,
                    None => (Default::default(), Default::default()),
                };
                println!("steps: {} length: {}", truth.0, truth.1.len());
                let mut to_print: HashSet<Coords> = HashSet::new();
                for mut node in truth.1 {
                    to_print.extend(self.path_reconstructor(&node));
                }

                self.maze_printer(to_print.clone());
                println!("AMOUNT OF STUFF: {}", to_print.len());
                break;
            }
            for (coords, mut node) in frontier {
                counter += 1;
                if counter % 20000 == 0 {
                    println!("{}", counter);
                }
                if node.dist_fr_start > 74392 {
                    continue;
                }
                node.path.push(node.coords.clone());
                node.visited.insert(node.coords.clone());
                let connected_nodes = self.maze.field_graph.get(&node.coords).unwrap().clone();
                for mut target in connected_nodes {
                    target.visited = node.visited.clone();
                    if target.visited.contains(&target.coords) {
                        continue;
                    }

                    target.direction = self.turn_detector(&node, target.coords);
                    let mut move_distance = target.dist_fr_neigh;
                    if target.direction != node.direction {
                        move_distance += 1000;
                    }
                    target.dist_fr_start = node.dist_fr_start + move_distance;
                    target.path = node.path.clone();

                    if target.coords == self.maze.end {
                        println!("candidate discovered!");
                        target.path.push(target.coords);
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
                        let prev = collector.get(&target.coords).unwrap();
                        if prev.dist_fr_start > target.dist_fr_start {
                            collector.insert(target.coords, target);
                        }
                        continue;
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

    //fn solve_deprec(&mut self) -> usize {
    //    let dur = Duration::from_secs(2);
    //    let mut path_tiles: HashSet<Coords> = HashSet::new();
    //    let mut output = usize::MAX;
    //    self.maze.make_graph();
    //    let mut frontier: BTreeMap<usize, VecDeque<Node>> = BTreeMap::new();
    //    let mut visited: HashMap<Coords, Node> = HashMap::new();
    //    let mut counter: usize = 0;
    //    let mut hit_count: usize = 0;
    //    let mut candidates: BTreeMap<usize, Vec<Node>> = BTreeMap::new();
    //    frontier.insert(
    //        0,
    //        VecDeque::from([Node {
    //            coords: self.maze.start,
    //            dist_fr_neigh: 0,
    //            dist_fr_start: 0,
    //            direction: Direction::East,
    //            path: Vec::new(),
    //        }]),
    //    );
    //    'outer: loop {
    //        let mut current_nodes = match frontier.pop_last() {
    //            Some(vec) => vec.1,
    //            None => break,
    //        };
    //        loop {
    //            match frontier.pop_last() {
    //                Some(mut more_nodes) => current_nodes.append(&mut more_nodes.1),
    //                None => break,
    //            }
    //        }
    //
    //        for mut node in current_nodes {
    //            counter += 1;
    //            if let Some(nod) = visited.get(&node.coords) {
    //                if node.dist_fr_start > nod.dist_fr_start {
    //                    continue;
    //                }
    //            }
    //            visited.insert(node.coords, node.clone());
    //            node.path.push(node.coords);
    //            let connected_nodes = self.maze.field_graph.get(&node.coords).unwrap().clone();
    //
    //            for mut destination in connected_nodes {
    //                let mut move_distance = destination.dist_fr_neigh;
    //                let move_direction = self.turn_detector(&node, destination.coords);
    //                destination.direction = move_direction;
    //                if move_direction != node.direction {
    //                    move_distance += 1000;
    //                }
    //                destination.dist_fr_start = node.dist_fr_start + move_distance;
    //                destination.path = node.path.clone();
    //                if destination.coords == self.maze.end {
    //                    if destination.dist_fr_start <= output {
    //                        destination.path.push(destination.coords);
    //                        candidates
    //                            .entry(destination.dist_fr_start)
    //                            .and_modify(|vec| {
    //                                if !vec.contains(&destination) {
    //                                    vec.push(destination.clone())
    //                                }
    //                            })
    //                            .or_insert(Vec::from([destination.clone()]));
    //                        output = destination.dist_fr_start;
    //                        println!("dist from start: {}", output);
    //                        hit_count += 1;
    //                    }
    //
    //                    //break 'outer;
    //                }
    //                frontier
    //                    .entry(destination.dist_fr_start)
    //                    .and_modify(|vec| {
    //                        if !vec.contains(&destination) {
    //                            vec.push_back(destination.clone())
    //                        }
    //                    })
    //                    .or_insert(VecDeque::from([destination.clone()]));
    //            }
    //        }
    //    }
    //    let shortest = candidates.pop_first().unwrap();
    //    for node in shortest.1 {
    //        path_tiles.extend(self.path_reconstructor(&node));
    //        self.maze_mutator(&path_tiles);
    //        self.maze_printer();
    //        println!("total tiles: {}", path_tiles.len());
    //    }
    //    println!(
    //        "iterations to destination: {} end hit count: {}",
    //        counter, hit_count
    //    );
    //    output
    //}
    fn path_reconstructor(&self, node: &Node) -> HashSet<Coords> {
        let mut source = node.path.clone();
        let mut reconstructed: HashSet<Coords> = HashSet::with_capacity(1000);
        let mut current = source.pop().unwrap();
        loop {
            reconstructed.insert(current);
            let mut next = match source.pop() {
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
    fn path_print(&self, node: &Node) {
        println!("");
        let mut maze = self.maze.field.clone();
        for pos in &node.path {
            maze[pos.y as usize][pos.x as usize] = 'O';
        }
        for line in maze {
            for c in line {
                print!("{c}");
            }
            println!("");
        }
        println!("");
    }
    fn turn_detector(&self, node: &Node, next_pos: Coords) -> Direction {
        let dir_indicator = node.coords - next_pos;
        match node.direction {
            Direction::North => {
                if dir_indicator.x == 0 {
                    return Direction::North;
                }
                if dir_indicator.x.is_negative() {
                    return Direction::East;
                }
                return Direction::West;
            }
            Direction::South => {
                if dir_indicator.x == 0 {
                    return Direction::South;
                }
                if dir_indicator.x.is_negative() {
                    return Direction::East;
                }
                return Direction::West;
            }
            Direction::East => {
                if dir_indicator.y == 0 {
                    return Direction::East;
                }
                if dir_indicator.y.is_negative() {
                    return Direction::South;
                }
                return Direction::North;
            }
            Direction::West => {
                if dir_indicator.y == 0 {
                    return Direction::West;
                }
                if dir_indicator.y.is_negative() {
                    return Direction::South;
                }
                return Direction::North;
            }
        }
    }
    //fn solve_deprec(&mut self) -> usize {
    //    self.maze.make_graph();
    //    let mut counter: usize = 0;
    //    self.to_check
    //        .insert(0, VecDeque::from([self.rudolph.clone()]));
    //    'outer: loop {
    //        let vector_rudolph = self.to_check.pop_first().unwrap();
    //
    //        for rudolph in vector_rudolph.1 {
    //            self.rudolph = rudolph;
    //            if counter % 1000000 == 0 {
    //                println!(
    //                    "counter: {}, rudolph vectors: {}",
    //                    counter,
    //                    self.to_check.len()
    //                );
    //                self.path_print();
    //            }
    //            //println!("iterations: {}", counter);
    //            self.rudolph.path.insert(self.rudolph.position);
    //            if self.rudolph.position == self.maze.end {
    //                break 'outer;
    //            }
    //            let connected_nodes = self.maze.field_graph.get(&self.rudolph.position).unwrap();
    //            for node in connected_nodes {
    //                if self.rudolph.path.contains(&node.coords) {
    //                    continue;
    //                }
    //                let mut move_points = self.rudolph.points + node.dist_fr_neigh;
    //                let move_direction = self.turn_detector(&self.rudolph, node.coords);
    //                if move_direction != self.rudolph.direction {
    //                    move_points += 1000;
    //                }
    //                let priority = move_points;
    //                let rudolph = Rudolph {
    //                    position: node.coords,
    //                    direction: move_direction.clone(),
    //                    points: move_points,
    //                    priority,
    //                    path: self.rudolph.path.clone(),
    //                };
    //                self.to_check
    //                    .entry(self.rudolph.points)
    //                    .and_modify(|vec| vec.push_back(rudolph.clone()))
    //                    .or_insert(VecDeque::from([rudolph]));
    //            }
    //
    //            counter += 1;
    //        }
    //    }
    //    self.path_print();
    //    return self.rudolph.points;
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
