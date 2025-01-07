use std::{fs::read_to_string, time::Instant, usize};
type Coords = (usize, usize);

#[derive(Debug, Clone)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}
impl ClawMachine {
    fn find_bxmod_zero(&self, a_presses:usize) -> usize{
        let delta_x_target = self.target.0 - (self.a.0 * a_presses);
        if  delta_x_target % self.b.0 == 0{
            return a_presses;
        }
        return self.find_bxmod_zero(a_presses + 1);
    }
    fn solve(&self) -> usize {

        let target = self.target;
        let a = self.a;
        let b = self.b;

        // this removes machines that will never reach bx_mod == 0 + some more
        if target.0 % gcd(a.0, b.0) != 0 || target.1 % gcd(a.1, b.1) != 0{
            return 0;
        }

        // First we figure out how many a presses we need to make the 
        // amount of b presses needed fit into mod delta target/x = 0
        let mut a_presses = self.find_bxmod_zero(0);

        // Now that we have our X, we ned to get Y into line.
        // Position holds state of Y-side
        let mut position = (a.0 * a_presses, a.1 * a_presses);
        
        // Start by calculating how much Y moves pr. LCM derived increment
        let lcm = lcm(a.0, b.0);
        let a_increment = lcm / a.0;
        let b_increment = lcm / b.0;
        
        let mut b_presses = (target.0 - position.0) / b.0;
        let current_y = (b.1 * b_presses) + position.1;
        let increment_y = (a_increment*a.1).abs_diff(b_increment * b.1);

        // Now that we have the Y increment value, we can test if it fits neatly
        // into the delta between the current y value and the target y value.
        // If it fails, we can discard the Claw Machine.
        let delta_y_target = current_y.abs_diff(target.1);
        if delta_y_target % increment_y != 0 {
            return 0;
        }
        // To get the solution, we need to know how many a presses are needed
        // to reach our target Y, first the a presses, luckily this is quite simple:
        let iter_to_go = delta_y_target / increment_y;
        a_presses += a_increment * iter_to_go;

        // then update the position to figure out how many b presses are needed
        // we could calculate b presses directly, but I think the added clarity
        // outwheighs the cost of the extra step
        position.0 = a.0 * a_presses;
        b_presses = (target.0 - position.0) / b.0;
        // then we return the answer
        return (a_presses * 3) + b_presses;
    }
}

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    pub fn get_machines(&self) -> Vec<ClawMachine> {
        let mut input_iter = self.input.lines();
        let mut claw_machines = Vec::with_capacity(400);
        let mut machine_coords: [Coords; 3] = [Coords::default(); 3];
        loop {
            for index in 0..3 {
                let part = match input_iter.next() {
                    Some(part) => part,
                    None => panic!("There should be data here"),
                };
                match index {
                    0 | 1 => machine_coords[index] = self.get_button(part),
                    _ => machine_coords[index] = self.get_target(part),
                };
            }
            claw_machines.push(ClawMachine {
                a: machine_coords[0],
                b: machine_coords[1],
                target: machine_coords[2],
            });
            match input_iter.next() {
                Some(_thing) => continue,
                None => break,
            }
        }
        claw_machines
    }
    fn get_target(&self, target: &str) -> Coords {
        let split: Vec<&str> = target.split('=').collect();
        let x: usize = split[1].strip_suffix(", Y").unwrap().parse().unwrap();
        let y: usize = split[2].parse().unwrap();
        (x + 10000000000000, y + 10000000000000)
    }
    fn get_button(&self, button: &str) -> Coords {
        let split: Vec<&str> = button.split('+').collect();
        let x: usize = split[1].strip_suffix(", Y").unwrap().parse().unwrap();
        let y: usize = split[2].parse().unwrap();
        (x, y)
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = match read_to_string(path) {
        Ok(data) => data,
        Err(_) => panic!("There should be data here"),
    };
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(input: String) -> usize {
    let mut acc = 0;
    let input = InputData { input };
    let machines = input.get_machines();
    for machine in machines {
        acc += machine.solve();
    }
    acc
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
