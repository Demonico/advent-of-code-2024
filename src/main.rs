mod solution;
mod days;

use crate::days::{day01, day02, day03};
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
}
