use crate::days::lines_to_vec;
use crate::solution::Solution;

pub struct Day02Puzzle1;

impl Solution for Day02Puzzle1 {
    fn solve(input: &str) -> String {
        let reports: Vec<Vec<i32>> = lines_to_vec(input);
        
        let mut safe_count:i32 = 0;
        

        for report in &reports {
            println!("{:?}", report);
            let mut is_ascending = true;
            let mut is_descending = true;
            let mut is_unsafe = false;
            for i in 1..report.len() {
                let diff = report[i] - report[i-1];
                if diff < 0 {
                    is_ascending = false;
                } else if diff > 0 {
                    is_descending = false;
                }
                if diff.abs() < 1 || diff.abs() > 3 {
                    is_unsafe = true;
                }
            }
            if (is_ascending || is_descending) && !is_unsafe {
                safe_count += 1;
            }
            println!("is safe: {}", !is_unsafe);
        }

        safe_count.to_string()
    }
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