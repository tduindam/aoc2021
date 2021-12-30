use crate::neighbors::GEN_NEIGHBORS_SELF;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::{count, many0, many1};
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl fmt::Debug for Pixel {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            Pixel::On => {"#"}
            Pixel::Off => {"."}
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

struct Image {
    algo: Vec<Pixel>,
    grid: HashSet<Point>,
    outside: Pixel,
    dims: Dims,
}

#[derive(Debug)]
enum Day20Error {
    NotBinaryNumber,
    InvalidGrid,
    InfiniteNumber,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Image {{ ")?;
        write!(f, "outside: {:?},\n ", self.outside)?;
        write!(f, "dims: {:?},\n ", self.dims)?;
        write!(f, "algo: {:?},\n ", self.algo)?;
        write!(f, "grid: \n")?;
        for y in self.dims.min_y..=self.dims.max_y {
            for x in self.dims.min_x..=self.dims.max_x {
                let val = if self.grid.contains(&Point { x, y }) {
                    Pixel::On
                } else {
                    Pixel::Off
                };
                write!(f, "{:?}", val)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Copy, Clone)]
struct Dims {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Dims {
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    fn from(grid: &HashSet<Point>, extra: i64) -> Dims {
        let max_x = grid
            .iter()
            .max_by_key(|p| p.x)
            .ok_or(Day20Error::InvalidGrid)
            .unwrap()
            .x
            + extra;
        let min_y = grid
            .iter()
            .min_by_key(|p| p.y)
            .ok_or(Day20Error::InvalidGrid)
            .unwrap()
            .y
            - extra;
        let min_x = grid
            .iter()
            .min_by_key(|p| p.x)
            .ok_or(Day20Error::InvalidGrid)
            .unwrap()
            .x
            - extra;
        let max_y = grid
            .iter()
            .max_by_key(|p| p.y)
            .ok_or(Day20Error::InvalidGrid)
            .unwrap()
            .y
            + extra;
        Dims {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl Image {
    fn read_pixel(&self, pos: &Point) -> Result<usize, Day20Error> {
        let offset_string: String = GEN_NEIGHBORS_SELF
            .iter()
            .map(|(o_x, o_y)| Point {
                x: pos.x + *o_x as i64,
                y: pos.y + *o_y as i64,
            })
            .map(|p| {
                //Store only ones
                if self.grid.contains(&p) {
                    '1'
                } else {
                    if self.dims.contains(&p) {
                        '0'
                    } else {
                        match self.outside {
                            Pixel::On => '1',
                            Pixel::Off => '0',
                        }
                    }
                }
            })
            .collect();

        let result = usize::from_str_radix(offset_string.as_str(), 2)
            .map_err(|_| Day20Error::NotBinaryNumber)?;
        Ok(result)
    }

    fn process(&self) -> Result<Self, Day20Error> {
        let mut grid = HashSet::<Point>::new();

        for y in self.dims.min_y - 1..=self.dims.max_y + 1 {
            for x in self.dims.min_x - 1..=self.dims.max_x + 1 {
                let p = Point { x, y };
                let index = self.read_pixel(&p)?;
                let val = self.algo[index];
                if val == Pixel::On {
                    grid.insert(p);
                }
            }
        }
        let dims = Dims::from(&grid, 0);
        Ok(Self {
            algo: self.algo.clone(),
            grid,
            outside: if self.outside == Pixel::Off {
                self.algo[0]
            } else {
                self.algo[511]
            },
            dims,
        })
    }

    fn count(&self) -> Result<usize, Day20Error> {
        match self.outside {
            Pixel::On => Err(Day20Error::InfiniteNumber),
            Pixel::Off => Ok(self.grid.len()),
        }
    }
}

fn parse_pixel(input: &str) -> IResult<&str, Pixel> {
    fn parse_on(input: &str) -> IResult<&str, Pixel> {
        let (input, _) = tag("#")(input)?;
        Ok((input, Pixel::On))
    }
    fn parse_off(input: &str) -> IResult<&str, Pixel> {
        let (input, _) = tag(".")(input)?;
        Ok((input, Pixel::Off))
    }
    alt((parse_on, parse_off))(input)
}

fn parse_algo(input: &str) -> IResult<&str, Vec<Pixel>> {
    count(parse_pixel, 512)(input)
}

fn parse_input(input: &str) -> IResult<&str, Image> {
    let (input, algo) = terminated(parse_algo, many0(line_ending))(input)?;
    let (input, grid) = many1(terminated(many1(parse_pixel), opt(line_ending)))(input)?;
    let mut output_grid = HashSet::<Point>::new();
    for y in 0i64..grid.len() as i64 {
        let line = &grid[y as usize];
        output_grid.extend(
            line.iter()
                .positions(|p| *p == Pixel::On)
                .map(|pos| Point { x: pos as i64, y }),
        );
    }
    let dims = Dims::from(&output_grid, 0);
    Ok((
        input,
        Image {
            algo,
            grid: output_grid,
            outside: Pixel::Off,
            dims,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn part_one_small() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (_, input) = parse_input(input).unwrap();
        assert_eq!(Pixel::Off, input.algo[0]);
        assert_eq!(Pixel::On, input.algo[34]);
        assert_eq!(Pixel::On, input.algo[50]);
        let processed = input.process().unwrap();
        let processed = processed.process().unwrap();
        assert_eq!(35, processed.grid.len());
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/day20").unwrap();
        let (_, image) = parse_input(input.as_str()).unwrap();
        let image = image.process().unwrap();
        assert!(image.count().is_err());
        let image = image.process().unwrap();
        assert_eq!(5229, image.count().unwrap());
    }

    #[test]
    fn part_two_small() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (_, mut input) = parse_input(input).unwrap();
        for _ in 0..50 {
            input = input.process().unwrap();
        }
        assert_eq!(3351, input.count().unwrap());
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input/day20").unwrap();
        let (_, mut image) = parse_input(input.as_str()).unwrap();
        for _ in 0..50 {
            image = image.process().unwrap();
        }
        assert_eq!(17009, image.count().unwrap());
    }
}
