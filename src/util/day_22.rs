use std::collections::{HashMap, VecDeque};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn calc_step_one(secret:u64) -> u64 {
    ((secret * 64) ^ secret) % 16777216
}

fn calc_step_two(secret:u64) -> u64 {
   ((secret / 32) ^ secret) % 16777216
}

fn calc_step_three(secret:u64) -> u64 {
   ((secret * 2048) ^ secret) % 16777216
}

fn calc_next(secret:u64) -> u64 {
   let first = calc_step_one(secret);
   let second = calc_step_two(first);
   calc_step_three(second)
}

fn calc_iterations(secret:u64, iterations:u64) -> u64 {
    (0..iterations).into_iter().fold(secret, |acc, _| calc_next(acc))
}

fn generate_map(secret:u64) -> HashMap<u32, i8> {
    let mut next = secret;
    let mut map = HashMap::new();
    let mut merged:u32 = 0;
    for i in 0..2000 {
        let price = (next % 10) as i8;
        next = calc_next(next);
        let next_price = (next % 10) as i8;
        let diff = price - next_price;
        let diff_unsigned = (diff as u32) & 0xff;
    
        merged = merged << 8 | diff_unsigned;

        if i >= 3 {
            map.entry(merged).or_insert(next_price);

        }
    }

    map
}

fn part1(lines:Vec<&str>) -> String {
    let numbers = lines.iter().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    numbers.into_iter()
        .fold(0, |acc, next| acc + calc_iterations(next, 2000))
        .to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let mut result_map = HashMap::new();
    let numbers = lines.iter().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for number in numbers {
        let map = generate_map(number);
        for (key, value) in map.into_iter() {
            result_map.entry(key).and_modify(|v| *v += value as i32).or_insert(value as i32);
        }
    }

    result_map.values().map(|v| *v).max().unwrap().to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1
10
100
2024";    
        assert_eq!("37327623", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_22.txt");
        assert_eq!("19150344884", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "1
2
3
2024";
        assert_eq!("23", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_22.txt");
        assert_eq!("2121", solve(input.to_string(), Part2));
    }
}
