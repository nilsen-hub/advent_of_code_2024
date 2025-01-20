use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
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
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
struct Node {
    coords: Coords,
    dist_fr_neigh: usize, // Distance from neighbor
    dist_fr_start: usize, // Distance from start tile
    direction: Direction,
    path: HashSet<Coords>,
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
        let directions = [Coords::NORTH, Coords::SOUTH, Coords::EAST, Coords::WEST];
        let mut nodes: Vec<Node> = Vec::with_capacity(5);
        let mut visited: HashSet<Coords> = HashSet::new();
        for direction in directions {
            let mut current_pos = start_pos;
            let mut steps = 0;

            'outer: loop {
                visited.insert(current_pos);
                current_pos = current_pos + direction;
                if self.field[current_pos.y as usize][current_pos.x as usize] == '#' {
                    break;
                }

                steps += 1;

                if self.field[current_pos.y as usize][current_pos.x as usize] == 'E' {
                    let node = Node {
                        coords: current_pos,
                        dist_fr_neigh: steps,
                        dist_fr_start: usize::MAX,
                        direction: Direction::East,
                        path: HashSet::new(),
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
                            path: HashSet::new(),
                        };
                        nodes.push(node);
                        break 'outer;
                    }
                }
            }
        }
        return nodes;
    }
    fn graph_printer(&self) {
        let mut field = self.field.clone();
        for (coord, _nodes) in self.field_graph.clone() {
            field[coord.y as usize][coord.x as usize] = '*';
        }
        for line in field {
            for tile in line {
                if tile == '.' {
                    print!(" ");
                    continue;
                }
                print!("{tile}");
            }
            println!(" ");
        }
    }
    fn point_printer(&self, points: HashSet<Coords>) {
        let mut field = self.field.clone();
        for point in points {
            field[point.y as usize][point.x as usize] = '*';
        }
        for line in field {
            for tile in line {
                if tile == '.' {
                    print!(" ");
                    continue;
                }
                print!("{tile}");
            }
            println!(" ");
        }
    }
    fn path_reconstructor(&self, path: HashSet<Coords>) -> HashSet<Coords> {
        let output: HashSet<Coords> = HashSet::new();

        output
    }
}

#[derive(Debug, Clone)]
struct Solver {
    maze: Maze,
}
impl Solver {
    fn solve(&mut self) -> usize {
        self.maze.make_graph();
        self.maze.graph_printer();
        let mut frontier: BTreeMap<usize, Vec<Node>> = BTreeMap::new();
        let mut visited: HashMap<Coords, Node> = HashMap::new();

        frontier.insert(
            0,
            Vec::from([Node {
                coords: self.maze.start,
                dist_fr_neigh: 0,
                dist_fr_start: 0,
                direction: Direction::East,
                path: HashSet::new(),
            }]),
        );

        loop {
            let current_nodes = match frontier.pop_first() {
                Some(vec) => vec.1,
                None => panic!("This should not happen"),
            };
            for mut node in current_nodes {
                if let Some(nod) = visited.get(&node.coords) {
                    if node.dist_fr_start >= nod.dist_fr_start {
                        continue;
                    }
                }
                let connected_nodes = self.maze.field_graph.get(&node.coords).unwrap().clone();

                for mut destination in connected_nodes {
                    destination.dist_fr_start = node.dist_fr_start + destination.dist_fr_neigh;
                    destination.direction = self.turn_detector(&node, destination.coords);

                    if destination.direction != node.direction {
                        destination.dist_fr_start += 1000;
                    }

                    if destination.coords == self.maze.end {
                        return destination.dist_fr_start;
                    }

                    frontier
                        .entry(destination.dist_fr_start)
                        .and_modify(|vec| vec.push(destination.clone()))
                        .or_insert(Vec::from([destination]));
                }
                visited.insert(node.coords, node);
            }
        }
    }
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
