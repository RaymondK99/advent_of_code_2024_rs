use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direcion {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x:usize,
    y:usize,
    direction:Direcion,
}

impl Position {
    fn new(x:usize, y:usize, direction:Direcion) -> Position {
        Position{x,y,direction}
    }

    fn forward(&self) -> Position {
        match self.direction {
            Direcion::Up => Position::new(self.x, self.y-1, self.direction),
            Direcion::Down => Position::new(self.x, self.y+1, self.direction),
            Direcion::Left => Position::new(self.x-1, self.y, self.direction),
            Direcion::Right => Position::new(self.x+1, self.y, self.direction),
        }
    }  

    fn turn_left(&self) -> Position {
        let next_dir = match self.direction {
            Direcion::Up => Direcion::Left,
            Direcion::Left => Direcion::Down,
            Direcion::Down => Direcion::Right,
            Direcion::Right => Direcion::Up,
        };
        Position::new(self.x, self.y, next_dir)
    }
    
    fn turn_right(&self) -> Position {
        let next_dir = match self.direction {
            Direcion::Up => Direcion::Right,
            Direcion::Left => Direcion::Up,
            Direcion::Down => Direcion::Left,
            Direcion::Right => Direcion::Down,
        };
        Position::new(self.x, self.y, next_dir)
    }

    fn backwards(&self) -> Position {
        match self.direction {
            Direcion::Up => Position::new(self.x, self.y+1, self.direction),
            Direcion::Down => Position::new(self.x, self.y-1, self.direction),
            Direcion::Left => Position::new(self.x+1, self.y, self.direction),
            Direcion::Right => Position::new(self.x-1, self.y, self.direction),
        }
    } 

}

fn parse(lines:Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}


fn traverse(map:&Vec<Vec<char>>) -> ((i32, Position), HashMap<Position, i32>) {
    let start_state = (0, Position::new(1, map.len() - 2, Direcion::Right));
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse(start_state));
    
    while !queue.is_empty() {
        let Reverse((score, current)) = queue.pop().unwrap();
        let map_item = map[current.y][current.x];

        if map_item == '#' {
            // Continue
            continue;
        } else if map_item == 'E' {
            visited.insert(current, score);
            return ((score, current), visited);
        } 

        if let Some(prev_score) = visited.get(&current) {
            // Continue, we already have a path
            if *prev_score > score {
                visited.insert(current, score);
            } else {
                // There is already a better path
                continue;
            } 
        } else {
            visited.insert(current, score);
        } 

        // push directions
        queue.push(Reverse((score+1, current.forward())));
        queue.push(Reverse((score+1000, current.turn_left())));
        queue.push(Reverse((score+1000, current.turn_right())));
    }

    // No solution
    panic!("...");
} 


fn backtrack(end_pos:Position, visited:&HashMap<Position, i32>) -> HashSet<(usize,usize)> {
    let mut set = HashSet::new();
    let mut queue = vec![];
    queue.push((*visited.get(&end_pos).unwrap(), end_pos));

    while !queue.is_empty() {
        let (score, current_pos) = queue.pop().unwrap();

        set.insert((current_pos.x, current_pos.y));

        let moves = [current_pos.backwards(), current_pos.turn_left(), current_pos.turn_right()];
        for current_move in moves {
            if let Some(back_step_score) = visited.get(&current_move) {
                // We managed to step back
                if current_move.direction == current_pos.direction && *back_step_score == score-1 {
                    queue.push((*back_step_score, current_move));
                } else if current_move.direction != current_pos.direction && *back_step_score == score-1000 {
                    queue.push((*back_step_score, current_move));
                }
            }
        }
    }

    set

}



fn part1(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let ((score,_), _) = traverse(&map);
    score.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let ((_, end_pos), visited) = traverse(&map);
    let set = backtrack(end_pos, &visited);
    set.len().to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", solve(input.to_string(), Part1));
    }

    #[test]
    fn test12() {

        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_16.txt");
        assert_eq!("83432", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";        
        assert_eq!("45", solve(input.to_string(), Part2));
    }

    #[test]
    fn test22() {

        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("64", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        // 488 too high
        let input = include_str!("../../input/input_16.txt");
        assert_eq!("467", solve(input.to_string(), Part2));
    }
}
