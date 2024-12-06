use crate::days::cols_to_vecs;
use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day01Puzzle2;

impl Solution for Day01Puzzle2 {
    fn solve(input: &str) -> String {        
        let (vec_a, vec_b) = cols_to_vecs(input);
        let mut counts = HashMap::new();
        for &item in vec_b.iter() {
            *counts.entry(item).or_insert(0) += 1;
        }
        vec_a
            .iter()
            .map(|item| counts.get(item).unwrap_or(&0) * item)
            .sum::<i32>()
            .to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let similarity: i32 = Day01Puzzle2::solve("test-input/day01.txt").parse().expect("Failed to parse distance");
        assert_eq!(similarity, 31);
    }

}