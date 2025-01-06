use std::{fs::read_to_string, time::Instant};
type Coords = (i128, i128);

#[derive(Debug, Clone)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}
impl ClawMachine {
    fn solve(&self) -> i128 {
        let target = self.target;
        let a = self.a;
        let b = self.b;
        let mut a_presses: i128 = 0;
        let a_increment = lcm(a.0, b.0) / a.0;
        let mut increment: i128 = 1;

        loop {
            let position: Coords = (a.0 * a_presses, a.1 * a_presses);
            let bx_to_go = target.0 - position.0;
            if a_presses > 100 {
                return 0;
            }
            if bx_to_go % b.0 == 0 {
                increment = a_increment;
                let b_presses = bx_to_go / b.0;
                if (b.1 * b_presses) + position.1 == target.1 {
                    return (a_presses * 3) + b_presses;
                }
            }
            a_presses += increment;
        }
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
        let x: i128 = split[1].strip_suffix(", Y").unwrap().parse().unwrap();
        let y: i128 = split[2].parse().unwrap();
        (x, y)
    }
    fn get_button(&self, button: &str) -> Coords {
        let split: Vec<&str> = button.split('+').collect();
        let x: i128 = split[1].strip_suffix(", Y").unwrap().parse().unwrap();
        let y: i128 = split[2].parse().unwrap();
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
fn babbage(input: String) -> i128 {
    let mut acc = 0;
    let input = InputData { input };
    let machines = input.get_machines();
    for machine in machines {
        acc += machine.solve();
    }
    acc
}
fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
fn lcm(a: i128, b: i128) -> i128 {
    return a * (b / gcd(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
