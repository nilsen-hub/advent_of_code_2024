use std::{fs::read_to_string, ops::Index, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/test";
    let full_data = get_list_from_file(path);
    babbage(full_data);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>){
    for line in full_data{
        // Clone line, we will mangle it, and we will need it later.
        let mut mul_strings: Vec<&str> = Vec::with_capacity(256);
        let mut mul_locations: Vec<usize> = Vec::with_capacity(256);
        let mut workbench = line.clone();
        loop {
            // first get a list of starting indices of "mul("
            // will hopefully narrow down the search a bit
            let mul = match workbench.find("mul(") {
                Some(val) => val,
                None => break,
            };
            let mut counter: usize = 10;
            let mut string: &str = String::new();
            'one: loop{
                string = match workbench.get(mul..mul + counter) {
                    Some(val) => {val; 
                        break 'one;} 
                    None => {counter -= 1; 
                        continue;}
                };
            }
            
            // drain charavters up to and including mul, avoiding
            // infinite loops
            workbench.drain(0..mul + 1);
            println!("{:?}", mul_locations);
        }
        for el in mul_locations
    }
    
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
