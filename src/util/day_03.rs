
use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug, Clone, Copy)]
enum MatchExpr<'a> {
    Expr(&'a str),
    Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchResult<'a> {
    MatchedNumber(i32, usize),
    MatchedString(&'a str, usize),
    NoMatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Mult(i32, i32, usize),
    Do(usize),
    DoNot(usize),
    NotFound,
}


fn parse_number(data:&[u8], offset:usize) -> MatchResult {
    let mut len = 0;
    let mut value = 0;
    while offset < data.len() {
        let ch = data[offset+len] as char;
        if ch.is_numeric() {
            len += 1;
            let digit = ch as i32 - 0x30;
            value = value * 10 + digit;
        } else {
            break;
        }
    }
    
    if len == 0 {
        MatchResult::NoMatch
    } else {
       MatchResult::MatchedNumber(value, len)
    }
}

fn match_string<'a>(expr:&'a str, data:&'a [u8], offset:usize) -> MatchResult<'a> {
    let mut i = 0;
    while offset < data.len() && i < expr.len() {
        if expr.as_bytes()[i] != data[offset+i] {
            return MatchResult::NoMatch;
        }
        i += 1;
    }
    
    if i == expr.len() {
        MatchResult::MatchedString(expr, i)
    } else {
        MatchResult::NoMatch
    }
}


fn handle_match<'a>(expr: MatchExpr<'a>, data:&'a [u8], offset:usize) -> MatchResult<'a> {
    match expr {
        MatchExpr::Expr(s_expr) => match_string(s_expr, data, offset),
        MatchExpr::Number => parse_number(data, offset),
    }
}

fn match_phrase(phrase:&Vec<MatchExpr>, data:&[u8], offset:usize) -> Operation {
    let mut i = offset;
    let mut numbers = vec![];
    for expr in phrase.iter() {
        let result = handle_match(*expr, data, i);
        match result {
            MatchResult::MatchedNumber(val, len) => {
                numbers.push(val);
                i += len;
            },
            MatchResult::MatchedString(s, len) => {
                // OK
                i += len;
                if s.eq("do()") && phrase.len() == 1 {
                    return Operation::Do(i - offset);
                } else if s.eq("don't()") && phrase.len() == 1 {
                    return Operation::DoNot(i - offset);
                }
            },
            MatchResult::NoMatch => {
                return Operation::NotFound;
            },
        }
    }
    Operation::Mult(numbers[0],numbers[1] , i - offset)
}


fn parse(data:&[u8], part2:bool) -> i32 {
    let mut offset = 0;
    let mut result = 0;
    let phrase_mult = vec![MatchExpr::Expr("mul("), 
        MatchExpr::Number, 
        MatchExpr::Expr(","), 
        MatchExpr::Number, 
        MatchExpr::Expr(")")];
    let phrase_do = vec![MatchExpr::Expr("do()")];
    let phrase_dont = vec![MatchExpr::Expr("don't()")];
    let mut enabled = true;

    'outer:
    while offset < data.len() {
        println!("ch={}, offset={},", data[offset] as char, offset);
        let list = [&phrase_mult, &phrase_do, &phrase_dont];
        for phrase in list {
            let operation: Operation = match_phrase(&phrase, data, offset);
            match operation {
                Operation::Mult(left, right, len) => {
                    if enabled || !part2 {
                        println!("mult({},{}), len={}", left, right, len);
                        result += left * right;
                    } else {
                        println!("----> DISABLED: mult({},{}), len={}", left, right, len);

                    }
                    offset += len;
                    continue 'outer;
                },
                Operation::NotFound => {
                    // Do nothing
                },
                Operation::Do(len) => {
                    println!("Do, len={}", len);
                    offset += len;
                    enabled = true;
                    continue 'outer;
                },
                Operation::DoNot(len) => {
                    println!("DoNot, len={}", len);
                    offset += len;
                    enabled = false;
                    continue 'outer;
                },
            }
        }

        offset += 1;

    }
    result

}


fn part1(lines:Vec<&str>) -> String {
    let mut result = 0;
    for line in lines {
        result += parse(line.as_bytes(), false);
    }
    println!("{}", result);
    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let mut result = 0;
    for line in lines {
        result += parse(line.as_bytes(), true);
    }
    println!("{}", result);
    result.to_string()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::super::Part::*;

    #[test]
    fn test1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_03.txt");
        assert_eq!("196826776", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        // to high => 114961848
        let input = include_str!("../../input/input_03.txt");
        assert_eq!("114961848", solve(input.to_string(), Part2));
    }
}
