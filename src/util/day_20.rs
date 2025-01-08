use std::{collections::VecDeque};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(lines:Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn find_pos(ch:char, map:&Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == ch {
                return (x,y);
            }
        }
    }
    panic!("...")
}

fn generate_map(map:&Vec<Vec<char>>) -> Vec<Vec<i32>> {

    let start_pos = find_pos('S', map);
    let end_pos = find_pos('E', map);
    let mut queue = VecDeque::new();
    let mut visited = vec![];
    
    for _ in 0..map.len() {
        let mut row = vec![];
        for _ in 0..map[0].len() {
            row.push(-1);
        }
        visited.push(row);
    }

    queue.push_back((0, start_pos));

    while !queue.is_empty() {
        let (curr_dist, (x,y)) = queue.pop_front().unwrap();
        
        // Skip
        if map[y][x] == '#' || visited[y][x] >= 0 {
            continue;
        }

        // Update visited
        visited[y][x] = curr_dist;

        if end_pos == (x,y) {
            println!("Done, dist:{}", curr_dist);
            break;
        }

        // Get adjacent
        let adjacent_positions = [(x+1,y),(x-1,y),(x,y+1),(x,y-1)];
        for next in adjacent_positions {
            queue.push_back((curr_dist+1, next));
        }
    }

    visited
}


fn find_cheats(map:&Vec<Vec<char>>, visited:&Vec<Vec<i32>>) -> i32 {
    let start_pos = find_pos('S', map);
    let end_pos = find_pos('E', map);

    let y_max = map.len()-1;
    let x_max = map[0].len()-1;
    let mut queue = VecDeque::new();
    let mut cheats = [0;20_000];
    let mut sum_cheats = 0;
    queue.push_back((0, start_pos));

    while !queue.is_empty() {
        let (curr_dist,(x,y)) = queue.pop_front().unwrap();

        //println!("eval {},{} at dist: {}", x,y, curr_dist);
        if end_pos == (x,y) {
            // Found end
            println!("Found end....");
            break;
        }

        let deltas = [(1,0),(-1,0),(0,0+1),(0,-1)];

        for (dx,dy) in deltas {
            let next_x = (dx + x as i32) as usize;
            let next_y = (dy + y as i32) as usize;
            
            // Calculate next step in maze
            if map[next_y][next_x] != '#' && visited[next_y][next_x] == curr_dist + 1 {
                queue.push_back((curr_dist+1,(next_x,next_y)));
            }

            // Check cheats
            if next_x == 0 || next_x == x_max || next_y == 0 || next_y == y_max {
                // Skip
                continue;
            }

            let next_next_x = (dx*2 + x as i32) as usize;
            let next_next_y = (dy*2 + y as i32) as usize;
            if map[next_y][next_x] == '#' && map[next_next_y][next_next_x] != '#' {
                // Eval cheat
                let next_dist = visited[next_next_y][next_next_x];
                let gain = next_dist - curr_dist - 2;
                if gain > 99 {
                    cheats[gain as usize] += 1;
                }
            }
        }
    }

    for i in 0..cheats.len() {
        if cheats[i] > 0 {
            sum_cheats += cheats[i];
        }
    }
    
    sum_cheats
}


fn part1(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let visited = generate_map(&map);
    find_cheats(&map, &visited).to_string()
}

fn part2(_lines:Vec<&str>) -> String {
    "2".to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("0", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_20.txt");
        assert_eq!("1417", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_20.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
