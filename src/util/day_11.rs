use std::collections::HashMap;

use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug, Clone, Copy)]
enum Stones {
    Single(u64),
    Two(u64,u64)
}



fn split_stone(stone:u64) -> Stones {
        if stone == 0 {
            return Stones::Single(1);
        } else {
            // digits
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let mult:u64 = 10_u64.pow(digits / 2);
                let left = stone / mult;
                let right = stone - left * mult;
                return Stones::Two(left, right);
            } else {
                return Stones::Single(stone * 2024);
            }
        }
}

fn parse(line:&str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .into_iter()
        .map(|item| item.parse::<u64>().unwrap())
        .collect()
}


fn process_blinks(stones:&Vec<u64>, blinks:u32) -> u64 {
    let mut cache:HashMap<(u64,u32),u64> = HashMap::new();
    stones.iter()
        .map(|stone| process_blinks_recursive(*stone, blinks, &mut cache))
        .sum()

}

fn process_blinks_recursive(stone:u64, blinks:u32, cache:&mut HashMap<(u64,u32),u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let key = (stone, blinks);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let result = match split_stone(stone) {
        Stones::Single(single) => {
            process_blinks_recursive(single, blinks-1, cache)
        },
        Stones::Two(left, right) => {
            let left_result: u64 = process_blinks_recursive(left, blinks-1, cache);
            let right_result: u64 = process_blinks_recursive(right, blinks-1, cache);
            left_result + right_result
        },
    };

    cache.insert(key, result);
    result
}


fn part1(lines:Vec<&str>) -> String {
    let stones = parse(lines.iter().next().unwrap());
    let result = process_blinks(&stones, 25);
    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let stones = parse(lines.iter().next().unwrap());
    let result = process_blinks(&stones, 75);
    result.to_string()

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "125 17";
        assert_eq!("55312", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_11.txt");
        assert_eq!("193269", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "125 17";
        assert_eq!("65601038650482", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_11.txt");
        assert_eq!("228449040027793", solve(input.to_string(), Part2));
    }
}
