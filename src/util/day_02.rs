use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn is_safe(levels:&Vec<u32>) -> bool {
    let increasing = levels[0] < levels[1];
    for i in 1..levels.len() {
        let last = levels[i-1];
        let next = levels[i];
        let next_increasing = next > last;
        let delta = next.abs_diff(last);
        if delta == 0 || delta > 3 || next_increasing != increasing {
            return false;
        }
    }

    true
}


fn parse(lines:Vec<&str>) -> Vec<Vec<u32>> {
    let mut list = vec![];
    for line in lines {
        let levels = line.split_ascii_whitespace().into_iter().map(|item| item.parse().unwrap()).collect();
        list.push(levels);
    }
    list
}

fn part1(lines:Vec<&str>) -> String {
    let list = parse(lines);
    list.iter().filter(|levels| is_safe(levels)).count().to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let list = parse(lines);
    let mut sum = 0;
    for mut numbers in list {
        if is_safe(&numbers) {
            sum += 1;
        } else {
            for i in 0..numbers.len() {
                let removed = numbers.remove(i);
                let is_safe = is_safe(&numbers);
                numbers.insert(i, removed);
                if is_safe {
                    sum += 1;
                    break;
                }
            }
        }
    }

    sum.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::*;


    #[test]
    fn test1() {

        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_02.txt");
        assert_eq!("236", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        // 291 => too low
        let input = include_str!("../../input/input_02.txt");
        assert_eq!("308", solve(input.to_string(), Part2));
    }
}
