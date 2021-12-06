use std::collections::HashMap;
use std::mem::swap;
use std::ops::RangeInclusive;

use crate::reader::read_lines;

pub fn main() {
    let mut world = make_world(false);

    println!("Day 5-1 result: {}", world.count_danger());
    let mut world = make_world(true);

    println!("Day 5-2 result: {}", world.count_danger());
}

fn make_world(allow_diagonal: bool) -> World {
    let input: Vec<String> = read_lines("input/day5")
        .unwrap()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .collect();
    let input: Vec<&str> = input.iter().map(AsRef::as_ref).collect();
    let lines: Vec<Line> = input
        .iter()
        .filter_map(|l| Line::from_str(l, allow_diagonal).ok())
        .flatten()
        .collect();

    let mut world = World::new();
    for line in lines {
        world.add_line(&line);
    }
    world
}

#[derive(Debug)]
enum Day5Err {
    ParseError,
    IsNotHorizontalOrVertical,
    CantMakeDiagonal,
}

type MyRange = RangeInclusive<u32>;

#[derive(Debug, Eq, PartialEq)]
struct Line {
    pub x: MyRange,
    pub y: MyRange,
}

struct World {
    pub x_lines: HashMap<u32, Vec<MyRange>>,
    pub y_lines: HashMap<u32, Vec<MyRange>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            x_lines: HashMap::new(),
            y_lines: HashMap::new(),
        }
    }

    fn is_horizontal(line: &Line) -> bool {
        line.x.start() < line.x.end()
    }

    pub fn add_line(&mut self, line: &Line) {
        if Self::is_horizontal(line) {
            //horizontal
            let list = self
                .x_lines
                .entry(*line.y.start())
                .or_insert_with(|| Vec::new());
            list.push(line.x.clone());
        } else {
            let list = self
                .y_lines
                .entry(*line.x.start())
                .or_insert_with(|| Vec::new());
            list.push(line.y.clone());
        }
    }

    pub fn count_danger(&mut self) -> u32 {
        let mut map = HashMap::<u32, Vec<u32>>::new();
        const I: usize = 1000;
        for (y, x_ranges) in self.x_lines.clone() {
            let list = map.entry(y).or_insert_with(|| vec![0u32; I]);
            for x_range in x_ranges {
                for x in x_range {
                    list[x as usize] += 1;
                }
            }
        }
        for (x, y_ranges) in self.y_lines.clone() {
            for y_range in y_ranges {
                for y in y_range {
                    let list = map.entry(y).or_insert_with(|| vec![0u32; I]);
                    list[x as usize] += 1;
                }
            }
        }
        map.iter()
            .map(|(_, line)| line.iter().filter(|v| **v >= 2).count() as u32)
            .sum::<u32>()
    }
}

impl Line {
    //"0,9 -> 5,9"
    fn from_str(s: &str, allow_diagonal: bool) -> Result<Vec<Self>, Day5Err> {
        let coords: Vec<&str> = s.split(" ").collect();
        if coords.len() != 3 {
            return Err(Day5Err::ParseError);
        }
        fn parse_chunk(chunk: &str) -> Result<(u32, u32), Day5Err> {
            let chunks: Vec<&str> = chunk.split(",").collect();
            if chunks.len() != 2 {
                return Err(Day5Err::ParseError);
            }
            Ok((
                chunks[0].parse::<u32>().map_err(|_| Day5Err::ParseError)?,
                chunks[1].parse::<u32>().map_err(|_| Day5Err::ParseError)?,
            ))
        }
        let (mut xs, mut ys) = parse_chunk(coords[0])?;
        let (mut xe, mut ye) = parse_chunk(coords[2])?;
        if xs != xe && ys != ye {
            if !allow_diagonal {
                Err(Day5Err::IsNotHorizontalOrVertical)
            } else {
                //For diagonal lines always swap both or none, messes up points otherwise
                if xs > xe {
                    swap(&mut xs, &mut xe);
                    swap(&mut ys, &mut ye);
                }
                let mut y_iter: Box<dyn Iterator<Item = u32> + '_> = if ys > ye {
                    Box::new((ye..=ys).rev())
                } else {
                    Box::new(ys..=ye)
                };
                let mut result = Vec::<Self>::new();
                for x in xs..=xe {
                    let y = y_iter.next().ok_or(Day5Err::CantMakeDiagonal)?;
                    result.push(Self {
                        x: RangeInclusive::new(x, x),
                        y: RangeInclusive::new(y, y),
                    });
                }
                Ok(result)
            }
        } else {
            if xs > xe {
                swap(&mut xs, &mut xe);
            }
            if ys > ye {
                swap(&mut ys, &mut ye);
            }

            Ok(vec![Self {
                x: RangeInclusive::new(xs, xe),
                y: RangeInclusive::new(ys, ye),
            }])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_horizontal() {
        let input = "0,9 -> 5,9";
        let line = Line::from_str(input, false).unwrap();
        assert_eq!((0..=5), line[0].x);
        assert_eq!((9..=9), line[0].y);
    }

    #[test]
    fn parse_line_diagonal_lt_rb() {
        let input = "1,1 -> 3,3";
        let line = Line::from_str(input, true).unwrap();
        assert_eq!(
            vec![
                Line { x: 1..=1, y: 1..=1 },
                Line { x: 2..=2, y: 2..=2 },
                Line { x: 3..=3, y: 3..=3 },
            ],
            line
        );
    }

    #[test]
    fn parse_line_diagonal_lb_rt() {
        let input = "9,7 -> 7,9";
        let line = Line::from_str(input, true).unwrap();
        assert_eq!(
            vec![
                Line { x: 7..=7, y: 9..=9 },
                Line { x: 8..=8, y: 8..=8 },
                Line { x: 9..=9, y: 7..=7 },
            ],
            line
        );
    }

    #[test]
    fn part_one() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let lines: Vec<Line> = input
            .iter()
            .filter_map(|l| Line::from_str(l, false).ok())
            .flatten()
            .collect();

        let mut world = World::new();
        for line in lines {
            world.add_line(&line);
        }

        assert_eq!(5, world.count_danger());
    }

    #[test]
    fn part_two() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let lines: Vec<Line> = input
            .iter()
            .filter_map(|l| Line::from_str(l, true).ok())
            .flatten()
            .collect();

        let mut world = World::new();
        for line in lines {
            world.add_line(&line);
        }

        assert_eq!(12, world.count_danger());
    }
}
