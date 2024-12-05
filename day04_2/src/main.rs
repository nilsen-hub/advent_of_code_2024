use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("There's {} X-MASes in the input data", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut output_acc: usize = 0;
    let mut field = parse_input(full_data);
    for (index, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if *c == 'M' {
                let mas_amount = find_xmas(field.clone(), index, idx);
                if mas_amount > 0{
                    output_acc += mas_amount;
                }
            }
        }
    }
    output_acc / 2
}
fn find_xmas(field: Vec<Vec<char>>, index: usize, idx: usize) -> usize {
    let mut acc: usize = 0;
    let north: Vec<(usize, usize)> = vec![(index - 1, idx), (index - 2, idx), (index - 3, idx)];
    let north_west: Vec<(usize, usize)> = vec![
        (index - 1, idx + 1),
        (index - 2, idx + 2),
    ];
    let west: Vec<(usize, usize)> = vec![(index, idx + 1), (index, idx + 2)];
    let south_west: Vec<(usize, usize)> = vec![
        (index + 1, idx + 1),
        (index + 2, idx + 2),
    ];
    let south: Vec<(usize, usize)> = vec![(index + 1, idx), (index + 2, idx)];
    let south_east: Vec<(usize, usize)> = vec![
        (index + 1, idx - 1),
        (index + 2, idx - 2),
    ];
    let east: Vec<(usize, usize)> = vec![(index, idx - 1), (index, idx - 2)];
    let north_east: Vec<(usize, usize)> = vec![
        (index - 1, idx - 1),
        (index - 2, idx - 2),
    ];
    let x_directions: Vec<Vec<(usize, usize)>> = vec![
        north_west, south_west, south_east, north_east,
    ];
    let field_north = field[north[1].0][north[1].1];
    let field_south = field[south[1].0][south[1].1];
    let field_west = field[west[1].0][west[1].1];
    let field_east = field[east[1].0][east[1].1];
    for (num, direction) in x_directions.iter().enumerate() {
        let mut mas: String = String::new();
        for index in direction {
            mas.push(field[index.0][index.1]);
        }  
        if mas == "AS" {
            match num {
                0 => {if field_north == 'M' && field_west == 'S' || field_north == 'S' && field_west == 'M'{
                    acc += 1;
                }},
                1 =>{if field_south == 'M' && field_west == 'S' || field_south == 'S' && field_west == 'M'{
                    acc += 1;
                }},
                2 =>{if field_south == 'M' && field_east == 'S' || field_south == 'S' && field_east == 'M'{
                    acc += 1;
                }},
                3 =>{if field_north == 'M' && field_east == 'S' || field_north == 'S' && field_east == 'M'{
                    acc += 1;
                }},
                _ => println!("you should never ever see this.."),
            };
        }
    }

    acc
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
