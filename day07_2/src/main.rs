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
        //println!("processing {} - {:?}", line.left, line.right);
        if is_possible(&line){
            acc += line.left;
        }
    }
    acc
}
fn number_cat(left: &usize, right: &usize) -> usize {
    format!("{}{}", left, right).parse().unwrap()
}
fn possibilitiy_expander(line: &Equation, possibility: &[usize;15]) -> bool{
    let right = &line.right;
    let bounds = right.len();
    let operators = possibility;
    let mut acc: usize = right[0];
    for (index, _el) in right.iter().enumerate(){
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
    let mut operators: [usize;15] = [0;15];
    let mut count: usize = 3_usize.pow(operator_amount as u32);
    // level one listing
    loop {
        count -= 1;
        if possibilitiy_expander(&line, &operators){
            return true
        }
        operators = n_radix_incrementer(operators, 3);
        if count == 0{
            break;
        }
    }
    false
}
fn n_radix_incrementer(mut number: [usize;15], radix: usize) -> [usize;15] {
    let mut index: usize = 0;
    loop{
        number[index] += 1;
        if number[index] == radix{
            number[index] = 0;
            index +=1;
            continue;
        }
        return number
    }
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
