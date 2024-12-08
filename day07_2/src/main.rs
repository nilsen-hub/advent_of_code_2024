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
    println!("program runtime: {}", now.elapsed().as_secs_f32());
}
fn babbage(full_data: Vec<String>) -> usize{
    let mut acc = 0;
    for line in full_data{
        let line = parse_line(line);
        println!("processing {} - {:?}", line.left, line.right);
        if is_possible(&line){
            acc += line.left;
        }
    }
    acc
}
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
fn number_cat(left: &usize, right: &usize) -> usize {
    format!("{}{}", left, right).parse().unwrap()
}
fn possibilitiy_expander(line: &Equation, possibility: &Vec<u32>) -> bool{
    let right = &line.right;
    let bounds = right.len();
    let operators = possibility;
    let mut acc: usize = right[0];
    for (index, el) in right.iter().enumerate(){
        if acc > line.left{
            break;
        }
        if index == bounds - 1{
            break;
        }
        match operators[index]{
            0 => acc += right[index + 1],
            1 => acc *= right[index + 1],
            2 => acc = number_cat(&acc, &right[index + 1]),
            _ => println!("Disaster has struck!"),
        }
    }
    if acc == line.left{
        return true
    }
    false
}
fn is_possible(line: &Equation) -> bool {
    let operator_amount = line.right.len() - 1;
    let mut count = 3_u32.pow(operator_amount as u32);
    // level one listing
    loop {
        count -= 1;
        let bin_str = format_radix(count, 3);
        let mut bin_vec: Vec<u32> = vec![0;operator_amount - bin_str.len()];
        for el in bin_str.chars(){
            bin_vec.push(el.to_digit(10).unwrap());
        }
        if possibilitiy_expander(&line, &bin_vec){
            return true
        }
        if count == 0{
            break;
        }
    }
    false
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
    // try to think of combinators to solve this
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
    fn passes_example() {

    }
}
