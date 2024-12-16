use crate::days::read_file;
use crate::solution::Solution;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub struct Day05Puzzle1;

impl Solution for Day05Puzzle1 {
    fn solve(input: &str) -> i32 {
        let mut result:i32 = 0;
        let mut page_order:HashMap<i32,HashSet<i32>> = HashMap::new();
        let mut page_seq:Vec<Vec<i32>> = Vec::new();
        let reader= read_file(input);
        for line in reader.lines() {
            let ln = line.unwrap();
            if ln.contains("|") {
                let order = &ln.split("|").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                page_order.entry(order[1]).or_insert(HashSet::new()).insert(order[0]);
            } else if ln.contains(",") {
                let seq = ln.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                page_seq.push(seq);
            }
        }
        for seq in page_seq {
            result += get_score(&seq,&page_order)
        }
        result
    }
}

fn get_score(pages:&Vec<i32>, preceding: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut disallow:HashSet<i32> = HashSet::new();    
    for page in pages {
        if disallow.contains(page) {
            return 0;
        } else {
            if let Some(page_order) = preceding.get(page) {
                disallow.extend(page_order);
            }
        }
    }
    pages[pages.len() / 2]
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