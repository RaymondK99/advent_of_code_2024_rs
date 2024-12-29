use std::{cmp::max, collections::{HashSet, VecDeque}, mem::Discriminant};

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


fn bfs(map:&Vec<Vec<char>>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let end_x = map.len() -1;
    let end_y = map.len() - 1;

    queue.push_back((0,0,0));
    while !queue.is_empty() {
        let (dist, x, y) = queue.pop_front().unwrap();

        if (x,y) == (end_x, end_y) {
            return Some(dist);
        }

        if visited.contains(&(x,y)) {
            continue;
        } else {
            visited.insert((x,y));
        }

        //println!("Eval dist:{}, x:{},y:{}", dist, x, y);

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

    None
}


fn part1(lines:Vec<&str>, num_bytes:usize) -> String {
    let (map,_) = parse(lines, num_bytes);
    bfs(&map).unwrap().to_string()
}

fn part2(lines:Vec<&str>, num_bytes:usize) -> String {
    let (mut map,remaining) = parse(lines, num_bytes);

    for next_pos in remaining {
        let (x,y) = next_pos;

        // Add next byte
        map[y][x] = '#';

        // Calc distance
        match bfs(&map) {
            Some(_dist) => {
                // Found solution
                //println!("dist:{}", dist);
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
