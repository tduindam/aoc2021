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
