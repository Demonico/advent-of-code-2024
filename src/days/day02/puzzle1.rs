use crate::days::lines_to_vec;
use crate::solution::Solution;

pub struct Day02Puzzle1;

impl Solution for Day02Puzzle1 {
    fn solve(input: &str) -> String {
        let reports: Vec<Vec<i32>> = lines_to_vec(input);

        let mut safe_count:i32 = 0;
        let increasing: Vec<i32> = (1..4).collect();
        let decreasing: Vec<i32> = (-3..0).collect();

        for report in &reports {
            safe_count += is_safe(report.clone(),increasing.clone()) 
                + is_safe(report.clone(),decreasing.clone());
        }

        safe_count.to_string()
    }
}

fn is_safe(nums: Vec<i32>,safe_range:Vec<i32>) -> i32 {    
    for i in 1..nums.len() {
        if !safe_range.contains(&(nums[i] - nums[i - 1])) {
            return 0
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_puzzle1() {
        let count: i32 = Day02Puzzle1::solve("test-input/day02.txt").parse().unwrap();
        assert_eq!(count, 3);
    }
}