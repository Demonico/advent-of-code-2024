use crate::days::read_lines;
use crate::solution::Solution;
use regex::Regex;

pub struct Day03Puzzle1 {}

impl Solution for Day03Puzzle1 {
    fn solve( input: &str) -> String {
        let mut total = 0;
        let mut tuples: Vec<(i32,i32)> = Vec::new(); 

        if let Ok(lines) = read_lines(input) {
            for line in lines {
                if let Ok(s) = line {
                    let muls = parse_muls(&s);
                    tuples.extend(muls);
                }
            }
        }
        for tuple in tuples {
            total += tuple.0 * tuple.1;
        }

        total.to_string()
    }}

fn parse_muls(hay: &str ) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(hay)
        .filter_map(|caps| {
            // Parse the numbers, skipping invalid matches
            let num1 = caps[1].parse::<i32>().ok();
            let num2 = caps[2].parse::<i32>().ok();
            if let (Some(n1), Some(n2)) = (num1, num2) {
                Some((n1, n2))
            } else {
                None
            }
        })
        .collect::<Vec<(i32,i32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_mul() {
        let matches = parse_muls("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");        
        assert_eq!( matches.len(), 4);
    }
    
    #[test]
    fn test_day03_puzzle1() {
        let result = Day03Puzzle1::solve("test-input/day03.txt");
        assert_eq!(result, "161");
    }
}