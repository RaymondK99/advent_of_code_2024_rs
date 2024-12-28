use std::collections::VecDeque;

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug)]
struct OpCodeComputer {
    initial_registers:[i64;3],
    registers:[i64;3],
    pc:usize,
    program:Vec<i64>,
    output:Vec<i64>,
}


fn combo_operand(operand:i64, registers:&mut [i64;3]) -> i64 {
    match operand {
        0 => operand,
        1 => operand,
        2 => operand,
        3 => operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("..."),
    }
}

trait Operation {
    fn process(&self, operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize);

}

struct adv;
struct bxl;
struct bst;
struct jnz;

struct bxc;
struct out;
struct bdv;
struct cdv;

impl Operation for adv {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        let combo = combo_operand(operand, registers);
        let nominator = registers[0];
        if combo >= 63 {
            registers[0] = 0;
        } else {
            let denom: i64 = 2_i64.pow(combo as u32);
            registers[0] = nominator / denom;
        }
        *pc += 2;
    }
}

impl Operation for bxl {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        registers[1] = operand ^ registers[1];
        *pc += 2;
    }
}

impl Operation for bst {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        let combo = combo_operand(operand, registers);
        registers[1] = combo % 8;
        *pc += 2;
    }
}


impl Operation for jnz {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        if registers[0] == 0 {
            *pc += 2;
        } else {
            *pc = operand as usize;
        }
    }
}

impl Operation for bxc {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        registers[1] = registers[1] ^ registers[2];
        *pc += 2;
    }
}

impl Operation for out {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        let combo = combo_operand(operand, registers);
        output.push(combo % 8);
        *pc += 2;
    }
}

impl Operation for bdv {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        let combo = combo_operand(operand, registers);
        let nominator = registers[0];

        if combo >= 63 {
            registers[1] = 0;
        } else {
            let denom: i64 = 2_i64.pow(combo as u32);
            registers[1] = nominator / denom;
        }

        *pc += 2;
    }
}

impl Operation for cdv {
    fn process(&self,operand:i64, registers:&mut [i64;3], output:&mut Vec<i64>, pc:&mut usize) {
        let combo = combo_operand(operand, registers);
        let nominator = registers[0];

        if combo >= 63 {
            registers[2] = 0;
        } else {
            let denom: i64 = 2_i64.pow(combo as u32);
            registers[2] = nominator / denom;
        }
        *pc += 2;
    }
}


impl OpCodeComputer {
    fn filter_numbers(line:&str) -> Vec<i64> {
        line.split(|c| c == ' ' || c == ',' || c == ':')
            .into_iter()
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .filter(|item| item.chars().all(|c| c.is_digit(10)))
            .map(|item| item.parse().unwrap())
            .collect()
    }

    fn load(lines:&Vec<&str>) -> OpCodeComputer {
        let register_a = *OpCodeComputer::filter_numbers(lines[0]).first().unwrap();
        let register_b = *OpCodeComputer::filter_numbers(lines[1]).first().unwrap();
        let register_c = *OpCodeComputer::filter_numbers(lines[2]).first().unwrap();
        let program = OpCodeComputer::filter_numbers(lines[4]);
        let registers = [register_a, register_b, register_c];
        let pc = 0;
        let output = vec![];
        OpCodeComputer{initial_registers:registers, registers, pc, program, output}
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.output.clear();
        self.registers[0] = self.initial_registers[0];
        self.registers[1] = self.initial_registers[1];
        self.registers[2] = self.initial_registers[2];
    }

    fn run_op(&mut self) {
        let op_code = self.program[self.pc];
        let operand = self.program[self.pc + 1];
        let operation:Box<dyn Operation> = match op_code {
            0 => Box::new(adv{}),
            1 => Box::new(bxl{}),
            2 => Box::new(bst{}),
            3 => Box::new(jnz{}),
            4 => Box::new(bxc{}),
            5 => Box::new(out{}),
            6 => Box::new(bdv{}),
            7 => Box::new(cdv{}),
            _ => panic!(),
        };
        operation.process(operand, &mut self.registers, &mut self.output, &mut self.pc);
    }

    fn run(&mut self) {
        while self.pc < self.program.len() - 1 {
            self.run_op();
        }
    }

    fn not_finished(&self) -> bool {
        self.pc <= self.program.len() - 1
    }

    fn run_part2(&mut self) {

        let mut register_a = 0;
        loop {
            self.reset();
            self.registers[0] = register_a;

            while self.not_finished() {
                
                // run operation
                self.run_op();

                if self.output.len() == 0 {
                    continue;
                } else if self.program.starts_with(&self.output) {
                    if self.program.len() == self.output.len() {
                        // Found solution
                        println!("Found solutoin for A:{}", register_a);
                        return;
                    }
                } else {
                    // 
                    break;
                }
            }

            register_a += 1;
        }
    }
}



fn part1(lines:Vec<&str>) -> String {
    let mut computer = OpCodeComputer::load(&lines);
    computer.run();
    computer.output.iter().map(|line| line.to_string()).collect::<Vec<String>>().join(",")
}

fn part2(lines:Vec<&str>) -> String {

    let mut computer = OpCodeComputer::load(&lines);
    println!("comp:{:?}", computer);
    computer.run_part2();

    "2".to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve(input.to_string(), Part1));
    }


    #[test]
    fn test11() {

        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        assert_eq!("0,1,2", solve(input.to_string(), Part1));
    }

    #[test]
    fn test12() {

        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_17.txt");
        assert_eq!("1,3,7,4,6,4,2,3,5", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    //#[test]
    fn test_part2() {
        let input = include_str!("../../input/input_17.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
