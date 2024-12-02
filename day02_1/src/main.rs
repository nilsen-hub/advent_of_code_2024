use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let mut acc: usize = 0;
    for line in full_data {
        let parsed = parse_line(line);
        let analysis = babbage(&parsed);
        if analysis.0 || problem_dampener(&parsed, analysis.1) {
            acc += 1;
            continue;
        }
    }
    println!("Theres {} safe reports", acc);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn problem_dampener(to_dampen: &Vec<usize>, index: usize) -> bool {
    let mut to_remove: [usize; 3] = [index, index + 1, 0];
    if index > 0 {
        to_remove[2] = index - 1;
    }
    for index in to_remove {
        let mut check = to_dampen.clone();
        check.remove(index);
        if babbage(&check).0 {
            return true;
        }
    }
    false
}
fn babbage(to_analysis: &Vec<usize>) -> (bool, usize) {
    let mut exp_flag = false;
    let mut sign_flag = false;
    let mut counter: usize = 0;

    if to_analysis[0] + to_analysis[1] >= to_analysis[0] * 2 {
        exp_flag = true;
    }
    let mut window = to_analysis.windows(2);
    loop {
        let slice = match window.next() {
            Some(val) => val,
            None => break,
        };
        let el = slice[0];
        let next = slice[1];
        sign_flag = el + next >= el * 2;
        if next.abs_diff(el) > 3 || next == el || exp_flag != sign_flag {
            return (false, counter);
        }
        counter += 1;
    }

    (true, 0)
}
fn parse_line(line: String) -> Vec<usize> {
    let split: Vec<&str> = line.split_whitespace().collect();
    let mut output: Vec<usize> = Vec::with_capacity(1000);
    for el in split {
        let number: usize = el.parse().unwrap();
        output.push(number);
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
