use std::{fs::read_to_string, time::Instant, usize};
type Coords = (i128, i128);

#[derive(Debug, Clone,)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}
impl ClawMachine {
    fn swap_x_y(&mut self){
        self.a = (self.a.1, self.a.0);
        self.b = (self.b.1, self.b.0);
        self.target = (self.target.1, self.target.0);
    }
}
#[derive(Debug, Clone)]
struct Solver {
    machine: ClawMachine,
    tokens_used: i128,
}

impl Solver {
    fn solve_copy(&mut self){
        let target = self.machine.target;
        let a = self.machine.a;
        let b = self.machine.b;
        let mut itercount = 0;
        for index in 0..100 {
            itercount += 1;
            let position: Coords = (a.0 * index, a.1 * index);
            let bx_to_go = target.0 - position.0;
            if bx_to_go % b.0 == 0 {
                let b_presses = bx_to_go / b.0;
                if (b.1 * b_presses) + position.1 == target.1 {
                    self.tokens_used = (index * 3) + b_presses;
                    break;
                }
            }
        }
        println!("itercount: {itercount}");
    }
    fn solve(&mut self) {
        let target = self.machine.target;
        let a = self.machine.a;
        let b = self.machine.b;

        let lcm = lcm(a.0, b.0);
        let a_increment = lcm/a.0;
        let b_increment = lcm/b.0;
        let mut increment: i128 = 1;

        let mut first_y: i128 = 0;
        let mut index: i128 = 0;
        let mut itercount: i128 = 0;
        loop{
            itercount += 1;
            let position: Coords = (a.0 * index, a.1 * index);
            
            if position > target{
                println!("itercount: {itercount}");
                break;
            }
            let bx_to_go = target.0 - position.0;
            if bx_to_go % b.0 == 0 {
                increment = a_increment;
                let b_presses = bx_to_go / b.0;
                let current_y = (b.1 * b_presses) + position.1;
                if first_y == 0{
                    first_y = current_y;
                } else {
                    let y_delta = first_y.abs_diff(current_y) as i128;
                    let diff = target.1.abs_diff(current_y) as i128;
                    if diff % y_delta.abs() == 0{
                        let remaining_increments = diff.abs() / y_delta.abs();
                        let remaining_a_presses= remaining_increments * a_increment;
                        let remaining_b_presses= remaining_increments * b_increment;
                        let calc = (remaining_a_presses * a.1) + (remaining_b_presses * b.1);
                        //println!("apre: {}", remaining_a_presses);
                        //println!("bpre: {}", remaining_b_presses);
                        //println!("rema: {}", remaining_increments);
                        //println!("diff: {}", diff);
                        //println!("calc: {}", calc);
                        //println!("");
                        let one_cycle = a.1 * a_increment + b.1 * b_increment;
                        println!("cycle: {}  delta: {}", one_cycle, y_delta);
                        if current_y < target.1 {
                            println!("current y + diff: {} diff: {} ", current_y + diff.abs(), diff.abs());
                        } else {
                            println!("current y - diff: {} diff: {} ", current_y - diff.abs(), diff.abs());
                        }
                        
                        
                        index += remaining_increments.abs() * increment;
                        //println!("index: {}", index);
                        break; 
                    }
                    break;
                }
                //}
                if (b.1 * b_presses) + position.1 == target.1 {
                    println!("itercount: {itercount}");
                    self.tokens_used = (index * 3) + b_presses;
                    break;
                }
            }
            index += increment;
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
        let mut target = self.get_coords(&proto_claw[0]);
        (target.0, target.1) = (target.0 + 10000000000000, target.1 + 10000000000000);
        let output = ClawMachine {
            a: self.get_coords(&proto_claw[2]),
            b: self.get_coords(&proto_claw[1]),
            target,
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
    let path = "./data/test";
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
        solver.solve();
        if solver.tokens_used > 0 {
            acc += solver.tokens_used;
        }
    }
    acc
}
fn gcd_more(numbers: &mut Vec<i128>) -> i128 {
    // will panic if less than two elements in numbers
    let mut gcd_more = gcd(numbers.pop().unwrap(), numbers.pop().unwrap());
    for i in numbers{
        gcd_more = gcd(gcd_more, *i);
    }
    gcd_more
}
fn gcd(a:i128, b:i128) -> i128 {
    if b == 0{
        return a
    } else {
        return gcd(b,a % b)
    }
}
fn lcm(a:i128, b:i128) -> i128 {
    return  a*(b/gcd(a,b));
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
