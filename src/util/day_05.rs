use std::cmp::Ordering;
use super::Part;


pub fn solve(input : String, part: Part) -> String {
    let lines = input.split("\n\n").collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse_rule(line:&str)  -> (u32,u32){
    let mut it = line.split('|');
    (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
}


fn check_update(update:&Vec<u32>, rules:&Vec<(u32,u32)>) -> bool {
    for i in 0..update.len() - 1 {
        if !rules.contains(&(update[i], update[i+1])) {
            return false;
        }
    }
    true
}

fn sort_and_get_mid(input:&Vec<u32>, rules:&Vec<(u32,u32)>) -> u32 {
    let mut update = input.iter().copied().collect::<Vec<u32>>();
    update.sort_by(|a,b| {
        if rules.contains(&(*a,*b)) {
            return Ordering::Less;
        } else if rules.contains(&(*b,*a)) {
            return Ordering::Greater;
        } else {
            panic!("Cant find ordering for pair");
        }
    } );

    update[update.len() / 2]
}

fn part1(input:Vec<&str>) -> String {
    let mut it = input.into_iter();
    let rules  = it.next().unwrap().lines()
        .map(|line| parse_rule(line))
        .collect::<Vec<(u32,u32)>>();

    let updates:Vec<Vec<u32>> = it.next().unwrap().lines()
        .map(|line| line.split(',').into_iter()
            .map(|item| item.parse::<u32>().unwrap())
            .collect())
        .collect();

    updates.into_iter()
        .filter(|update| check_update(update, &rules))
        .map(| update| update[update.len()/2])
        .sum::<u32>()
        .to_string()
}



fn part2(input:Vec<&str>) -> String {
    let mut it = input.into_iter();
    let rules  = it.next().unwrap().lines()
        .map(|line| parse_rule(line))
        .collect::<Vec<(u32,u32)>>();

    let updates:Vec<Vec<u32>> = it.next().unwrap().lines()
        .map(|line| line.split(',').into_iter()
            .map(|item| item.parse::<u32>().unwrap())
            .collect())
        .collect();

    let unsorted_updates:Vec<Vec<u32>> = updates.into_iter()
            .filter(|update| !check_update(update, &rules))
            .collect();

    unsorted_updates.into_iter()
            .filter(|update| !check_update(update, &rules))
            .map(|update| sort_and_get_mid(&update, &rules))
            .sum::<u32>()
            .to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::*;


    #[test]
    fn test1() {

        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_05.txt");
        assert_eq!("4578", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!("123", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_05.txt");
        assert_eq!("6179", solve(input.to_string(), Part2));
    }
}
