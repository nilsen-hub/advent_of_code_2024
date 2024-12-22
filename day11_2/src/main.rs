use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let data = parse(full_data);
    let acc = hashing_it_out(data, 75);
    acc
}
fn hashing_it_out(input: HashMap<usize, usize>, limit: usize) -> usize {
    let mut input_map = input;
    let mut limit = limit;
    while limit > 0 {
        let mut temp_map: HashMap<usize, usize> = HashMap::with_capacity(4000);
        for (stone, amount) in input_map {
            if stone == 0 {
                temp_map
                    .entry(1)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                continue;
            }
            let len = stone.checked_ilog10().unwrap_or(0) + 1;
            if len & 1 == 0 {
                let divisor = 10_usize.pow(len / 2);
                temp_map
                    .entry(stone / divisor)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                temp_map
                    .entry(stone % divisor)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                continue;
            }
            temp_map
                .entry(stone * 2024)
                .and_modify(|count| *count += amount)
                .or_insert(amount);
        }
        input_map = temp_map;
        limit -= 1;
    }
    let acc = input_map.values().sum();
    acc
}
fn parse(full_data: Vec<String>) -> HashMap<usize, usize> {
    let mut output: HashMap<usize, usize> = HashMap::new();
    let split: Vec<&str> = full_data[0].split_whitespace().collect();
    for el in split {
        let num: usize = el.parse().unwrap();
        output
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
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
