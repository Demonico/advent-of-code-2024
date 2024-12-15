use crate::days::read_file;
use crate::solution::Solution;
use std::collections::HashMap;
use std::io::BufRead;

pub struct Day05Puzzle1;

impl Solution for Day05Puzzle1 {
    fn solve(input: &str) -> i32 {
        let mut result:i32 = 0;
        let mut page_order:HashMap<i32,Vec<i32>> = HashMap::new();
        let mut page_seq:Vec<Vec<i32>> = Vec::new();
        let reader= read_file(input);
        for line in reader.lines() {
            let ln = line.unwrap();
            if ln.contains("|") {
                let order = parse_order(&ln);
                page_order.entry(order[1]).or_insert(Vec::new()).push(order[0]);
            } else if ln.contains(",") {
                let seq = parse_page_seq(&ln);
                page_seq.push(seq);
            }
        }
        for seq in page_seq {
            let mut disallow = Vec::new();
            let mut valid = true;
            for page in &seq {
                if disallow.contains(page) {
                    valid = false;
                    break;
                } else {
                    if let Some(pages) = page_order.get(&page) {
                        disallow.extend(pages);
                    }
                }
            }
            if valid {
                result += seq[seq.len() / 2]
            }
        }
        result
    }
}

fn parse_order(order: &str) -> Vec<i32> {
    order.split("|").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn parse_page_seq(seq: &str) -> Vec<i32> {
    seq.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_puzzle1() {
        let total = Day05Puzzle1::solve("test-input/day05.txt");
        assert_eq!(total, 143);
    }
}