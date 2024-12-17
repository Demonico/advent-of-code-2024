use crate::days::string_lines_to_grid;
use crate::solution::Solution;
use std::collections::HashSet;

pub struct Day06Puzzle1;

impl Solution for Day06Puzzle1 {
    fn solve(input: &str) -> i32 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut grid = string_lines_to_grid(input);
        // find starting point
        // check if the next position is valid
            // if blocked turn
            // else walk
        // if exit stop loop        
        let mut current_position = find_start_point(&grid);
        let mut current_direction = (-1, 0);
        let mut loops = 0;
        loop {
            let next_position = walk(current_position, current_direction);
            if is_valid(next_position, &grid) {
                if is_blocked(next_position, &grid) {
                    current_direction = turn_90(current_direction);
                } else {                    
                    visited.insert(next_position);
                    current_position = next_position;
                }
            } else {
                break
            }
            // println!("loop: {} pos: {:?}", loops, current_position);
            loops += 1;
        }

        visited.len() as i32
    }
}

fn find_start_point(grid: &Vec<Vec<char>>) -> (isize, isize) {
    for (i , row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                return (i as isize, j as isize);
            }
        }
    }
    (0, 0)
}

fn turn_90(cur_dir:(isize, isize)) -> (isize, isize) {
    match cur_dir {
        (-1, 0) => (0, 1),   // up -> right
        (0, 1) => (1, 0),  // right -> down
        (1, 0) => (0, -1), // down -> left
        (0, -1) => (-1, 0), // left -> up
        _ => (0, 0)
    }
}

fn is_blocked(pos: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
    grid[pos.0 as usize][pos.1 as usize] == '#'
}

fn is_valid(pos: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
    pos.0 < grid.len() as isize && pos.1 < grid[0].len() as isize && pos.0 >= 0 && pos.1 >= 0
}

fn walk(pos: (isize, isize), dir: (isize,isize)) -> (isize, isize) {
    (pos.0+dir.0, pos.1+dir.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day06_puzzle1() {
        let count = Day06Puzzle1::solve("test-input/day06.txt");
        assert_eq!(count, 41);
    }
}

