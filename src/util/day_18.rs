use std::{cmp::max, collections::{HashMap, VecDeque}};

use super::Part;


pub fn solve(input : String, part: Part) -> String {
    solve_internal(input, part, 1024)
}

fn solve_internal(input : String, part: Part, num_bytes:usize) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines, num_bytes),
        Part::Part2 => part2(lines, num_bytes)
    }
}

fn parse(lines:Vec<&str>, num_bytes:usize) -> (Vec<Vec<char>>, Vec<(usize,usize)>) {
    let mut coordinates = lines.iter()
    .map(|line| line.trim())
    .map(|line| {
            let mut it = line.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            (x,y)
        }).collect::<Vec<(usize,usize)>>();

    let dimension = coordinates.iter()
        .map(|item| max(item.0, item.1))
        .max()
        .unwrap() + 1;
    
    let remaining = coordinates.split_off(num_bytes);
    let mut map = vec![];
    for y in 0..dimension {
        let mut row= vec![];
        for x in 0..dimension {
            if coordinates.contains(&(x,y)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        map.push(row);
    }

    (map, remaining)
}


fn bfs(map:&Vec<Vec<char>>) -> Option<Vec<(usize,usize)>> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    let end_x = map.len() -1;
    let end_y = map.len() - 1;
    let mut distance_to_end = 0;

    queue.push_back((0,0,0));
    while !queue.is_empty() {
        let (dist, x, y) = queue.pop_front().unwrap();

        if visited.contains_key(&(x,y)) {
            continue;
        } else {
            visited.insert((x,y), dist);
        }

        if (x,y) == (end_x, end_y) {
            distance_to_end = dist;
            break;
        }

        // Add adjacent positions
        let mut adjacent = vec![];
        if x > 0 {
            adjacent.push((x-1,y));
        }
        if x < end_x {
            adjacent.push((x+1,y));
        }
        if y > 0 {
            adjacent.push((x,y-1));
        }
        if y < end_y {
            adjacent.push((x,y+1));
        }

        adjacent.into_iter()
            .filter(|pos| map[pos.1][pos.0] != '#')
            .for_each(|(x_next,y_next)| queue.push_back((dist+1, x_next, y_next)));
        
    }

    // Did we find a solution?
    if distance_to_end == 0 {
        return None;
    } else {
        queue.clear();
    }

    // Get optimal path by back tracking
    queue.push_back((distance_to_end, end_x, end_y));
    let mut path = vec![];
    let mut next_distance = distance_to_end;

    while !queue.is_empty() {
        let (dist, x, y) = queue.pop_front().unwrap();

        if next_distance != dist {
            continue;
        }

        path.push((x,y));

        if x == 0 &&  y == 0 {
            return Some(path);
        }

        // Add adjacent positions
        let mut adjacent = vec![];
        if x > 0 {
            adjacent.push((x-1,y));
        }
        if x < end_x {
            adjacent.push((x+1,y));
        }
        if y > 0 {
            adjacent.push((x,y-1));
        }
        if y < end_y {
            adjacent.push((x,y+1));
        }

        next_distance = dist - 1;
        adjacent.into_iter()
            .filter(|pos| visited.contains_key(pos))
            .filter(|pos| *visited.get(pos).unwrap() == next_distance)
            .for_each(|(x_next,y_next)| queue.push_back((next_distance, x_next, y_next)));

    }
    
    None
}



fn part1(lines:Vec<&str>, num_bytes:usize) -> String {
    let (map,_) = parse(lines, num_bytes);
    let path = bfs(&map).unwrap();
    let no_steps = path.len() - 1;
    no_steps.to_string()
}

fn part2(lines:Vec<&str>, num_bytes:usize) -> String {
    let (mut map,remaining) = parse(lines, num_bytes);
    let mut last_optimal_path = vec![];
    for next_pos in remaining {
        let (x,y) = next_pos;

        // Add next byte
        map[y][x] = '#';

        if !last_optimal_path.is_empty() && !last_optimal_path.contains(&(x,y)) {
            // This does not affect the path
            continue;
        }

        // Calc distance
        match bfs(&map) {
            Some(path) => {
                // Found solution
                last_optimal_path = path;
                continue;
            },
            None => {
                // No solution
                return format!("{},{}", x,y);
            },
        }

    }
    "2".to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", solve_internal(input.to_string(), Part1, 12));


    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("294",solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0";

        assert_eq!("6,1", solve_internal(input.to_string(), Part2, 12));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("31,22", solve(input.to_string(), Part2));
    }
}
