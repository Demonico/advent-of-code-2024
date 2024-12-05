use crate::days::lines_to_vecs;
use crate::solution::Solution;
use std::collections::BinaryHeap;

pub struct Day01Puzzle1;


impl Solution for Day01Puzzle1 {
    fn solve(input: &str) -> String {
        let (vec_a, vec_b) = lines_to_vecs(input);
        let mut heap_a = BinaryHeap::from(vec_a);
        let mut heap_b = BinaryHeap::from(vec_b);

        let mut result = 0;
        while let (Some(a), Some(b)) = (heap_a.pop(), heap_b.pop()) {
            result += a.abs_diff(b)
        }

        result.to_string()
    }
}