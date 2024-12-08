use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone, Default)]
struct Equation{
    left: usize,
    right: Vec<usize>,
}
fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize{
    let mut acc = 0;
    for equation in full_data{
        let mut line = parse_line(equation);
        let first = line.right[0];
        let rest: Vec<usize> = line.right.drain(1..).collect();
        if is_possible(line.left, first, &rest){
            acc += line.left;
        }
    }
    acc
}
fn is_possible(target: usize, acc: usize, values: &Vec<usize>) -> bool {
    if values.len() == 0{
        return acc == target;
    }
    let first = values[0];
    let rest: Vec<usize> = values.clone().drain(1..).collect();

    if is_possible(target, acc + first,  &rest){
        return true
    }
    if is_possible(target, acc * first,  &rest){
        return true
    }
    if is_possible(target, number_cat(&acc, &first),  &rest){
        return true
    }
    return false
}
fn number_cat(left: &usize, right: &usize) -> usize {
    let output:usize = format!("{}{}", left, right).parse().unwrap();
    output
} 
fn parse_line(line:String) -> Equation{
    let line: Vec<&str> = line.split(':').collect();
    let right_str: Vec<&str> = line[1].split(" ").collect();
    let mut right:Vec<usize> = Vec::new();
    for (index, el) in right_str.iter().enumerate(){
        if index == 0{
            continue;
        }
        right.push(el.parse::<usize>().unwrap());
    } 
    let output = Equation{
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
