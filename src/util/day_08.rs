use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_map(lines:&Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn get_antennas(map:&Vec<Vec<char>>) -> Vec<((i32, i32), char)>{
    let mut antennas = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = (x as i32,y as i32);
            let ch = map[y][x];
            if ch == '.' {
                continue;
            }
            antennas.push((pos, ch));
        }
    }
    antennas
}


fn calc_anti_node_delta(x:i32,y:i32, x_other:i32, y_other:i32) -> (i32,i32) {
    let delta_x = (x - x_other).abs() as i32;
    let delta_y = (y - y_other).abs() as i32;
    if x < x_other {
        if y < y_other {
            (-delta_x, -delta_y)
        } else {
            (-delta_x, delta_y)
        }
    } else {
        if y < y_other {
            (delta_x, -delta_y)
        } else {
            (delta_x, delta_y)
        }
    }
}

fn calc_anti_nodes(width:i32, height:i32, x:i32, y:i32, dx:i32, dy:i32, part2:bool) -> Vec<(i32,i32)> {
    let mut anti_nodes = vec![];
    let mut next_x = x + dx;
    let mut next_y = y + dy;

    if part2 {
        anti_nodes.push((x,y));
    }
    
    while next_x >= 0 && next_x < width && next_y >= 0 && next_y < height {
        if !anti_nodes.contains(&(next_x,next_y)) {
            anti_nodes.push((next_x,next_y));
        }
        next_x += dx;
        next_y += dy;

        if !part2 {
            break;
        }
    }
    anti_nodes
}

fn get_anti_nodes(width:i32, height:i32,antennas:&[((i32, i32),char)], part2:bool) -> Vec<(i32,i32)> {
    let mut anti_nodes = vec![];
    for i in 0..antennas.len() {
        let ((x,y), curr) = antennas[i];
        for j in i+1..antennas.len() {
            let ((x_other, y_other), other) = antennas[j];
            if other == curr {
                let (dx, dy) = calc_anti_node_delta(x, y, x_other, y_other);
                calc_anti_nodes(width, height, x, y, dx, dy, part2)
                    .into_iter().for_each(|node| anti_nodes.push(node));

                let (dx, dy) = calc_anti_node_delta(x_other, y_other, x, y);
                calc_anti_nodes(width, height, x_other, y_other, dx, dy, part2)
                    .into_iter().for_each(|node| anti_nodes.push(node));   
            }
        }
    }

    anti_nodes.sort();
    anti_nodes.dedup();
    anti_nodes
}


fn part1(lines:Vec<&str>) -> String {
    let map = parse_map(&lines);
    let antennas = get_antennas(&map);
    let anti_nodes = get_anti_nodes(map.len() as i32, map[0].len() as i32, &antennas, false);
    anti_nodes.len().to_string()    
}

fn part2(lines:Vec<&str>) -> String {
    let map = parse_map(&lines);
    let antennas = get_antennas(&map);
    let anti_nodes = get_anti_nodes(map.len() as i32, map[0].len() as i32, &antennas, true);
    anti_nodes.len().to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};

    const INPUT:&str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";


    #[test]
    fn test1() {
        assert_eq!("14", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let real_input = include_str!("../../input/input_08.txt");
        assert_eq!("289", solve(real_input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("34", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let real_input = include_str!("../../input/input_08.txt");
        assert_eq!("1030", solve(real_input.to_string(), Part2));
    }
}
