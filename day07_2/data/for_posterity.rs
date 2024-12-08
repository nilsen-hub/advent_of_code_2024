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
        let mut possibilities = list_possibilities(&line);
        //if possibilitiy_expander(&line, possibilities){
        //    acc += line.left;
        //}
    }
    acc
}
fn number_cat(left: &usize, right: &usize) -> usize {
    let cat = format!("{}{}", left, right);
    let output: usize = cat.parse().unwrap();
    output
}
fn possibilitiy_expander(line: &Equation, possibilities: Vec<Vec<u32>>) -> bool{
    let right = &line.right;
    let bounds = right.len();
    let mut count = possibilities.len() - 1;
    loop{
        let operators = &possibilities[count];
        let mut acc: usize = 0;
        for (index, el) in right.iter().enumerate(){
            if index == 0 {
                acc = *el;
            }
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
        if count == 0{
            break;
        }
        count -= 1;
    }
    false
}
fn list_possibilities(line: &Equation) -> Vec<Vec<u32>> {
    let mut possibiilities: Vec<Vec<u32>> = Vec::with_capacity(16000);
    let operator_amount = line.right.len() - 1;
    let mut count = 2_usize.pow(operator_amount as u32);
    print!("original: {:?}", line.right);
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
     level two listing, adding all third options
    let possibilities_copy = possibiilities.clone();
    loop{
        
        for el in &possibilities_copy{
            let mut third_alternative: Vec<u32> = Vec::with_capacity(25);
            for (index, c) in el.iter().enumerate(){
                let temp_val = possibiilities[count][index] + *c;
                third_alternative.push(temp_val);
            }
            if third_alternative.contains(&2){
                possibiilities.push(third_alternative);
            }
        }
        count += 1;
        if count == possibilities_copy.len(){
            break;
        }
    }
    print!(" {} possibilities detected", possibiilities.len());
    println!("");
    possibiilities
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
