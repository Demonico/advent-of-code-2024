mod solution;
mod days;

use crate::days::day01::puzzle1::Day01Puzzle1;
use solution::Solution;

fn main() {
    println!("Day 1, Puzzle 1: {}", Day01Puzzle1::solve("input/day01.txt"));

}
