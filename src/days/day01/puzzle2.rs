use crate::days::lines_to_vecs;
use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day01Puzzle2;

impl Solution for Day01Puzzle2 {
    fn solve(input: &str) -> String {
        let mut result = 0;
        let (vec_a, vec_b) = lines_to_vecs(input);
        let mut map_a: HashMap<_,_> = vec_a.into_iter().map(|x| (x,0)).collect();

        for item in vec_b {
            map_a.entry(item).and_modify(|v| *v += 1);
        }
        for (key, val) in &map_a {
            if *val != 0 {
                result += key * val;
            }
        }

        result.to_string()
    }
}