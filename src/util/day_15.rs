use std::{collections::VecDeque, usize};

use super::Part;


pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_map(lines:Vec<&str>, part2:bool) -> (Vec<Vec<char>>, VecDeque<char>) {
    let map:Vec<Vec<char>> = lines.iter().filter(|line| line.starts_with('#'))
        .map(|line| {
            let cols = line.chars().collect::<Vec<char>>();
            if part2 {
                let mut res = vec![];
                for ch in cols {
                    if ch == '#' || ch == '.' {
                        res.push(ch);
                        res.push(ch);
                    } else if ch == '@' {
                        res.push('@');
                        res.push('.');       
                    } else if ch == 'O' {
                        res.push('[');
                        res.push(']');       
                    }
                }
                return res;
            } else {
                return cols;
            }
        })
        .collect();

    let path = lines.iter()
        .filter(|line| line.contains(&['v','^','>','<']))
        .flat_map(|line| line.chars())
        .collect();

    (map, path)
}

fn get_start_pos(map:&Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                return (x,y)
            }
        }  
    }
    panic!("Cant find start pos")
}

fn get_next_pos(dir:char, x:usize, y:usize) -> (usize,usize) {
    match dir {
        '>' => (x+1,y),
        '<' => (x-1,y),
        '^' => (x,y-1),
        'v' => (x,y+1),
        _ => panic!("..."),
    }
} 

fn _print_map(map:&Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
    println!();
}


fn traverse(map:&mut Vec<Vec<char>>, mut path:VecDeque<char>, start_x:usize, start_y:usize) {

    let mut x = start_x;
    let mut y = start_y;

    while !path.is_empty() {
        let dir = path.pop_front().unwrap();
        let (next_x, next_y) = get_next_pos(dir, x, y);
        let item = map[next_y][next_x];

        if item == '#' {
            // We cant move, next diretion
            continue;
        } else if item == '.' {
            // update map
            map[y][x] = '.';
            map[next_y][next_x] = '@';

            // free space
            x = next_x;
            y = next_y;
        } else if item == 'O' {
            // Box, check if there is free space
            let mut x_search = next_x;
            let mut y_search = next_y;
            loop {
                let next_item = map[y_search][x_search];
                if next_item == 'O' {
                    // Check next
                    (x_search, y_search) = get_next_pos(dir, x_search, y_search);
                } else if next_item == '#' {
                    // Unable to push box
                    break;
                } else if next_item == '.' {
                    // Free space, lets push the boxes
                    map[y_search][x_search] = 'O';
                    map[next_y][next_x] = '@';
                    map[y][x] = '.';
                    y = next_y;
                    x = next_x;
                    break;
                }
            }
        }
    }


}

fn push_box_verfical(check_only:bool, dir:char, x:usize, y:usize, map:&mut Vec<Vec<char>>) -> bool {

    //println!("Detected box at:{},{}", x,y);

    // check up left
    let next_y = if dir == 'v' {
        y + 1 
    } else {
        y - 1
    };
    
    let left = map[next_y][x];
    let rigth = map[next_y][x+1];
    
    let left_ok = match left {
        '.' => true,
        '#' => false,
        ']' => push_box_verfical(check_only, dir, x-1, next_y, map),
        '[' => push_box_verfical(check_only, dir, x, next_y, map),
        _ => panic!(".."),
    };

    let right_ok = left_ok && match rigth {
        '.' => true,
        ']' => true,
        '#' => false,
        '[' => push_box_verfical(check_only, dir, x+1, next_y, map),
        _ => panic!("unexpected: left:{}, right:{}", left, rigth),
    };

    if left_ok && right_ok {
        // Move boxes
        if !check_only {
            map[y][x] = '.';
            map[y][x+1] = '.';

            map[next_y][x] = '[';
            map[next_y][x+1] = ']';
        }
        return true;
    } else {
        return false;
    }

}


fn push_box_horizontal(dir:char, x:usize, y:usize, map:&mut Vec<Vec<char>>) -> bool {

    //println!("Detected box at:{},{}", x,y);
    // check up left
    let next_x = if dir == '<' {
        x - 1
    } else {
        x + 2
    };

    let next_pos = map[y][next_x];
    let move_ok = match next_pos {
        '.' => true,
        '#' => false,
        ']' => push_box_horizontal(dir, next_x-1, y, map),
        '[' => push_box_horizontal(dir, next_x, y, map),
        _ => panic!(".."),
    };

    if move_ok {
        // Move boxes
        map[y][x] = '.';
        map[y][x+1] = '.';
        
        if dir == '<' {
            map[y][x-1] = '[';
            map[y][x] = ']';
        } else {
            map[y][x+1] = '[';
            map[y][x+2] = ']';
        }

        true
    } else {
        false
    }
}


fn traverse_part2(map:&mut Vec<Vec<char>>, mut path:VecDeque<char>, start_x:usize, start_y:usize) {
    let mut x = start_x;
    let mut y = start_y;

    while !path.is_empty() {
        let dir = path.pop_front().unwrap();
        let (next_x, next_y) = get_next_pos(dir, x, y);
        let item = map[next_y][next_x];

        if item == '#' {
            // We cant move, next diretion
            continue;
        } else if item == '.' {
            // update map
            map[y][x] = '.';
            map[next_y][next_x] = '@';

            // update coordinates
            x = next_x;
            y = next_y;
        } else if item == '[' || item == ']' {
            let push_ok = if dir == '<' {
                push_box_horizontal(dir, next_x-1, next_y, map)
            } else if dir == '>' {
                push_box_horizontal(dir, next_x, next_y, map)
            } else if dir == '^' || dir == 'v' {
                let box_x = if item == '[' {
                    next_x
                } else {
                    next_x-1
                };

                if push_box_verfical(true, dir, box_x, next_y, map) {
                    push_box_verfical(false, dir, box_x, next_y, map)
                } else {
                    false
                }
            } else {
                panic!("...");
            };

            if push_ok {
                // update map
                map[y][x] = '.';
                map[next_y][next_x] = '@';

                // update coordinates
                x = next_x;
                y = next_y;
            }
        }
    }


}


fn part1(lines:Vec<&str>) -> String {
    let (mut map, path) = parse_map(lines, false);
    let (start_x,start_y) = get_start_pos(&map);
    traverse(&mut map, path, start_x, start_y);
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }

    sum.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let (mut map, path) = parse_map(lines, true);
    let (start_x,start_y) = get_start_pos(&map);
    traverse_part2(&mut map, path, start_x, start_y);
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '[' {
                sum += 100 * y + x;
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

        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let input2 = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!("2028", solve(input.to_string(), Part1));
        assert_eq!("10092", solve(input2.to_string(), Part1));


    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_15.txt");
        assert_eq!("1406392", solve(input.to_string(), Part1));
    }



    #[test]
    fn test21() {

        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        assert_eq!("618", solve(input.to_string(), Part2));
    }

    #[test]
    fn test22() {
        let input2 = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!("9021", solve(input2.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_15.txt");
        assert_eq!("1429013", solve(input.to_string(), Part2));
    }
}
