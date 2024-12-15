/*
  Create Grid
  Traverse grid cells
    if val is x or s trigger search
        identify boundaries via directions
        each search should repeat in same direction
            and terminate if val is not correct cha
        if correct string found
            save values in set of tuples
    return set lengthr

    (1,1) can only move right, down-right, and down
*/
use crate::days::string_lines_to_grid;
use crate::solution::Solution;

pub struct Day04Puzzle2;

impl Solution for Day04Puzzle2 {
    fn solve(input: &str) -> i32 {
        let mut total = 0;
        
        let grid = string_lines_to_grid(input);        
        
        for r in 1..(grid.len()-1) {
            for c in 1..(grid[0].len()-1) {
                if grid[r][c].to_string() == "A" {
                    total += count(&grid,r,c);
                }
            }
        }
        
        total
    }
}

fn count(grid: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {    
    let word = "MAS";
    let rev_word = "SAM";
    let up_left = grid[r-1][c-1].to_string();
    let up_right = grid[r-1][c+1].to_string();
    let down_left = grid[r+1][c-1].to_string();
    let down_right = grid[r+1][c+1].to_string();
    let pos_diag = down_left + "A" + &up_right;
    let neg_diag = up_left + "A" + &down_right;
    
    if (pos_diag == word || pos_diag == rev_word) && (neg_diag == word || neg_diag == rev_word) {1} else {0}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_puzzle2() {
        let count = Day04Puzzle2::solve("test-input/day04.txt");
        assert_eq!(count, 9);
    }
}
