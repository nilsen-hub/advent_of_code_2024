use std::{fs::read_to_string, iter::repeat_n, time::Instant};
#[derive(Debug, Clone, Default)]
struct Equation {
    left: usize,
    right: Vec<usize>,
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
    let mut acc = 0;
    for line in full_data {
        let line = parse_line(line);
        if numberwang(&line) {
            acc += line.left;
        }
    }
    acc
}
fn numberwang(line: &Equation) -> bool {
    let operator_amount = line.right.len() - 1;
    let mut count = 2_usize.pow(operator_amount as u32);
    loop {
        count -= 1;
        let mut bin_str = format!("{count:b}");
        let z_pad: String = repeat_n("0", operator_amount - bin_str.len()).collect();
        bin_str = format!("{}{}", z_pad, bin_str);

        let bin_vec: Vec<char> = bin_str.chars().collect();
        let mut acc = 0;
        for (index, el) in line.right.iter().enumerate() {
            if index == 0 {
                acc = *el;
            }
            if index == operator_amount {
                break;
            }
            if bin_vec[index] == '0' {
                acc *= line.right[index + 1];
            } else {
                acc += line.right[index + 1];
            }
        }
        if acc == line.left {
            return true;
        }
        if count == 0 {
            break;
        }
    }
    false
}
fn parse_line(line: String) -> Equation {
    let line: Vec<&str> = line.split(':').collect();
    let right_str: Vec<&str> = line[1].split(" ").collect();
    let mut right: Vec<usize> = Vec::new();
    for (index, el) in right_str.iter().enumerate() {
        if index == 0 {
            continue;
        }
        right.push(el.parse::<usize>().unwrap());
    }
    // try to think of combinators to solve this
    let output = Equation {
        left: line[0].parse().unwrap(),
        right,
    };
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
