use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn get_map(lines:&Vec<&str>) -> Vec<Vec<u8>> {
    lines.iter().map(|s| s.as_bytes().iter().copied().collect()).collect()
}


fn find_xmas(map:&Vec<Vec<u8>>, x0:i32, y0:i32, dx:i32, dy:i32) -> bool {
    let xmas = "XMAS".as_bytes();
    for i in 0..xmas.len() {
        let x = x0 + i as i32*dx;
        let y = y0 + i as i32*dy;
        if y < 0 || x < 0 || y as usize >= map.len() || x as usize >= map[0].len() {
            return false;
        } else if map[y as usize][x as usize] != xmas[i] {
            return false;
        }
    } 
    true
}

fn get_char(map:&Vec<Vec<u8>>, x:i32, y:i32) -> char {
    if y < 0 || x < 0 || y as usize >= map.len() || x as usize >= map[0].len() {
        return '#';
    } else {
        return map[y as usize][x as usize] as char;
    }
}

fn find_mas(map:&Vec<Vec<u8>>, x:i32, y:i32) -> bool {
    if map[y as usize][x as usize] == 'A' as u8 {
        let up_left: char = get_char(map, x-1, y-1);
        let down_left: char = get_char(map, x-1, y+1);
        let up_right: char = get_char(map, x+1, y-1);
        let down_right: char = get_char(map, x+1, y+1);

        if up_left == 'M' && down_right == 'S' || up_left == 'S' && down_right == 'M' {
            if up_right == 'M' && down_left == 'S' || up_right == 'S' && down_left == 'M' {
                return true;
            }
        }
    }
    false
}


fn part1(lines:Vec<&str>) -> String {
    let map = get_map(&lines);
    let deltas = vec![(1,0), (-1,0),(0,1),(0,-1),(1,1),(-1,-1),(-1,1),(1,-1)];
    let mut result = 0;
    
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
        result += deltas.iter()
            .filter(|(dx,dy)| find_xmas(&map, x, y, *dx, *dy))
            .count();
        }
    }

    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let map = get_map(&lines);
    let mut result = 0;

    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            if find_mas(&map, x, y) {
                result += 1;
            }
        }
    }

    result.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::*;


    #[test]
    fn test1() {

        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!("18", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_04.txt");
        assert_eq!("2483", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_04.txt");
        assert_eq!("1925", solve(input.to_string(), Part2));
    }
}
