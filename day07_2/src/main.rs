use std::{fs::read_to_string, time::Instant};
#[derive(Debug, Clone, Default)]
struct Equation{
    left: usize,
    right: Vec<usize>,
}
fn main() {
    let now = Instant::now();
    let path = "./data/test";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize{
    let mut acc = 0;
    for line in full_data{
        let line = parse_line(line);
        if numberwang(&line){
            acc += line.left;
        }
        let blah = list_possibilities(&line);
    }
    acc
}
fn number_cat(line_: &Equation) -> bool {
    let operator_amount = line_.right.len() - 1;
    println!("original:           {:?}", line_.right);
    let mut count = 2_usize.pow(operator_amount as u32);
    loop {
        count -= 1;
        let mut line = line_.clone();
        let bin_str = format!("{count:b}");
        let mut bin_vec: Vec<char> = vec!['0';operator_amount - bin_str.len()];
        let mut concatenated_numbers: Vec<usize> = Vec::with_capacity(16);
        for el in bin_str.chars(){
            bin_vec.push(el);
        }
        println!("binary vector: {:?}", bin_vec);
        println!("should be original: {:?}", line.right);
        for (index, el) in line.right.iter().enumerate(){
            if index == 0{
                concatenated_numbers.push(*el);
            }
            if index == operator_amount{
                break;
            }
            if bin_vec[index] == '1'{
                let left = concatenated_numbers.pop().expect("This is a number").to_string();
                println!("concat nums: {:?}", concatenated_numbers);
                println!("left: {}", left);
                let right = line.right[index + 1];
                println!("right: {}", right);
                let cat = format!("{}{}", left, right);
                println!("cat: {}", cat);
                let new: usize = cat.parse().unwrap();
                println!("new number {}", new);
                concatenated_numbers.push(new)
            } else {
                concatenated_numbers.push(line.right[index + 1]);
            }
        }
        line.right = concatenated_numbers;
        println!("should have all nums: {:?}", line.right);
        println!("");
        if numberwang(&line){
            return true
        }
        if count == 0{
            break
        }
        
    }
    false
}
fn numberwang(line: &Equation) -> bool {
    let operator_amount = line.right.len() - 1;
    let mut count = 2_usize.pow(operator_amount as u32);
    loop {
        count -= 1;
        let bin_str = format!("{count:b}");
        let mut bin_vec: Vec<char> = vec!['0';operator_amount - bin_str.len()];
        for el in bin_str.chars(){
            bin_vec.push(el);
        }
        let mut acc = 0;
        for (index, el) in line.right.iter().enumerate(){
            if index == 0{
                acc = *el;
            }
            if index == operator_amount{
                break;
            }
            if bin_vec[index] == '0'{
                acc *= line.right[index + 1];
            } else {
                acc += line.right[index + 1];
            }
        }
        if acc == line.left{
            return true
        }
        if count == 0{
            break
        }
        
    }
    false
}
fn list_possibilities(line: &Equation) -> Vec<Vec<u32>> {
    let mut possibiilities_1: Vec<Vec<u32>> = Vec::with_capacity(16000);
    let operator_amount = line.right.len() - 1;
    let mut count = 2_usize.pow(operator_amount as u32);
    println!("original: {:?}", line.right);
    // level one listing
    loop {
        count -= 1;
        let bin_str = format!("{count:b}");
        let mut bin_vec: Vec<u32> = vec![0;operator_amount - bin_str.len()];
        for el in bin_str.chars(){
            bin_vec.push(el.to_digit(10).unwrap());
        }
        possibiilities.push(bin_vec);
        
        if count == 0{
            break;
        }
    }
    let possibilities_2 = possibiilities_1.clone();
    for el in possibilities_2{
        count = 2_usize.pow(operator_amount as u32);
        loop {
            count -= 1;
            let bin_str = format!("{count:b}");
            let mut bin_vec: Vec<u32> = vec![0;operator_amount - bin_str.len()];
            for el in bin_str.chars(){
                bin_vec.push(el.to_digit(10).unwrap());
            }
            let mut extra_vec: Vec<u32> = Vec::new(16);
            for (index, el) in bin_vec.iter().enumerate(){
                extra_vec[index] = *el + 
            }
            
            if count == 0{
                break;
            }
        }    
    }
    possibiilities_1
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
