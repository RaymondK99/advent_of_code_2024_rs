use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse_eq(line:&str) -> (i64, Vec<i64>) {
    let mut it = line.split(|ch|  ch == ':' || ch == ' ')
        .into_iter().filter(|item| !item.is_empty())
        .map(|item| item.parse::<i64>().unwrap());

    let result = it.next().unwrap();
    let terms = it.collect();
    (result, terms)
}


fn solve_eq(result:i64, terms:&[i64], acc:i64, part2:bool) -> bool {
    if terms.len() == 0 {
        if acc == result {
            return true;
        } else {
            return false;
        }
    } else {
        let next = terms[0];
        if acc == 0 {
            return solve_eq(result, &terms[1..], next, part2);
        } else if acc > result {
            return false;
        } else {
            if solve_eq(result, &terms[1..], acc * next, part2) || solve_eq(result, &terms[1..], acc + next, part2) {
                return true;
            } else {
                let num_digits = next.to_string().len() as i64;
                let mut left = acc;
                for _ in 0..num_digits {
                    left *= 10;
                }
                return part2 && solve_eq(result, &terms[1..], left + next, part2);
            }
        }
    }
}

fn part1(lines:Vec<&str>) -> String {
    let equations:Vec<(i64, Vec<i64>)> = lines.iter()
        .map(|line| parse_eq(line))
        .collect();
    let mut sum = 0;

    for (test_value, terms) in equations {
        if solve_eq(test_value, terms.as_slice(),  0, false) {
            sum += test_value;
        }
    }

    sum.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let equations:Vec<(i64, Vec<i64>)> = lines.iter()
        .map(|line| parse_eq(line))
        .collect();
    let mut sum = 0;

    for (test_value, terms) in equations {
        if solve_eq(test_value, terms.as_slice(),  0, true) {
            sum += test_value;
        }
    }

    sum.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_07.txt");
        assert_eq!("1582598718861", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!("11387", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_07.txt");
        assert_eq!("165278151522644", solve(input.to_string(), Part2));
    }
}
