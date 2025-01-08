
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


fn match_pattern_rec(towels:&Vec<&str>, pattern:&str, index:usize, cache:&mut Vec<i64>) -> i64 {  

    // init cache if needed
    if cache.is_empty() {
        for _ in 0..pattern.len() {
            cache.push(-1);
        }
    }

    // Check cache if we have a solution
    if cache[index] != -1 {
        return cache[index];
    }

    let mut matches = 0;
    let current_pattern = &pattern[index..];

    for i in 0..towels.len() {
        let towel = towels[i];

        if towel.eq(current_pattern) {
            matches += 1;
        } else if current_pattern.starts_with(&towel){
            // Match, make rec call
            let next_index = index + towel.len();
            matches += match_pattern_rec(towels, pattern, next_index, cache);
        }
    }

    // populate cache
    cache[index] = matches;

    return matches;
}


fn part1(lines:Vec<&str>) -> String {
    let (towels, patterns) = parse(lines);
    patterns.iter()
        .filter(|pattern| match_pattern_rec(&towels, pattern, 0, &mut vec![]) > 0)
        .count()
        .to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let (towels, patterns) = parse(lines);
    patterns.iter()
        .map(|pattern| match_pattern_rec(&towels, pattern, 0, &mut vec![]))
        .sum::<i64>()
        .to_string()
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
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("300", solve(input.to_string(), Part1));
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
        assert_eq!("16", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("624802218898092", solve(input.to_string(), Part2));
    }
}
