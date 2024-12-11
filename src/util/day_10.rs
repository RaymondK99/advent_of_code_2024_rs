use std::collections::{HashSet, VecDeque};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(lines:Vec<&str>) -> Vec<Vec<u8>> {
    lines.iter()
        .map(|line| line.as_bytes()
            .iter()
            .copied()
            .map(|b| {
                if b >= 0x30 {
                    b - 0x30
                } else {
                    '.' as u8
                }   
            })
            .collect()).collect()
}


fn get_trail_heads(map:&Vec<Vec<u8>>)-> Vec<(usize, usize, u8)> {
    let mut positions = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                positions.push((x,y, 0));
            }
        }
    }
    positions
}

fn find_height(start:(usize, usize, u8), map:&Vec<Vec<u8>>)  -> u32 {
    let mut result = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let (x,y, height) = queue.pop_front().unwrap();

        if visited.contains(&(x,y)) {
            continue;
        } else {
            visited.insert((x,y));
        }

        if height == 9 {
            // Found height
            result += 1;
            continue;
        }

        // Get neightbours
        let next_nodes = get_next_pos((x,y, height), map);
        for next in next_nodes {
            let (x,y, _) = next;
            queue.push_back(next);
        } 
    }

    result
}


fn get_next_pos(pos:(usize, usize, u8), map:&Vec<Vec<u8>>) -> Vec<(usize, usize, u8)> {
    let (x,y,height) = pos;
    let mut next = vec![];

    // Get neightbours
    let next_height = height + 1;
    if y > 0 && map[y-1][x] == next_height {
        next.push((x,y-1, next_height));
    }
    
    if y < map.len() - 1 && map[y+1][x] == next_height {
        next.push((x,y+1, next_height));
    }
    
    if x > 0 && map[y][x-1] == next_height {
        next.push((x-1,y, next_height));
    }
    
    if x < map[0].len() - 1 && map[y][x+1] == next_height {
         next.push((x+1,y, next_height));
    }

    next
}

fn find_height_distinct(start:(usize, usize, u8), map:&Vec<Vec<u8>>)  -> u32 {
    let mut result = 0;
    let mut queue = VecDeque::new();
    let mut visited:HashSet<Vec<(usize, usize)>> = HashSet::new();
    let path:Vec<(usize, usize)> = vec![(start.0, start.1)];
    queue.push_back((start, path));

    while !queue.is_empty() {
        let (pos, path) = queue.pop_front().unwrap();
        let height = pos.2;
        if visited.contains(&path) {
            continue;
        } else {
            visited.insert(path.iter().copied().collect());
        }

        if height == 9 {
            // Found height
            result += 1;
            continue;
        }

        // Get neightbours
        let next_nodes = get_next_pos(pos, map);
        for next in next_nodes {
            let mut next_path:Vec<(usize, usize)> = path.iter().copied().collect();
            next_path.push((next.0, next.1));
            queue.push_back((next, next_path));
        } 
    }

    result
}


fn part1(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let start_positions = get_trail_heads(&map);
    start_positions.into_iter()
        .map(|start| find_height(start, &map))
        .sum::<u32>()
        .to_string()

}

fn part2(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let start_positions = get_trail_heads(&map);
    start_positions.into_iter()
        .map(|start| find_height_distinct(start, &map))
        .sum::<u32>()
        .to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_10.txt");
        assert_eq!("667", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_10.txt");
        assert_eq!("1344", solve(input.to_string(), Part2));
    }
}
