use crate::days::read_file;
use crate::solution::Solution;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub struct Day05Puzzle2 {
    rules: HashMap<i32, HashSet<i32>>,
    pages: HashMap<i32, Vec<i32>>,
}

impl Solution for Day05Puzzle2 {
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
            result += get_score(&seq,&page_order, false)
        }
        result
    }
}

fn get_score(pages:&Vec<i32>, rules: &HashMap<i32, HashSet<i32>>, is_reordered:bool) -> i32 {
    let mut disallow:HashMap<i32, i32> = HashMap::new();    
    for (idx, page) in pages.iter().enumerate() {
        if disallow.contains_key(page) {
            let j = *disallow.get(page).unwrap() as usize;
            let mut reordered: Vec<i32> = Vec::new();
            reordered.extend(&pages[..j]);
            reordered.push(*page);
            reordered.extend(&pages[j..idx]);
            reordered.extend(&pages[idx+1..]);            
            return get_score(&reordered, rules, true);
        } 
        if let Some(rule) = rules.get(page) {
            for r in rule {
                if !disallow.contains_key(r) {
                    disallow.insert(*r, idx as i32);
                }
            }
        }
    }
    if is_reordered { pages[pages.len() / 2] } else {0}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_puzzle2() {
        let total = Day05Puzzle2::solve("test-input/day05.txt");
        assert_eq!(total, 123);
    }
}