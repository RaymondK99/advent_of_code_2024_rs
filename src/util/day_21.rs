use std::{collections::VecDeque, usize};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

const NUMERIC_PAD:[[char;3];4] = [['7','8','9'],['4','5','6'],['1','2','3'],[' ','0','A']];
const DIRECTIONAL_PAD:[[char;3];4] = [[' ','^','A'],['<','v','>'],[' ',' ',' '],[' ',' ',' ']];
const WIDTH:usize = NUMERIC_PAD[0].len();
const HEIGHT:usize = NUMERIC_PAD.len();


fn find_button(button:char, pad:&[[char;3];4]) -> (usize, usize){
    for y in 0..pad.len() {
        for x in 0..pad[0].len() {
            if pad[y][x] == button {
                return (x,y);
            }
        }
    }

    panic!();
}

fn cost(path:&Vec<char>) -> i32 {
    let mut cost = 1;
    for i in 1..path.len() {
        if path[i-1] != path[i] {
            cost += 1;
        }
    }
    return cost;
}

fn bfs(start_button:char, target_button:char, pad:&[[char;3];4]) -> Vec<char> {
    let mut queue = VecDeque::new();
    let mut solutions = vec![];
    let start_pos = find_button(start_button, pad);
    let (x_end, y_end) = find_button(target_button, pad);

    
    queue.push_back((start_pos, vec![]));
    
    while !queue.is_empty() {
        let (current_pos, mut current_path) = queue.pop_front().unwrap();
        let (x,y) = current_pos;
        let current_button = pad[y][x];

        if current_button == target_button {
            // Found it, let's add 'A' as well
            current_path.push('A');
            solutions.push(current_path);
            continue;
        }

        let mut next_move = vec![];
        // Check left
        if x > 0 && x_end < x {
            next_move.push(((x-1,y),'<'));
        }
        if x < WIDTH-1 && x_end > x {
            next_move.push(((x+1,y),'>'));
        }
        if y > 0 && y_end < y {
            next_move.push(((x,y-1),'^'));
        }
        if y < HEIGHT-1 && y_end > y {
            next_move.push(((x,y+1), 'v'));
        }

        for ((next_x,next_y), direction) in next_move {
            if pad[next_y][next_x] != ' ' {
                let mut next_path:Vec<char> = current_path.iter().copied().collect();
                next_path.push(direction);
                let next_pos = (next_x,next_y);
                let next_item = (next_pos, next_path);
                queue.push_back(next_item);
            }
        }
    }

    solutions.sort_by(|a,b| {
        let cost_a = cost(a);
        let cost_b = cost(b);
        return cost_a.cmp(&cost_b);
    });

    return solutions[0].clone();
}

fn run_robots(incoming_commands:&str) -> usize {
    let code = incoming_commands.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut commands = incoming_commands.chars().collect::<Vec<char>>();

    for i in 0..3 {
        let mut current_button = 'A';
        let mut outgoing_path = vec![];
        let pad = if i == 0 {
            NUMERIC_PAD
        } else {
            DIRECTIONAL_PAD
        };

        for command in commands.iter() {
            let target_button = *command;
            let path = bfs(current_button, target_button, &pad);
            for ch in path {
                outgoing_path.push(ch);
            }
            current_button = target_button;
        }

        let s = outgoing_path.iter().collect::<String>();
        println!("Commands at level:{} => len:{}, {:?}", i, s.len(), s);
        commands = outgoing_path;
    }

    println!("input:{}, code:{}, len:{}", incoming_commands, code, commands.len());
    commands.len() * code
}


fn part1(lines:Vec<&str>) -> String {
    let mut sum  = 0;
    for line in lines {
        let complexity = run_robots(line);
        println!("Compl:{}", complexity);
        sum += complexity;
    }

    sum.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    "2".to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        // too high: 131526
        // too low: 121526
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
