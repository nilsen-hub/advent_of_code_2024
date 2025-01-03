use std::{fs::read_to_string, time::Instant, usize};
type Coords = (i128, i128);

#[derive(Debug, Clone)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}
#[derive(Debug, Clone)]
struct Solver {
    machine: ClawMachine,
    tokens_used: i128,
}

impl Solver {
    fn solve_new(&mut self){
        let target = self.machine.target;
        let a = self.machine.a;
        let b = self.machine.b;

        let mut bxm = target.0 / b.0;
        loop {
            if target.0 - b.0 * bxm % a.0 == 0{
                
            }
            if bxm == 0 {
                break;
            }
            bxm -= 1;
        }       
    }
    fn solve(&mut self) {
        let target = self.machine.target;
        let a = self.machine.a;
        let b = self.machine.b;

        for index in 0..100 {

            let position: Coords = (a.0 * index, a.1 * index);
            let bx_to_go = target.0 - position.0;
            if (b.0 * 100) + position.0 >= target.0 && bx_to_go % b.0 == 0 {
                let b_presses = bx_to_go / b.0;
                if (b.1 * b_presses) + position.1 == target.1 {
                    self.tokens_used = (index * 3) + b_presses;
                    break;
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
struct InputData {
    input: Vec<String>,
}
impl InputData {
    fn get_next(&mut self) -> Option<ClawMachine> {
        let mut proto_claw: Vec<String> = Vec::with_capacity(3);
        proto_claw.push(match self.input.pop() {
            Some(prize) => prize,
            None => return None,
        });
        proto_claw.push(self.input.pop().unwrap());
        proto_claw.push(self.input.pop().unwrap());

        if !self.input.is_empty() {
            self.input.pop().unwrap();
        }

        let output = ClawMachine {
            a: self.get_coords(&proto_claw[2]),
            b: self.get_coords(&proto_claw[1]),
            target: self.get_coords(&proto_claw[0]),
        };

        Some(output)
    }
    fn get_coords(&self, to_clean: &String) -> Coords {
        let string_ch: Vec<char> = to_clean.chars().collect();
        let mut x:i128  = 0;
        let mut y:i128  = 0;
        let mut switch: bool = true;
        let mut skippy: usize = 0;
        for (index, c) in string_ch.iter().enumerate() {
            if skippy > index {
                continue;
            }
            if c.is_numeric() {
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
        return (x, y);
    }
    fn get_int_from_char(&self, chars: &Vec<char>, index: &usize) -> (i128, usize) {
        // returns tuple: (int, end_index + 1)
        let mut temp_number: Vec<char> = Vec::with_capacity(6);
        let mut count = *index;
        loop {
            if count < chars.len() && chars[count].is_numeric() {
                temp_number.push(chars[count]);
                count += 1;
                continue;
            }
            break;
        }
        let as_string: String = temp_number.iter().collect();
        let number:i128  = as_string.parse().unwrap();

        (number, 1 + count)
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(input: Vec<String>) -> i128 {
    let mut acc = 0;
    let mut input = InputData { input };
    loop {
        let mut solver = Solver {
            machine: match input.get_next() {
                Some(machine) => machine,
                None => break,
            },
            tokens_used: 0,
        };
        println!("GCD A: {}, {}", gcd(solver.machine.target.0, solver.machine.a.0), gcd(solver.machine.target.1, solver.machine.a.1));
        println!("GCD B: {}, {}", gcd(solver.machine.target.0, solver.machine.b.0), gcd(solver.machine.target.1, solver.machine.b.1));
        solver.solve();
        if solver.tokens_used > 0 {
            println!("good");
            acc += solver.tokens_used;
        } else {
            println!("bad");
        }
    }
    acc
}
fn gcd(a:i128, b:i128) -> i128 {
    if b == 0{
        return a
    } else {
        return gcd(b,a % b)
    }
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
