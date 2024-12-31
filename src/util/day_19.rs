use std::collections::{HashSet, VecDeque};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(lines:Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut it = lines.into_iter();
    let towels:Vec<&str> = it.next().unwrap().split(',')
        .map(|item| item.trim())
        .collect();

    let patterns = it
        .filter(|line| line.len() > 0)
        .collect();
    (towels, patterns)
}

fn starts_with(pattern:&str, towel:&str) -> bool {
    if pattern.len() < towel.len() {
        return false;
    }

    for i in 0..towel.as_bytes().len() {
        if pattern.as_bytes()[i] != towel.as_bytes()[i] {
            return false;
        }
    }

    true
}

fn match_pattern(towels:&Vec<&str>, pattern:&str) -> i32 {

    println!("trying with pattern:{}", pattern);
    let mut permutations = 0;
    let mut stack = VecDeque::new();
    let mut no_solution_index = HashSet::new();

    stack.push_back((0, vec![]));

    while !stack.is_empty() {
        let (pattern_index, used_towels) = stack.pop_front().unwrap();
        let mut found_solution = false; 

        if no_solution_index.contains(&pattern_index) {
            println!("Skip index:{}", pattern_index);
            continue;
        }

        if pattern_index == pattern.len() {
            permutations += 1;
            continue;
        }

        let remaining_pattern = &pattern[pattern_index..];

        for i in 0..towels.len() {
            if used_towels.contains(&i) {
                //continue;
            }

            // Try to match a towel to remaining pattern
            let towel = towels[i];
            if starts_with(remaining_pattern, towel) {

                //println!("Current pattern:{} matches with towel:{}", remaining_pattern, towel);
                let next_index = pattern_index + towel.len();

                if no_solution_index.contains(&next_index) {
                    // No point searching this..
                } else {

                    let mut next_used_towel_index = used_towels.clone();
                    next_used_towel_index.push(i);
                    stack.push_front((next_index, next_used_towel_index));
                    found_solution = true;
                }
            }
        }

        if !found_solution {
            // Remember..
            no_solution_index.insert(pattern_index);
        }
    }

    return permutations;

}

fn part1(lines:Vec<&str>) -> String {
    let (towels, patterns) = parse(lines);
    let mut count = 0;
    for pattern in patterns {
        if match_pattern(&towels, pattern) > 0 {
            count += 1;
            //break;
        }

    }
    count.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let (towels, patterns) = parse(lines);
    let mut count = 0;
    for pattern in patterns {
        let permutations=  match_pattern(&towels, pattern);
        count += permutations;
    }
    count.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        // 399 => too high
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {


        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
