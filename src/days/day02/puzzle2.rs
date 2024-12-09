use crate::days::lines_to_vec;
use crate::solution::Solution;

pub struct Day02Puzzle2;

impl Solution for Day02Puzzle2 {
    fn solve(input: &str) -> String {
        let reports: Vec<Vec<i32>> = lines_to_vec(input);

        let mut safe_count: i32 = 0;
        let increasing: Vec<i32> = (1..4).collect();
        let decreasing: Vec<i32> = (-3..0).collect();

        for report in &reports {
            let reversed_report:Vec<i32> = report.iter().rev().copied().collect::<Vec<i32>>();
            let safe = [
                is_safe(&report, increasing.clone(), true),
                is_safe(&report, decreasing.clone(), true),
                is_safe(&reversed_report, increasing.clone(), true),
                is_safe(&reversed_report, decreasing.clone(), true)
            ];
            // safe_count += if safe.iter().any(|&i| i == true) { 1 } else { 0 };
            if safe.iter().any(|x| *x == true) {                
                safe_count += 1;
            } else {
                println!("Report {:?}", report);
            }
        }

        safe_count.to_string()
    }
}

fn is_safe(nums: &[i32], safe_range: Vec<i32>, mut allow_skip: bool) -> bool {
    let mut prev:i32 = nums[0];    
    for curr in nums.iter().skip(1) {
        if safe_range.contains(&(curr - prev)) {
            prev = *curr;
        } else if !allow_skip {            
            return false;
        } else {
            allow_skip = false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_puzzle2() {
        let count: i32 = Day02Puzzle2::solve("test-input/day02.txt").parse().unwrap();
        assert_eq!(count, 8);
    }
    
    #[test]
    fn test_day02_puzzle2_edges() {
        let count: i32 = Day02Puzzle2::solve("test-input/day02edgecases.txt").parse().unwrap();
        assert_eq!(count, 10);
    }
}
