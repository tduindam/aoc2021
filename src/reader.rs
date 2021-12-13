use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_ints(line: &str, separator: &str) -> Vec<u32> {
    line.split(separator)
        .filter_map(|v| v.parse::<u32>().ok())
        .collect()
}

pub fn split_list(input: &str) -> Vec<String> {
    input.split("\n").map(|l| l.to_string()).collect()
}

pub fn parse_grid(raw_input: &str) -> ((usize, usize), Vec<u32>) {
    let input: Vec<String> = raw_input.split("\n").map(|l| l.to_string()).collect();
    let col_size = input.len();

    let parsed: Vec<u32> = input
        .iter()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .flatten()
        .collect();

    let row_size = parsed.len() / col_size;
    let col_size = input.len();

    ((row_size, col_size), parsed)
}

pub fn read_lines_filter_ok<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    read_lines(filename)
        .unwrap()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .collect()
}
