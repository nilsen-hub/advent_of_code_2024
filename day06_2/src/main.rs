use std::{collections::HashSet, fs::read_to_string, time::Instant};
#[derive(Debug, Clone, PartialEq)]
struct Agent {
    x: usize,
    y: usize,
    dir: usize,
}
type Field = Vec<Vec<char>>;
fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc: usize = 0;
    let field = parse(full_data);
    let mut field_copy = field.clone();
    let (x, y): (usize, usize) = find_start(&field);
    let agent = Agent { x, y, dir: 0 };
    let path_store = path_recorder(&field, &agent);
    for spot in path_store {
        let shelf = field_copy[spot.1][spot.0];
        field_copy[spot.1][spot.0] = '#';
        if infinite_loop_checker(&field_copy, &agent) {
            acc += 1;
        }
        field_copy[spot.1][spot.0] = shelf;
    }
    acc
}
fn path_recorder(field: &Field, agent: &Agent) -> HashSet<(usize, usize)> {
    let mut agent = agent.clone();
    let mut position_tracker: HashSet<(usize, usize)> = HashSet::with_capacity(10000);
    let bounds = field[1].len();
    loop {
        position_tracker.insert((agent.x, agent.y));
        let mut next_step: (usize, usize) = (agent.x, agent.y);
        match agent.dir {
            0 => {
                next_step = {
                    if agent.y == 0 {
                        return position_tracker;
                    }
                    (agent.x, agent.y - 1)
                }
            }
            1 => {
                next_step = {
                    if agent.x == bounds - 1 {
                        return position_tracker;
                    }
                    (agent.x + 1, agent.y)
                }
            }
            2 => {
                next_step = {
                    if agent.y == bounds - 1 {
                        return position_tracker;
                    }
                    (agent.x, agent.y + 1)
                }
            }
            3 => {
                next_step = {
                    if agent.x == 0 {
                        return position_tracker;
                    }
                    (agent.x - 1, agent.y)
                }
            }
            _ => println!("impossible is nothing"),
        }
        if field[next_step.1][next_step.0] == '#' {
            if agent.dir != 3 {
                agent.dir += 1;
                continue;
            }
            agent.dir = 0;
            continue;
        }
        agent.x = next_step.0;
        agent.y = next_step.1;
    }
}
fn infinite_loop_checker(field: &Field, agent_ref: &Agent) -> bool {
    let mut agent_snapshots: Vec<Agent> = Vec::with_capacity(512);
    let mut agent = agent_ref.clone();
    let bounds = field[1].len();
    loop {
        let mut next_step: (usize, usize) = (agent.x, agent.y);
        match agent.dir {
            0 => {
                next_step = {
                    if agent.y == 0 {
                        return false;
                    }
                    (agent.x, agent.y - 1)
                }
            }
            1 => {
                next_step = {
                    if agent.x == bounds - 1 {
                        return false;
                    }
                    (agent.x + 1, agent.y)
                }
            }
            2 => {
                next_step = {
                    if agent.y == bounds - 1 {
                        return false;
                    }
                    (agent.x, agent.y + 1)
                }
            }
            3 => {
                next_step = {
                    if agent.x == 0 {
                        return false;
                    }
                    (agent.x - 1, agent.y)
                }
            }
            _ => println!("impossible is nothing"),
        }
        if field[next_step.1][next_step.0] == '#' {
            if agent_snapshots.contains(&agent) {
                return true;
            }
            agent_snapshots.push(agent.clone());
            if agent.dir != 3 {
                agent.dir += 1;
                continue;
            }
            agent.dir = 0;
            continue;
        }
        agent.x = next_step.0;
        agent.y = next_step.1;
    }
}
fn find_start(field: &Field) -> (usize, usize) {
    let mut output: (usize, usize) = (0, 0);
    for (idy, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            match c {
                '#' | '.' => continue,
                _ => output = (idx, idy),
            }
        }
    }
    output
}
fn parse(full_data: Vec<String>) -> Field {
    let mut output: Field = Vec::with_capacity(150);
    for line in full_data {
        let string_as_char = line.chars().collect();
        output.push(string_as_char);
    }
    output
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
