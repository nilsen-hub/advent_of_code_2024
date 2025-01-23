use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) -> Computer {
        let input = self.input.lines();
        let mut computer = Computer::default();
        for (index, line) in input.into_iter().enumerate() {
            match index {
                0 => {
                    let a: Vec<&str> = line.split_whitespace().collect();
                    computer.register_a = a[2].parse().unwrap();
                }
                1 => {
                    let b: Vec<&str> = line.split_whitespace().collect();
                    computer.register_b = b[2].parse().unwrap();
                }
                2 => {
                    let c: Vec<&str> = line.split_whitespace().collect();
                    computer.register_c = c[2].parse().unwrap();
                }
                3 => (),
                4 => {
                    let p: Vec<&str> = line.split_whitespace().collect();
                    let nums: Vec<&str> = p[1].split(',').collect();
                    for num in nums {
                        computer.program.push(num.parse().unwrap());
                    }
                }
                _ => panic!("Parser should be done by now.."),
            }
        }
        return computer;
    }
}
#[derive(Debug, Clone, Default)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    output: Vec<usize>,
}

impl Computer {
    fn find_equal_setting(&mut self) -> usize {
        // step 1: find correct length of output by raising 2 to the power
        // of multiples of 3 until desired length is reached, we call this value
        // product of nth place exponent or PNPE
        // step 2: find the correct last digit by adding PNPE until final number
        // matches the target
        // step 3: decrement the exponent by 3 and add this new PNPE to the sum
        // until second to final digit matches
        // step 4: repeat until all digits match
        let mut current_index: usize = 0;
        let mut a_reg: usize = 0;
        loop {
            loop {
                self.register_a = a_reg;
                self.cpu();
                if self.program[current_index] == self.output[current_index] {
                    // check if previous digit still matches, if not, run backtrack

                    println!("program: {:?}", self.program);
                    println!("output:  {:?}", self.output);
                    println!("");
                    current_index += 1;
                    if current_index == self.program.len() {
                        println!("program: {:?}", self.program);
                        println!("output:  {:?}", self.output);
                        return a_reg;
                    }
                    self.output.clear();
                    continue;
                }
                a_reg += 2_usize.pow((current_index * 3) as u32);

                //a_reg = 0;
                self.output.clear();
                multiplier += 1;
            }
        }
    }
    fn cpu(&mut self) {
        let mut window: (usize, usize) = (0, 1);

        'main_loop: loop {
            if window.0 >= self.program.len() {
                //print!("output: ");
                for (index, value) in self.output.iter().enumerate() {
                    if index == self.output.len() - 1 {
                        //print!("{}", value);
                        //println!("");
                        break 'main_loop;
                    }
                    //print!("{},", value);
                }
            }
            let opcode = self.program[window.0];
            let operand = self.program[window.1];
            match opcode {
                0 => self.op0_adv(&operand),
                1 => self.op1_bxl(&operand),
                2 => self.op2_bst(&operand),
                3 => {
                    if self.register_a != 0 {
                        window.0 = operand;
                        window.1 = window.0 + 1;
                        continue;
                    }
                }
                4 => self.op4_bxc(),
                5 => self.op5_out(&operand),
                6 => self.op6_bdv(&operand),
                7 => self.op7_cdv(&operand),
                _ => panic!("Cpu shit the bed"),
            }
            window.0 += 2;
            window.1 = window.0 + 1;
        }
    }
    fn op0_adv(&mut self, operand: &usize) {
        // opcode 0
        let operand = self.combo_decoder(operand);
        let denominator = 2_usize.pow(operand as u32);
        self.register_a = self.register_a / denominator;
    }
    fn op1_bxl(&mut self, operand: &usize) {
        // opcode 1
        self.register_b = self.register_b ^ operand;
    }
    fn op2_bst(&mut self, operand: &usize) {
        // opcode 2
        let operand = self.combo_decoder(operand);
        self.register_b = operand % 8;
    }
    fn op4_bxc(&mut self) {
        // opcode 4
        self.register_b = self.register_b ^ self.register_c;
    }
    fn op5_out(&mut self, operand: &usize) {
        // opcode 5
        let operand = self.combo_decoder(operand);
        self.output.push(operand % 8);
    }
    fn op6_bdv(&mut self, operand: &usize) {
        // opcode 6
        let operand = self.combo_decoder(operand);
        let denominator = 2_usize.pow(operand as u32);
        self.register_b = self.register_a / denominator;
    }
    fn op7_cdv(&mut self, operand: &usize) {
        // opcode 7
        let operand = self.combo_decoder(operand);
        let denominator = 2_usize.pow(operand as u32);
        self.register_c = self.register_a / denominator;
    }

    fn combo_decoder(&self, operand: &usize) -> usize {
        // interprets combo operands
        match operand {
            0..=3 => return *operand,
            4 => return self.register_a,
            5 => return self.register_b,
            6 => return self.register_c,
            _ => panic!("should never happen"),
        };
    }

    fn check_output_equality(&self) -> bool {
        if self.output.len() != self.program.len() {
            return false;
        }
        for (index, el) in self.program.iter().enumerate() {
            if *el != self.output[index] {
                return false;
            }
        }
        return true;
    }
}
fn main() {
    let path = "./data/data";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    babbage(input);
}
fn babbage(input: InputData) {
    let now = Instant::now();
    let mut computer = input.parse();
    println!("The answer is: {}", computer.find_equal_setting());
    println!("babbage runtime: {}", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
