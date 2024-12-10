use std::collections::HashSet;

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direcion {
    Up,
    Down,
    Left,
    Right    
}

impl Direcion {
    fn turn_right(&self) -> Direcion {
        match self {
            Direcion::Up => Direcion::Right,
            Direcion::Left => Direcion::Up,
            Direcion::Down => Direcion::Left,
            Direcion::Right => Direcion::Down,            
        }
    }
}


fn parse_map(input:&Vec<&str>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn get_start_pos(map:&Vec<Vec<char>>) -> (Direcion, (i32,i32)) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = (x as i32, y as i32);
            match map[y][x] {
                '<' => return (Direcion::Left, pos),
                '>' => return (Direcion::Right, pos),
                '^' => return (Direcion::Up, pos),
                'v' => return (Direcion::Down, pos),
                _ => {}
            } 
        }
    }

    panic!("....");
}

fn get_next_pos(dir:Direcion, pos:(i32, i32), map:&Vec<Vec<char>>) -> Option<(i32, i32)> {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let (x,y) = pos;

    let result = match dir {
        Direcion::Up => {
            if y > 0 {
                Some((x, y-1))
            } else {
                None
            }
        }, Direcion::Down => {
            if y < height - 1 {
                Some((x, y+1))
            } else {
                None
            }
        }, Direcion::Left => {
            if x > 0 {
                Some((x-1, y))
            } else {
                None
            }
        }, Direcion::Right => {
            if x < width - 1 {
                Some((x+1, y))
            } else {
                None
            }
        },
    };

    result
}

fn calc_path(map:&Vec<Vec<char>>) -> HashSet<(i32,i32)>{
    let (mut dir, (mut x,mut y)) = get_start_pos(&map);
    let mut set = HashSet::new();

    loop {
        set.insert((x,y));

        match get_next_pos(dir, (x, y), &map) {
            Some( (x_next, y_next)) => {
                let ch = map[y_next as usize][x_next as usize];
                if ch == '#' {
                    dir = dir.turn_right();
                } else {
                    x = x_next;
                    y = y_next;
                }
            },
            None => {
                //println!("x={},y={}", x,y);
                break;
            },
        }
    }

    set
}

fn part1(lines:Vec<&str>) -> String {
    let map = parse_map(&lines);
    let set = calc_path(&map);
    set.len().to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let map = parse_map(&lines);
    let (start_dir, (start_x, start_y)) = get_start_pos(&map);
    let path= calc_path(&map);
    let mut sum = 0;


    for (x_obstacle, y_obstacle) in path {
            let mut dir = start_dir;
            let mut x = start_x;
            let mut y = start_y;    
            let mut current_path = HashSet::new();    
            
            if x_obstacle == start_x && y_obstacle == start_y {
                continue;
            }

            loop {
                if current_path.contains(&(dir, x, y)) {
                    // Found cycle
                    sum += 1;
                    break;
                } else {
                    current_path.insert((dir, x,y));
                }

                match get_next_pos(dir, (x, y), &map) {
                    Some( (x_next, y_next)) => {
                        let ch = map[y_next as usize][x_next as usize];
                        if ch == '#' {
                            dir = dir.turn_right();
                        } else if y_next == y_obstacle as i32 && x_next == x_obstacle as i32 {
                            dir = dir.turn_right();
                        } else {
                            x = x_next;
                            y = y_next;
                        }
                    },
                    None => {
                        break;
                    },
                }
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

        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_06.txt");
        assert_eq!("5239", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_06.txt");
        assert_eq!("1753", solve(input.to_string(), Part2));
    }
}
