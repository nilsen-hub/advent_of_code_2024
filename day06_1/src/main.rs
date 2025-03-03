use std::{fs::read_to_string, time::Instant};

struct Agent {
    x: usize,
    y: usize,
    dir: usize,
}

fn main() {
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let now = Instant::now();
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let field = parse(full_data);
    let (x, y): (usize, usize) = find_start(&field);
    let agent = Agent { x, y, dir: 0 };
    let walked_field = walk_n_count(&field, agent);
    let mut acc: usize = 0;
    for line in walked_field {
        for c in line {
            if c == 'x' {
                acc += 1;
            }
        }
    }
    acc
}
fn walk_n_count(field: &Vec<Vec<char>>, mut agent: Agent) -> Vec<Vec<char>> {
    let mut field = field.clone();
    let bounds = field[1].len();
    loop {
        field[agent.y][agent.x] = 'x';
        let mut next_step: (usize, usize) = (agent.x, agent.y);
        match agent.dir {
            0 => {
                next_step = {
                    if agent.y == 0 {
                        return field;
                    }
                    (agent.x, agent.y - 1)
                }
            }
            1 => {
                next_step = {
                    if agent.x == bounds - 1 {
                        return field;
                    }
                    (agent.x + 1, agent.y)
                }
            }
            2 => {
                next_step = {
                    if agent.y == bounds - 1 {
                        return field;
                    }
                    (agent.x, agent.y + 1)
                }
            }
            3 => {
                next_step = {
                    if agent.x == 0 {
                        return field;
                    }
                    (agent.x - 1, agent.y)
                }
            }
            _ => println!("impossible is nothing"),
        }
        if field[next_step.1][next_step.0] == '#' {
            if agent.dir == 3 {
                agent.dir = 0;
            } else {
                agent.dir += 1;
            }
            continue;
        }
        agent.x = next_step.0;
        agent.y = next_step.1;
    }
}
fn find_start(field: &Vec<Vec<char>>) -> (usize, usize) {
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
fn parse(full_data: Vec<String>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(150);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
