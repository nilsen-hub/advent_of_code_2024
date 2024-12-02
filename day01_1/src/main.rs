use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let analytical_result = babbage(full_data);
    println!("The answer is: {}", analytical_result);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc: usize = 0;
    let mut left_list: Vec<usize> = Vec::with_capacity(1000);
    let mut right_list: Vec<usize> = Vec::with_capacity(1000);
    for el in full_data {
        let problem = parse_line(&el);
        left_list.push(problem[0]);
        right_list.push(problem[1]);
    }
    left_list.sort_unstable();
    right_list.sort_unstable();
    for (index, el) in left_list.iter().enumerate() {
        acc += el.abs_diff(right_list[index]);
    }
    acc
}
fn parse_line(line: &String) -> [usize; 2] {
    let mut output: [usize; 2] = [0, 0];
    let numbers: Vec<&str> = line.split_whitespace().collect();
    for (index, el) in numbers.iter().enumerate() {
        let number = el.parse().unwrap();
        output[index] = number;
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
