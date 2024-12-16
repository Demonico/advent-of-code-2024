mod solution;
mod days;

use crate::days::{day01, day02, day03, day04, day05};
use solution::Solution;

fn main() {
    // Day 1
    println!("Day 1, Puzzle 1: {}", day01::puzzle1::Day01Puzzle1::solve("input/day01.txt"));
    println!("Day 1, Puzzle 2: {}", day01::puzzle2::Day01Puzzle2::solve("input/day01.txt"));
    // Day 2
    println!("Day 2, Puzzle 1: {}", day02::puzzle1::Day02Puzzle1::solve("input/day02.txt"));
    println!("Day 2, Puzzle 2: {}", day02::puzzle2::Day02Puzzle2::solve("input/day02.txt"));
    // Day 3
    println!("Day 3, Puzzle 1: {}", day03::puzzle1::Day03Puzzle1::solve("input/day03.txt"));
    println!("Day 3, Puzzle 2: {}", day03::puzzle2::Day03Puzzle2::solve("input/day03.txt"));
    // Day 4
    println!("Day 4, Puzzle 1: {}", day04::puzzle1::Day04Puzzle1::solve("input/day04.txt"));
    println!("Day 4, Puzzle 2: {}", day04::puzzle2::Day04Puzzle2::solve("input/day04.txt"));
    // Day 5
    println!("Day 5, Puzzle 1: {}", day05::puzzle1::Day05Puzzle1::solve("input/day05.txt"));
    println!("Day 5, Puzzle 2: {}", day05::puzzle2::Day05Puzzle2::solve("input/day05.txt"));
    
}
