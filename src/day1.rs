use std::io;
use std::path::Path;

use crate::reader::read_lines;

pub fn main() {
    match read_number("input/part1") {
        Ok(numbers) => {
            println!("Counted {}", count_increasing(&numbers));
            println!("Counted {}", count_increasing_window(&numbers, 3));
        }
        Err(err) => {
            eprintln!("Could not read file {:?}", err)
        }
    }
}

fn count_increasing(numbers: &[i32]) -> i32 {
    let mut count = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            count += 1;
        }
    }
    count
}

fn count_increasing_window(numbers: &[i32], window_size: usize) -> i32 {
    let mut count = 0;
    let mut last_window = 0;
    for i in 0..numbers.len() - (window_size - 1) {
        let window = {
            let mut sum = 0;
            for j in 0..window_size {
                sum += numbers[i + j];
            }
            sum
        };
        if last_window != 0 && window > last_window {
            count += 1;
        }
        last_window = window;
    }
    count
}

fn read_number<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let line_iter = read_lines(filename)?;

    Ok(line_iter
        .filter_map(|s| s.unwrap().parse::<i32>().ok())
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sliding_window() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increasing_window(&input, 3), 5);
    }
}
