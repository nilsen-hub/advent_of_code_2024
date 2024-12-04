use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("There's {} XMASes in the input data", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut output_acc: usize = 0;
    let field = parse_input(full_data);
    for line in &field {
        println!("{:?}", line);
    }
    for (index, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if *c == 'X' {
                output_acc += find_xmas(field.clone(), index, idx)
            }
        }
    }
    output_acc
}
fn find_xmas(field: Vec<Vec<char>>, index: usize, idx: usize) -> usize {
    let mut output_acc: usize = 0;
    let north: Vec<(usize, usize)> = vec![(index - 1, idx), (index - 2, idx), (index - 3, idx)];
    let north_west: Vec<(usize, usize)> = vec![
        (index - 1, idx + 1),
        (index - 2, idx + 2),
        (index - 3, idx + 3),
    ];
    let west: Vec<(usize, usize)> = vec![(index, idx + 1), (index, idx + 2), (index, idx + 3)];
    let south_west: Vec<(usize, usize)> = vec![
        (index + 1, idx + 1),
        (index + 2, idx + 2),
        (index + 3, idx + 3),
    ];
    let south: Vec<(usize, usize)> = vec![(index + 1, idx), (index + 2, idx), (index + 3, idx)];
    let south_east: Vec<(usize, usize)> = vec![
        (index + 1, idx - 1),
        (index + 2, idx - 2),
        (index + 3, idx - 3),
    ];
    let east: Vec<(usize, usize)> = vec![(index, idx - 1), (index, idx - 2), (index, idx - 3)];
    let north_east: Vec<(usize, usize)> = vec![
        (index - 1, idx - 1),
        (index - 2, idx - 2),
        (index - 3, idx - 3),
    ];
    let directions: Vec<Vec<(usize, usize)>> = vec![
        north, north_west, west, south_west, south, south_east, east, north_east,
    ];
    for direction in directions {
        let mut mas: String = String::new();
        for index in direction {
            mas.push(field[index.0][index.1]);
        }
        if mas == "MAS" {
            output_acc += 1;
        }
    }

    output_acc
}
fn parse_input(full_data: Vec<String>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(146);
    let width = full_data[1].len() + 6;
    let mut count = 3;
    while count > 0 {
        let to_output = vec!['0'; width];
        output.push(to_output);
        count -= 1;
    }

    for line in full_data {
        let to_output = format!("000{}000", line);
        output.push(to_output.chars().collect());
    }
    count = 3;
    while count > 0 {
        let to_output: Vec<char> = vec!['0'; width];
        output.push(to_output);
        count -= 1;
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
