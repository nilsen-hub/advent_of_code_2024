use std::{collections::BTreeMap, fs::read_to_string, time::Instant, usize};
type Coords = (usize, usize);

#[derive(Debug, Clone)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}
impl ClawMachine {
    fn is_solveable(&self) -> bool {
        let ax = is_coprime(self.a.0, self.target.0);
        let ay = is_coprime(self.a.1, self.target.1);
        let bx = is_coprime(self.b.0, self.target.0);
        let by = is_coprime(self.b.1, self.target.1);

        if ax && ay {
            return false;
        }
        if bx && by {
            return false;
        }
        if ax && bx {
            return false;
        }
        if ay && by {
            return false;
        }

        true
    }
}
#[derive(Debug, Clone)]
struct Solver {
    machine: ClawMachine,
    tokens_used: usize,
}

impl Solver {
    fn solve(&mut self){
        
    }
    fn solve_deprec(&mut self) {
        let target = self.machine.target;
        let a = self.machine.a;
        let b = self.machine.b;

        let mut tar_x_factors = prime_factorization(target.0);
        let mut simplified_x_target: usize = 1;

        loop {
            let factor = tar_x_factors.pop_first().unwrap();
            if tar_x_factors.len() == 1{
                break
            }
            simplified_x_target *= factor.0.pow(factor.1 as u32);
        }
        let multiplier = tar_x_factors.pop_first().unwrap().0;
        for index in 0..10000000 {
            let position: Coords = (a.0 * index, a.1 * index);
            if position.0 > simplified_x_target{
                break;
            }
            let bx_to_go = simplified_x_target - position.0;
            if bx_to_go % b.0 == 0 {
                let b_presses = (bx_to_go / b.0) * multiplier;
                if (b.1 * b_presses) + (position.1 * multiplier) == target.1 {
                    self.tokens_used = ((index * 3) + b_presses) * multiplier;
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

        let mut target = self.get_coords(&proto_claw[0]);

        target.0 += 10000000000000;
        target.1 += 10000000000000;

        let output = ClawMachine {
            a: self.get_coords(&proto_claw[2]),
            b: self.get_coords(&proto_claw[1]),
            target,
        };

        Some(output)
    }
    fn get_coords(&self, to_clean: &String) -> Coords {
        let string_ch: Vec<char> = to_clean.chars().collect();
        let mut x: usize = 0;
        let mut y: usize = 0;
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
    fn get_int_from_char(&self, chars: &Vec<char>, index: &usize) -> (usize, usize) {
        // returns tuple: (int, end_index + 1)
        let mut temp_number: Vec<char> = Vec::with_capacity(6);
        let mut count = *index;
        let bounds = chars.len();
        loop {
            if count < bounds && chars[count].is_numeric() {
                temp_number.push(chars[count]);
                count += 1;
                continue;
            }
            break;
        }
        let as_string: String = temp_number.iter().collect();
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
    println!("program runtime: {}", now.elapsed().as_secs_f64());
}
fn babbage(input: Vec<String>) -> usize {
    let mut acc = 0;
    let mut input = InputData { input };
    loop {
        let mut solver = Solver {
            machine: match input.get_next() {
                Some(machine) => machine,
                None => break,
            },
            tokens_used: usize::MAX,
        };
        let machine = solver.machine.clone();
        println!("a increments: {}, {}", machine.a.0, machine.a.1);
        println!("b increments: {}, {}", machine.b.0, machine.b.1);
        println!("t coordinates: {}, {}", machine.target.0, machine.target.1);
        let a_0_factors = prime_factorization(machine.a.0);
        let a_1_factors = prime_factorization(machine.a.1);
        let b_0_factors = prime_factorization(machine.b.0);
        let b_1_factors = prime_factorization(machine.b.1);
        let comb_x_fact = prime_factorization(machine.a.0 + machine.b.0);
        let comb_Y_fact = prime_factorization(machine.a.1 + machine.b.1);
        let t_0_factors = prime_factorization(machine.target.0);
        let t_1_factors = prime_factorization(machine.target.1);

        print!("a factors: ");
        for (factor, _freq) in a_0_factors {
            print!("{},", factor);
        }
        print!(": ");
        for (factor, _freq) in a_1_factors {
            print!("{},", factor);
        }
        println!("");

        print!("b factors: ");
        for (factor, _freq) in b_0_factors {
            print!("{},", factor);
        }
        print!(": ");
        for (factor, _freq) in b_1_factors {
            print!("{},", factor);
        }
        println!("");

        print!("target factors: ");
        for (factor, freq) in t_0_factors {
            print!("{}:{}  ", factor, freq);
        }
        print!("| ");
        for (factor, freq) in t_1_factors {
            print!("{}:{}  ", factor, freq);
        }
        println!("");
        println!("");

        //solver.solve();
        if solver.tokens_used != usize::MAX {
            acc += solver.tokens_used;
            // println!("a presses: {} b presses: {}", solver.a_presses, solver.b_presses);
            println!("is solveable");
        } else {
            println!("is not solveable");
        }
        //println!("function is_solveable: {}", solver.machine.is_solveable());
        println!("");
    }
    acc
}
fn is_coprime(num_1: usize, num_2: usize) -> bool {
    let num_1_factors = prime_factorization(num_1);
    let num_2_factors = prime_factorization(num_2);

    for (factor, _freq) in num_1_factors {
        if num_2_factors.contains_key(&factor) {
            return false;
        }
    }

    true
}
fn prime_factorization(mut number: usize) -> BTreeMap<usize, usize> {
    let mut prime_factors: BTreeMap<usize, usize> = BTreeMap::new();

    // Step 1 : Divide by 2
    let mut freq: usize = 0;

    // You can use number % 2 == 0 also,
    // but this method is much more efficient
    while number & 1 == 0 {
        number >>= 1;
        // Again, You can use number /= 2 also,
        // but this is much more efficient
        freq += 1;
    }

    if freq > 0 {
        prime_factors.insert(2, freq);
    }

    // Step 2 : start from 3, and go till square root of number
    let mut i = 3;
    while i * i <= number {
        // Step 3 : Check if i is factor of number
        if number % i == 0 {
            freq = 0;
            while number % i == 0 {
                number /= i;
                freq += 1;
            }
            prime_factors.insert(i, freq);
        }
        i += 2;
    }

    // Step 4 : Check if number become 1 or not
    if number > 1 {
        prime_factors.insert(number, 1);
    }

    return prime_factors;
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
