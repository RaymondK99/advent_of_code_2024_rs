
use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn get_numeric_pair(line:&str) -> (u64,u64) {
    let mut it = line.split(&['=','+', ' ', ':', ','])
        .into_iter()
        .filter(|item| !item.is_empty())
        .filter(|item| item.chars().all(|c| c.is_digit(10)))
        .map(|item| {
            return item.parse::<u64>().unwrap();
        });


    let left = it.next().unwrap();
    let right = it.next().unwrap();
    (left, right)
}
fn parse(lines:&[&str]) -> ((u64,u64),(u64,u64),(u64,u64)) {
    let button_a = get_numeric_pair(lines[0]);
    let button_b = get_numeric_pair(lines[1]);
    let target = get_numeric_pair(lines[2]);
    (button_a, button_b, target)
}


fn find_target_2(button_a:(u64,u64), button_b:(u64,u64), target:(u64,u64)) -> Option<(u64,u64)>{
    let (x_a, y_a) = button_a;
    let (x_b, y_b) = button_b;
    let (x_t, y_t) = target;


    println!("{:?},{:?},{:?}", button_a, button_b, target);
    let mut i  = 1;
    let mut step_size = 1;
    let mut last_mult = 0;

    loop {

        let step_x = x_a * i;
        let step_y = y_a * i;

        if step_x > x_t || step_y > y_t {
            break;
        }

        let dx = x_t - x_a * i;
        let dy = y_t - y_a * i;

        if dx % x_b == 0  && dy % y_b == 0 {

            if last_mult == 0 {
                last_mult = i;
            } else if step_size == 1 {
                step_size = i-last_mult;
            }

            println!("mult a:{}, mult b1:{}, mult b2:{}", i, dx/x_b, dy/y_b);

            if dx / x_b == dy / y_b {
                println!("mult a:{}, mult:{}", i, dx/x_b);
                return Some((i, dx/x_b));
            } 
        }

        i += step_size;
    }

    None
}



fn find_target_3(button_a:(u64,u64), button_b:(u64,u64), target:(u64,u64)) {
    let (x_a, y_a) = button_a;
    let (x_b, y_b) = button_b;
    let (x_t, y_t) = target;



}

fn part1(lines:Vec<&str>) -> String {
    let chunks = lines.chunks(4);
    let mut result = 0;
    for chunk in chunks {
        let (button_a, button_b, target) = parse(chunk);
        match find_target_2(button_a, button_b, target) {
            Some((a,b)) => {
                let tokens = a*3 + b;
                println!("tokens={}", tokens);
                result += tokens;
            },
            None => {
                //panic!("....");
            },
        };
    }

    
    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let chunks = lines.chunks(4);
    let mut result = 0;
    for chunk in chunks {
        let (button_a, button_b, target) = parse(chunk);
        let new_target = (target.0 + 10000000000000, target.1 + 10000000000000);
        match find_target_2(button_a, button_b, new_target) {
            Some((a,b)) => {
                let tokens = a*3 + b;
                println!("tokens={}", tokens);
                result += tokens;
            },
            None => {},
        };
    }

    
    result.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_13.txt");
        assert_eq!("29711", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400
        
        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176
        
        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450
        
        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

        assert_eq!("0", solve(input.to_string(), Part2));
    }

    //#[test]
    fn test_part2() {
        // too low => 82525073961064
        let input = include_str!("../../input/input_13.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
