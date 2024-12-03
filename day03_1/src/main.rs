use std::{default, fs::read_to_string, ops::Index, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("Answer: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc: usize = 0;
    for line in full_data {
        let mut mul_strings: Vec<&str> = Vec::with_capacity(256);
        let mut workbench = line.clone();
        'outer: loop {
            let mul = match workbench.find("mul(") {
                Some(val) => val,
                None => break,
            };
            let mut counter: usize = 15;
            let mut string: String = loop {
                match workbench.get((mul + 4)..mul + counter) {
                    Some(val) => break val.to_owned(),
                    None => {
                        counter -= 1;
                        continue;
                    }
                };
            };
            let chars: Vec<char> = string.chars().collect();
            let mut left_c: String = String::new();
            let mut right_c: String = String::new();
            let mut direction_flag = false;
            for c in chars {
                if c.is_numeric() && direction_flag == false {
                    left_c.push(c);
                    continue;
                } else if c == ',' {
                    direction_flag = true;
                    continue;
                }
                if c.is_numeric() && direction_flag == true {
                    right_c.push(c);
                    continue;
                } else if c == ')' {
                    workbench.drain(0..mul + 1);
                    break;
                }
                workbench.drain(0..mul + 1);
                continue 'outer;
            }
            let left: usize = left_c.parse().unwrap();
            let right: usize = right_c.parse().unwrap();
            acc += (left * right);
        }
    }
    acc
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
    fn passes_example() {
        let full_data: Vec<String> = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned(),
        ];
        let answer = babbage(full_data);
        let expected: usize = 161;

        assert_eq!(answer, expected);
    }
}
