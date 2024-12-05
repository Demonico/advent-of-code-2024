mod solution;
mod days;

use crate::days::day01;
// use crate::days::day01::puzzle1::Day01Puzzle1;
// use crate::days::day01::puzzle2::Day01Puzzle2;
use solution::Solution;

fn main() {
    println!("Day 1, Puzzle 1: {}", day01::puzzle1::Day01Puzzle1::solve("input/day01.txt"));
    println!("Day 1, Puzzle 2: {}", day01::puzzle2::Day01Puzzle2::solve("input/day01.txt"));

}
