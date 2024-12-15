use crate::days::read_lines;
use crate::solution::Solution;
use regex::Regex;

pub struct Day03Puzzle2 {}

impl Solution for Day03Puzzle2 {
    fn solve( input: &str) -> i32 {
        let mut total = 0;
        let mut matches: Vec<Matches> = Vec::new(); 

        if let Ok(lines) = read_lines(input) {
            for line in lines {
                if let Ok(s) = line {
                    let matched = parse_muls(&s);
                    matches.extend(matched);
                }
            }
        }
        let mut calc = true;
        for item in matches {
            // println!("item {:?}", item);
            match item {
                Matches::Nums(n1,n2) => if calc { total += n1 * n2 }  else { total += 0 }
                Matches::Word(word) => if word == "don't" {calc = false} else {calc = true}                    
            }
            // println!("total {}", total);
        }

        total
    }}

#[derive(Debug)]
enum Matches {
    Nums(i32,i32),
    Word(String),
}

fn parse_muls(hay: &str ) -> Vec<Matches> {
    let re = Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)|(?P<word>don't|do)").unwrap();
    let mut matches: Vec<Matches> = Vec::new();
    
    for caps in re.captures_iter(hay) {
        if let (Some(num1), Some(num2)) = (caps.name("num1"), caps.name("num2")) {
            // Destructure the named groups for tuples
            let num1: i32 = num1.as_str().parse().unwrap();
            let num2: i32 = num2.as_str().parse().unwrap();
            matches.push(Matches::Nums(num1, num2));
        } else if let Some(word) = caps.name("word") {
            // Capture the named word group
            matches.push(Matches::Word(word.as_str().to_string()));
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_mul() {
        let matches = parse_muls("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");        
        assert_eq!( matches.len(), 6);
    }
    
    #[test]
    fn test_day03_puzzle1() {
        let result = Day03Puzzle2::solve("test-input/day03.txt");
        assert_eq!(result, 48);
    }
}