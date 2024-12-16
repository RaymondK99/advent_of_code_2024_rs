use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(line:&str) -> ((i32,i32),(i32,i32)){
    let columns:Vec<&str> = line.split(&[' ',',','=']).collect();

    let x:i32 = columns[1].parse().unwrap();
    let y:i32 = columns[2].parse().unwrap();
    let dx:i32 = columns[4].parse().unwrap();
    let dy:i32 = columns[5].parse().unwrap();

    ((x,y),(dx,dy))
}

fn get_quadrant(x:i32,y:i32,width:i32,height:i32) -> Option<usize> {
    let quadrant_width = width / 2;
    let quadrant_height = height / 2;

    if x == quadrant_width || y == quadrant_height {
        return None;
    }

    let left = x < quadrant_width;
    let up = y < quadrant_height;
    let mut quad_no = 0;
    if !left {
        quad_no += 1;
    }
    if !up {
        quad_no += 2;
    }
    Some(quad_no)
}

fn part1(lines:Vec<&str>) -> String {
    let robots:Vec<((i32,i32),(i32,i32))> = lines.iter().map(|line| parse(line)).collect();
    let height: i32 = *robots.iter().map(|((_,y),(_,_))|y).max().unwrap()+1;
    let width: i32 = *robots.iter().map(|((x,_),(_,_))|x).max().unwrap()+1;
    let seconds = 100;
    let mut quad_count = vec![0,0,0,0];
    for robot in robots {
        let ((x,y), (dx,dy)) = robot;
        let mut last_x = x; 
        let mut last_y = y; 

        for _ in 0..seconds {
            last_x += dx;
            last_y += dy;
            if last_x < 0 {
                last_x = width + last_x;
            } else {
                last_x = last_x % width;
            }

            if last_y < 0 {
                last_y = height + last_y;
            } else {
                last_y = last_y % height;
            }
        }

        match get_quadrant(last_x, last_y, width, height) {
            Some(quad_no) => {
                quad_count[quad_no] += 1;
            },
            None => {},
        }
    }

    quad_count.iter().product::<usize>().to_string()
}


fn get_area(robots:&Vec<((i32,i32),(i32,i32))>) -> i32{
    let max_x: i32 = *robots.iter().map(|((x,_),(_,_))|x).max().unwrap();
    let max_y: i32 = *robots.iter().map(|((_,y),(_,_))|y).max().unwrap();
    let min_x: i32 = *robots.iter().map(|((x,_),(_,_))|x).min().unwrap();
    let min_y: i32 = *robots.iter().map(|((_,y),(_,_))|y).min().unwrap();

    (max_x - min_x) * (max_y - min_y)
}

fn part2(lines:Vec<&str>) -> String {
    let mut robots:Vec<((i32,i32),(i32,i32))> = lines.iter().map(|line| parse(line)).collect();
    let height: i32 = *robots.iter().map(|((_,y),(_,_))|y).max().unwrap()+1;
    let width: i32 = *robots.iter().map(|((x,_),(_,_))|x).max().unwrap()+1;
    let seconds = 100_000;
    //let mut time_map = vec![];

    let mut min_area = i32::MAX;

    for _ in 0..seconds {
        for i in 0..robots.len() {
            let ((x,y), (dx,dy)) = robots[i];
        
            let mut next_x = x + dx;
            let mut next_y = y + dy;
            
            if next_x < 0 {
                next_x = width + next_x;
            } else {
                next_x = next_x % width;
            }

            if next_y < 0 {
                next_y = height + next_y;
            } else {
                next_y = next_y % height;
            }

            robots[i] = ((next_x, next_y),(dx,dy));
        }

        let mut map:Vec<i32> = Vec::new();
        for _ in 0..width*height {
            map.push(0);
        }

        for robot in robots.iter() {
            let ((x,y),(_,_)) = *robot;
            let index: i32 = y*width + x;
            map[index as usize] += 1;
        }

        //let area = get_area(&robots);
        if true {

            //println!("new min area = {}", area);
            //min_area = area;
            
                    // Print
            for y in 0..height {
                for x in 0..width {
                    let index: i32 = y*width + x;
                    let value = map[index as usize];
                    if value == 0 {
                        print!(" ");
                    } else {
                        print!("{}", value);
                    }
                }
                println!();
            }
            println!();
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

        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!("12", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_14.txt");
        assert_eq!("215476074", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_14.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
