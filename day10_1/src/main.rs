use std::{collections::HashSet, fs::read_to_string, ops::Range, time::Instant};

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
    let map = make_map(full_data);
    let bounds: Range<usize> = 0..map[0].len();
    for (idy, line) in map.iter().enumerate(){
        for (idx, pos) in line.iter().enumerate(){
            if *pos == 0{
                let found = recursion_excursion((idy, idx), &map, &bounds);
                acc += found.len();
            }
        } 
    }
    acc
}
fn recursion_excursion(coord: (usize, usize), map: &Vec<Vec<usize>>, bounds: &Range<usize>) -> HashSet<(usize, usize)> {
    let mut acc: HashSet<(usize, usize)> = HashSet::new();
    let current = map[coord.0][coord.1];
    if current == 9{
        acc.insert(coord);
        return acc
    }
    let directions: [(usize, usize);4] = [
        (coord.0 - 1, coord.1), //north
        (coord.0  +1, coord.1), //south
        (coord.0, coord.1 + 1), //east
        (coord.0, coord.1 - 1)  //west
    ];
    let mut to_explore:Vec<(usize, usize)> = Vec::with_capacity(4);
    for (y, x) in directions{
        if bounds.contains(&y) && bounds.contains(&x){
            if map[y][x] == current + 1{
                to_explore.push((y,x));
            }
        } 
    }
    if to_explore.len() == 0{
        return acc
    }
    for (y,x) in to_explore{
        let found = recursion_excursion((y,x), map, bounds);
        acc.extend(&found);
    }

    acc
}
    

fn make_map(full_data: Vec<String>) -> Vec<Vec<usize>>{
    let mut output: Vec<Vec<usize>>  = Vec::with_capacity(150);
    // working nine to five to take away the edge
    let fives:Vec<usize> = vec![5;full_data[1].len()+4];
    let nines:Vec<usize> = vec![9;full_data[1].len()+4];
    output.push(nines.clone());
    output.push(fives.clone());
    for line in full_data{
        let mut as_num: Vec<usize> = Vec::with_capacity(150);
        as_num.push(9);
        as_num.push(5);
        for el in line.chars(){
            as_num.push(el.to_digit(10).unwrap() as usize);
        }
        as_num.push(5);
        as_num.push(9);
        output.push(as_num);
    }
    output.push(fives);
    output.push(nines);
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
