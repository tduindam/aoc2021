use std::str::FromStr;

use crate::reader::read_lines;

pub fn main() {
    calc_one();
    calc_two();
}

fn calc_one() {
    let line_iter = read_lines("input/day2").unwrap();
    let (forward, depth) = line_iter
        .filter_map(|s| s.unwrap().parse::<Instruction>().ok())
        .fold((0i32, 0i32), |sum, val| (sum.0 + val.x, sum.1 + val.y));

    println!("Forward {} Depth {}", forward, depth);
    println!("Solution 1: {}", forward * depth);
}

fn calc_two() {
    let line_iter = read_lines("input/day2").unwrap();
    let (forward, depth, aim) = line_iter
        .filter_map(|s| s.unwrap().parse::<Instruction>().ok())
        .fold((0i32, 0i32, 0i32), |val, i| apply_instruction(val, &i));

    println!("Forward {} Depth {} Aim {}", forward, depth, aim);
    println!("Solution 2: {}", forward * depth);
}

fn apply_instruction((x, y, aim): (i32, i32, i32), instruction: &Instruction) -> (i32, i32, i32) {
    let aim = aim + instruction.y;
    (x + instruction.x, y + aim * instruction.x, aim)
}

enum Day2Err {
    InvalidDirection,
    InvalidIncrement,
    InvalidInput,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = Day2Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(Day2Err::InvalidDirection),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
}

impl FromStr for Instruction {
    type Err = Day2Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<&str> = s.split(" ").collect();
        if chunks.len() != 2 {
            return Err(Day2Err::InvalidInput);
        }
        let direction = Direction::from_str(chunks[0])?;
        let increment = chunks[1]
            .parse::<i32>()
            .map_err(|_| Day2Err::InvalidIncrement)?;

        let (x, y) = match direction {
            Direction::Forward => (increment, 0),
            Direction::Backward => (-increment, 0),
            Direction::Up => (0, -increment),
            Direction::Down => (0, increment),
        };
        Ok(Self { direction, x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        let input = vec![
            Instruction {
                direction: Direction::Forward,
                x: 5,
                y: 0,
            },
            Instruction {
                direction: Direction::Down,
                x: 0,
                y: 5,
            },
            Instruction {
                direction: Direction::Forward,
                x: 8,
                y: 0,
            },
            Instruction {
                direction: Direction::Up,
                x: 0,
                y: -3,
            },
            Instruction {
                direction: Direction::Down,
                x: 0,
                y: 8,
            },
            Instruction {
                direction: Direction::Forward,
                x: 2,
                y: 0,
            },
        ];
        let (forward, depth, aim) = input.iter().fold((0i32, 0i32, 0i32), apply_instruction);
        assert_eq!(forward, 15);
        assert_eq!(depth, 60);
        assert_eq!(forward * depth, 900);
    }
}
