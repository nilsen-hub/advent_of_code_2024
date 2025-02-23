use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) {}
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
    let now = Instant::now();
    let mut acc: usize = 0;
    println!("babbage runtime: {}", now.elapsed().as_micros());
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
