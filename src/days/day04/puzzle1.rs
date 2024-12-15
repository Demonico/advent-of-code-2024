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

pub struct Day04Puzzle1;

impl Solution for Day04Puzzle1 {
    fn solve(input: &str) -> i32 {
        let mut total = 0;
        
        let grid = string_lines_to_grid(input);        
        
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c].to_string() == "X" {
                    total += count(&grid,r,c);
                }
            }
        }
        
        total
    }
}

fn count(grid: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let mut print_me = Vec::new();
    let mut count = 0;
    let word = "XMAS";
    let rev_word = "SAMX";
    let left = collect_chars(grid,r,c,Direction::Left,4);
    let right = collect_chars(grid,r,c,Direction::Right,4);
    let up = collect_chars(grid,r,c,Direction::Up,4);
    let down = collect_chars(grid,r,c,Direction::Down,4);
    let up_left = collect_chars(grid,r,c,Direction::UpLeft,4);
    let up_right = collect_chars(grid,r,c,Direction::UpRight,4);
    let down_left = collect_chars(grid,r,c,Direction::DownLeft,4);
    let down_right = collect_chars(grid,r,c,Direction::DownRight,4);
    
    if left == word || left == rev_word {
        print_me.push("left");
        count += 1
    }
    if right == word || right == rev_word {
        print_me.push("right");
        count += 1
    }
    if up == word || up == rev_word {
        print_me.push("up");
        count += 1
    }
    if down == word || down == rev_word {
        print_me.push("down");
        count += 1
    }
    if up_left == word || up_left == rev_word {
        print_me.push("up_left");
        count += 1
    }
    if up_right == word || up_right == rev_word {
        print_me.push("up_right");
        count += 1
    }
    if down_left == word || down_left == rev_word {
        print_me.push("down_left");
        count += 1
    }
    if down_right == word || down_right == rev_word {
        print_me.push("down_right");
        count += 1
    }    
    // println!("r {} c {} {:?}",r,c,print_me);
    count
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn collect_chars(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    direction: Direction,
    length: usize,
) -> String {
    let mut result = String::new();
    let mut row = start_row as isize; // Use isize for bounds checking with negative directions
    let mut col = start_col as isize;

    for _ in 0..length {
        // Check if the indices are within bounds
        if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
            break;
        }

        // Collect the character
        result.push(grid[row as usize][col as usize]);

        // Move in the specified direction
        match direction {
            Direction::Up => row -= 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
            Direction::UpLeft => {
                row -= 1;
                col -= 1;
            }
            Direction::UpRight => {
                row -= 1;
                col += 1;
            }
            Direction::DownLeft => {
                row += 1;
                col -= 1;
            }
            Direction::DownRight => {
                row += 1;
                col += 1;
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_puzzle1() {
        let count = Day04Puzzle1::solve("test-input/day04.txt");
        assert_eq!(count, 18);
    }
}
