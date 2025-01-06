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
        let first_mod: i128 = target.0 % b.0;

        let a_increment = lcm(a.0, b.0) / a.0;
        let mut a_presses: i128 = 0;
        let mut bx_mod_counter: i128 = 0;
        let mut position: Coords = (a.0 * a_presses, a.1 * a_presses);

        if target.1 % a.1 == 0 && a.0 * target.1 / a.1 == target.0 {
            return (target.1 / a.1) * 3;
        }
        // this loop solves X
        loop {
            let bx_to_go = target.0 - position.0;
            let bx_mod = bx_to_go % b.0;

            if bx_mod == 0 {
                break;
            }
            if bx_mod == first_mod {
                bx_mod_counter += 1;
                if bx_mod_counter == 3 {
                    return 0;
                }
            }
            a_presses += 1;
            position = (a.0 * a_presses, a.1 * a_presses);
        }
        // Now that we have our X, we ned to get Y into line
        // first we have to find the value of Y at the moment to
        // reference later:
        let mut b_presses = (target.0 - position.0) / b.0;
        let first_y = (b.1 * b_presses) + position.1;

        // then we increment by one LCM derived step
        // update our position and calculate the delta between
        // the current Y value and our reference value.
        // This gives us the value of each LCM increment
        a_presses += a_increment;
        position = (a.0 * a_presses, a.1 * a_presses);
        b_presses = (target.0 - position.0) / b.0;
        let current_y = (b.1 * b_presses) + position.1;
        let increment_y = first_y.abs_diff(current_y) as i128;

        // Now that we have the increment value, we can see if it fits neatly
        // into the delta between current y value and the target value.
        // If it fails, we can discard the Claw Machine. If sucsess, we know
        // the machine will solve
        let delta_y_target = current_y.abs_diff(target.1) as i128;
        if delta_y_target % increment_y != 0 {
            return 0;
        }

        // To get the solution, we need to know how many a presses are needed
        // to reach our target Y, luckily this is quite simple:
        let iter_to_go = delta_y_target / increment_y;
        a_presses += a_increment * iter_to_go;

        // then update the position to figure out how many b presses are needed
        position = (a.0 * a_presses, a.1 * a_presses);
        b_presses = (target.0 - position.0) / b.0;

        // then we return
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
        let x: i128 = split[1].strip_suffix(", Y").unwrap().parse().unwrap();
        let y: i128 = split[2].parse().unwrap();
        (x + 10000000000000, y + 10000000000000)
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
