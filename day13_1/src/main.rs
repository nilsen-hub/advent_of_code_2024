use std::{fs::read_to_string, time::Instant, usize};
type Coords = (usize,usize);


#[derive(Debug, Clone)]
struct ClawMachine{
    a: Coords,
    b: Coords,
    target: Coords,
}
#[derive(Debug, Clone)]
struct Solver{
    machine: ClawMachine,
    presses: usize,
    tokens: usize,
    position: Coords,
}
#[derive(Debug, Clone)]
struct InputData{
    input: Vec<String>
}
impl InputData{
    fn get_next(&mut self) -> Option<ClawMachine> {
        let mut proto_claw: Vec<String> = Vec::with_capacity(3);
        proto_claw.push(match self.input.pop(){
            Some(prize) => prize,
            None => return None,
        });
        proto_claw.push(self.input.pop().unwrap());
        proto_claw.push(self.input.pop().unwrap());
        
        if !self.input.is_empty(){
            self.input.pop().unwrap();
        }
        
        let output = ClawMachine{
            a: self.get_coords(&proto_claw[2]),
            b: self.get_coords(&proto_claw[1]),
            target: self.get_coords(&proto_claw[0]),
        };

        Some(output)
    }
    fn get_coords(&self, to_clean: &String) -> Coords {
        let string_ch: Vec<char> = to_clean.clone().chars().collect();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut switch: bool = true;
        let mut skippy: usize = 0;
        for (index, c) in string_ch.iter().enumerate(){
            if skippy > index{
                continue;
            }
            if c.is_numeric(){
                let temp = self.get_int_from_char(&string_ch, &index);
                if switch {
                    x = temp.0;
                    skippy = temp.1;
                    switch = false;
                    continue;
                }
                y = temp.0;
                break;
            }
        }
        return (x,y)
    }
    fn get_int_from_char(&self, chars: &Vec<char>, index: &usize) -> (usize, usize) {
        // returns tuple: (int, end_index + 1)
        let mut temp_number: Vec<char> = Vec::with_capacity(6);
        let mut count = *index;
        let bounds = chars.len();
        loop{
            if count < bounds && chars[count].is_numeric() {
                temp_number.push(chars[count]);
                count += 1;
                continue;
            }
            break;
        }
        let as_string:String = temp_number.iter().collect();
        let number: usize = as_string.parse().unwrap();
        
        (number, count + 1) 
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/test";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(input: Vec<String>) -> usize{
    let mut acc = 0;
    let mut input = InputData{input,};
    loop{
        let solver = Solver{
            machine: match input.get_next(){
                Some(machine) => machine,
                None => break,
            },
            presses: 0,
            tokens: usize::MAX,
            position: (0,0),
        };
        acc += 1;
        println!("button a: {:?} button b: {:?} target: {:?}", solver.machine.a, solver.machine.b, solver.machine.target);
    }
    acc
}
fn solve(solver: Solver){
    
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
