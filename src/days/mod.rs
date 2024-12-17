use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub mod day01 {
    pub mod puzzle1;
    pub mod puzzle2;
}

pub mod day02 {
    pub mod puzzle1;
    pub mod puzzle2;
}

pub mod day03 {
    pub mod puzzle1;
    pub mod puzzle2;
}

pub mod day04 {
    pub mod puzzle1;
    pub mod puzzle2;
}

pub mod day05 {
    pub mod puzzle1;
    pub mod puzzle2;
}

pub mod day06 {
    pub mod puzzle1;
}

pub(crate) fn read_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn cols_to_vecs(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vac_a = Vec::new();
    let mut vac_b = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                let numbers: Vec<i32> = content
                    .split_whitespace() // Split the line into parts
                    .filter_map(|n| n.parse::<i32>().ok()) // Parse numbers
                    .collect();

                if numbers.len() == 2 {
                    vac_a.push(numbers[0]);
                    vac_b.push(numbers[1]);
                }
            }
        }
    }

    (vac_a, vac_b)
}

pub(crate) fn lines_to_vec(file_path: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                let numbers: Vec<i32> = content
                    .split_whitespace() // Split the line into parts
                    .filter_map(|n| n.parse::<i32>().ok()) // Parse numbers
                    .collect();
                result.push(numbers);
            }
        }
    }
    result
}

pub(crate) fn string_lines_to_grid(file_path: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                grid.push(line.chars().collect::<Vec<char>>());
            }
        }
    }    
    grid
}