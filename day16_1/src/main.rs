use std::{fs::read_to_string, time::Instant};

type Field = Vec<Vec<char>>;
type Coords = (usize, usize); // coords are (X,Y)

#[derive(Debug, Clone)]
enum Direction{
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) {
        let lines = self.input.lines();

    }
}
#[derive(Debug, Clone)]
struct Maze{
    field: Field,
    start: Coords,
    end: Coords,
}
#[derive(Debug, Clone)]
struct Rudolph{
    position: Coords,
    direction: Direction,
    points: usize,
}
#[derive(Debug, Clone)]
struct Solver{
    maze: Maze,
    rudolph: Rudolph,
}
fn main() {
    let path = "./data/data";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    let answer = babbage(input);
    println!("The answer is: {}", answer);
}
fn babbage(input: InputData) -> usize {
    let mut acc: usize = 0;
    let now = Instant::now();
    println!("babbage runtime: {}", now.elapsed().as_micros());
    return acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_examples() {
    }
}
