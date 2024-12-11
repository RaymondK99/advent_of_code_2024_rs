use std::collections::VecDeque;

use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug, Clone, Copy)]
enum Block {
    FileBlock(u64),
    Empty,    
}


fn parse_line(line:&str) -> VecDeque<Block> {
    let numbers:Vec<char> = line.chars().collect();
    let mut queue = VecDeque::new();
    let mut file_id = 0;
    let mut is_file_block = true;
    for ch in numbers {
        let block_size = ch.to_digit(10).unwrap();

        for _ in 0..block_size {
            if is_file_block {
                queue.push_back(Block::FileBlock(file_id));
            } else {
                queue.push_back(Block::Empty);
            }
        }

        if is_file_block {
            file_id += 1;
        }

        is_file_block = !is_file_block;
    }

    queue
}


fn compact(mut content:VecDeque<Block>) -> VecDeque<Block> {
    let mut left = VecDeque::new();
    while !content.is_empty() {
        let item = content.pop_front().unwrap();
        match item {
            Block::FileBlock(file_id) => {
                left.push_back(Block::FileBlock(file_id));
            },
            Block::Empty => {
                // Take the last item
                while !content.is_empty() {
                    let last_item = content.pop_back().unwrap();
                    match last_item {
                        Block::FileBlock(last_file_id) => {
                            // Fill up with last item
                            left.push_back(Block::FileBlock(last_file_id));
                            break;
                        },
                        Block::Empty => {
                            // continue
                            continue;
                        }
                    }
                }  
            },
        }
    }


    left
}


fn calc_checksum(content:&VecDeque<Block>) -> u64 {
    content.iter().enumerate()
        .map(|(i, item)| {
            let value = match item {
                Block::FileBlock(file_id) => *file_id * i as u64,
                Block::Empty => 0,
            };
            return value;
        }).sum::<u64>()
}

fn get_max_file_id(data:&Vec<Block>) -> u64 {
    data.iter().map(|block| match block {
        Block::FileBlock(id) => *id,
        _ => 0,
    })
    .max().unwrap()
}
fn get_free_space(file_id:u64, data:&Vec<Block>) -> Option<(usize, usize)> {
    let block_size = data.iter()
    .filter(|block: &&Block| match block{
        Block::FileBlock(id) => *id == file_id,
        Block::Empty => false,
    }).count();

    let mut free_space = false;
    let mut free_space_index = 0;
    for i in 0..data.len() {
        match data[i] {
            Block::FileBlock(_) => {
                if free_space {
                    // Close free space
                    free_space = false;
                    let free_space_size = i - free_space_index;
                    if free_space_size >= block_size {
                        return Some((free_space_index, free_space_size));
                    }
                
                }
            },
            Block::Empty => {
                if !free_space {
                    // Start free space
                    free_space = true;
                    free_space_index = i;
                }
            },
        }
    }

    None

}

fn compact_v2(file_id:u64, data:&mut Vec<Block>) {

    let file_index = data.iter().enumerate().filter(|(i, block)| match block {
        Block::FileBlock(id) => *id == file_id,
        Block::Empty => false,
    })
    .map(|(i, _)| i)
    .next().unwrap();

    let file_block_size = data.iter()
        .filter(|block: &&Block| match block{
            Block::FileBlock(id) => *id == file_id,
            Block::Empty => false,
        }).count();

    match get_free_space(file_id, &data) {
        Some((free_space_index, free_space_size)) => {
            if free_space_index < file_index {
                for i in file_index..file_index+file_block_size {
                    // Clear 
                    data[i] = Block::Empty;
                }
                for i in free_space_index..free_space_index+file_block_size {
                    data[i] = Block::FileBlock(file_id);
                }
            } else {
                //println!("No free space found for {}", file_id);
            }
        },
        None => {
            //println!("No free space found for {}", file_id);
        },
    }
}


fn part1(lines:Vec<&str>) -> String {
    let line = lines.first().unwrap();
    let content = parse_line(line);
    let file_blocks = compact(content);
    calc_checksum(&file_blocks).to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let line: &&str = lines.first().unwrap();
    let mut data = parse_line(line).iter().copied().collect();
    let mut current_file_id = get_max_file_id(&data);

    loop {

        compact_v2(current_file_id, &mut data);  
        if current_file_id == 0 {
            break;
        } else {
            current_file_id -= 1;
        }
    }

    calc_checksum(&data.iter().copied().collect()).to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "2333133121414131402";
        assert_eq!("1928", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_09.txt");
        assert_eq!("6262891638328", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "2333133121414131402";
        assert_eq!("2858", solve(input.to_string(), Part2));
    }

    //#[test]
    fn test_part2() {
        let input = include_str!("../../input/input_09.txt");
        assert_eq!("6287317016845", solve(input.to_string(), Part2));
    }
}
