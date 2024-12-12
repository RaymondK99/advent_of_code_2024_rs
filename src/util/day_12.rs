use std::{collections::{HashMap, VecDeque}, vec};

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x:i32,
    y:i32,
}

impl Position {
    fn create(x_in:usize, y_in:usize) -> Position {
        Position { x: x_in as i32 , y: y_in as i32 }
    }

    fn up(&self) -> Position {
        Position{x:self.x, y:self.y-1}
    }

    fn down(&self) -> Position {
        Position{x:self.x, y:self.y+1}
    }

    fn left(&self) -> Position {
        Position{x:self.x-1, y:self.y}
    }

    fn right(&self) -> Position {
        Position{x:self.x+1, y:self.y}
    }
}


#[derive(Debug)]
struct Region {
    region_type:char,
    positions:Vec<Position>,
}


impl Region {

    fn print(&self) {
        println!("Region[type:{}, pos:{:?}]", self.region_type, self.positions);
    }

    fn contains_position(&self, pos:&Position) -> bool {
        self.positions.contains(pos)
    }

    fn calc_perimeter(&self) -> u32 {
        let mut sum = 0;
        for pos in self.positions.iter() {
            sum += self.calc_perimiter_for_pos(pos);
        }

        sum

    }
    fn calc_perimiter_for_pos(&self, pos:&Position) -> u32 {
        let mut perimiter = 0;
        let adjacent_pos = vec![pos.left(), pos.right(), pos.up(), pos.down()];
        for pos in adjacent_pos {
            if !self.contains_position(&pos) {
                perimiter += 1;
            }
        }
        perimiter
    }

    fn calc_area(&self) -> u32 {
        self.positions.len() as u32
    }
    
}

struct Map {
    width:i32,
    height:i32,
    grid:HashMap<Position, char>,
    regions:Vec<Region>,
}

impl Map {
    fn create(lines:Vec<&str>) -> Map {
        let mut grid = HashMap::new();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        for y in 0..lines.len() {
            let chars:Vec<char> = lines[y].chars().collect();
            for x in 0 ..chars.len() {
                let ch = chars[x];
                grid.insert(Position::create(x, y), ch);
            }
        }
        Map{width, height, grid:grid, regions:vec![]}
    }

    fn explore(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = Position{x,y};
                // If it does not belong to a region
                if !self.is_in_reqion(&pos) {
                    let region_type = *self.grid.get(&pos).unwrap();
                    // create new region
                    let new_region = self.explore_region(region_type, pos);
                    self.regions.push(new_region);
                } else {
                    // pos is already inside a region
                }
            }
        }
    }
    
    fn explore_region(&self, region_type:char, pos:Position) -> Region {
        let mut queue:VecDeque<Position> = VecDeque::new();
        let mut positions = vec![];
        queue.push_back(pos);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if positions.contains(&current) {
                continue;
            }

            match self.grid.get(&current) {
                Some(next_region_type) => {
                    if *next_region_type == region_type {

                        // Add neighbors
                        queue.push_back(current.down());
                        queue.push_back(current.up());
                        queue.push_back(current.left());
                        queue.push_back(current.right());

                        // Add current to region
                        positions.push(current);
                    }
                },
                None => {
                    // outside grid
                },
            }  
        }

        // Allocate region
        Region{ region_type, positions }

    }

    fn is_in_reqion(&self, pos:&Position) -> bool {
        for region in self.regions.iter() {
            if region.contains_position(pos) {
                return true;
            }
        }
        return false;
    }
}

fn part1(lines:Vec<&str>) -> String {
    let mut map = Map::create(lines);
    let mut sum = 0;
    map.explore();

    for region in map.regions.iter() {
        sum += region.calc_perimeter() * region.calc_area();
    }

    sum.to_string()
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

        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_12.txt");
        assert_eq!("1473408", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";
        assert_eq!("2", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_12.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
