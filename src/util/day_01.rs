use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(line:&str) -> (i32,i32) {
    let numbers:Vec<&str> = line.split_ascii_whitespace().collect();
    let left = numbers.first().unwrap().parse::<i32>().unwrap();
    let right = numbers.last().unwrap().parse::<i32>().unwrap();
    (left, right)
}

fn part1(lines:Vec<&str>) -> String {
    let mut first:Vec<i32> = lines.iter().map(|line| parse(line)).map(|item| item.0).collect();
    let mut second:Vec<i32> = lines.iter().map(|line| parse(line)).map(|item| item.1).collect();
    first.sort();
    second.sort();

    first.iter().enumerate()
        .map(|(index, left)| left.abs_diff(second[index]))
        .sum::<u32>()
        .to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let first:Vec<usize> = lines.iter().map(|line| parse(line)).map(|item| item.0 as usize).collect();
    let second:Vec<usize> = lines.iter().map(|line| parse(line)).map(|item| item.1 as usize).collect();
    let mut freq_array = [0;100_000];

    for index in second {
        freq_array[index] += 1;
    }

    first.iter().map(|index| index * freq_array[*index])
        .sum::<usize>()
        .to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::*;


    #[test]
    fn test1() {

        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_01.txt");
        assert_eq!("2344935", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_01.txt");
        assert_eq!("27647262", solve(input.to_string(), Part2));
    }
}
