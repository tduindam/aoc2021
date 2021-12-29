use crate::day19::Direction::*;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::character::streaming::space1;
use nom::combinator::opt;
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, terminated, tuple};
use nom::IResult;
use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/day19").unwrap();
    let scanners = parse_primary(input.to_string());
    let solution = Solution::solve(scanners, 12);
    println!("Day 19 - 1 {}", solution.beacons.len());
    println!("Day 19 - 2 {}", solution.max_distance());
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

type Orientation = [Direction; 3];

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    #[allow(dead_code)]
    fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn translate(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn rotate(&self, ori: &Orientation) -> Self {
        fn single_transform(p: &Point, direction: Direction) -> i64 {
            match direction {
                PosX => p.x,
                NegX => -p.x,
                PosY => p.y,
                NegY => -p.y,
                PosZ => p.z,
                NegZ => -p.z,
            }
        }

        Self {
            x: single_transform(&self, ori[0]),
            y: single_transform(&self, ori[1]),
            z: single_transform(&self, ori[2]),
        }
    }

    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Solution {
    scanners: Vec<Scanner>,
    beacons: HashSet<Point>,
}

#[derive(Debug, Clone)]
struct Scanner {
    #[allow(dead_code)]
    name: String,
    beacons: Vec<Point>,
    position: Option<Point>,
}

impl Scanner {
    //Insert orientations here
    fn delta(&self, index: usize, ori: &Orientation) -> (Point, Vec<Point>) {
        let mut deltas = Vec::<Point>::new();
        let reference = self.beacons[index].rotate(&ori);
        for i in 0..self.beacons.len() {
            if i == index {
                continue;
            }
            let rotated = self.beacons[i].rotate(ori);
            let delta = rotated.translate(&reference);
            deltas.push(delta);
        }
        deltas.sort();
        (reference, deltas)
    }

    //Returns points transformed if there is overlap >= threshold
    fn overlapping_beacons(
        &self,
        other: &Scanner,
        overlap_threshold: usize,
    ) -> Option<(Point, Vec<Point>)> {
        let orientations = make_orientations();

        println!(
            "Computing overlap between {:?} and {:?}",
            self.name, other.name
        );
        let overlap = orientations
            .iter()
            .filter_map(|ori| {
                (0..self.beacons.len())
                    .filter_map(|ref_index| {
                        let (reference_self, deltas) = self.delta(ref_index, &orientations[0]);
                        let deltas_self: HashSet<Point> = deltas.into_iter().collect();
                        let overlaps = (0..other.beacons.len())
                            .filter_map(|reference_index| {
                                let mut overlap = deltas_self.clone();
                                let (other_reference, other_deltas) =
                                    other.delta(reference_index, ori);
                                overlap.extend(other_deltas.iter());
                                //Check if there are at least `overlap_threshold` new points
                                let not_overlapping = overlap.len() - deltas_self.len();
                                if other.beacons.len() - not_overlapping >= overlap_threshold {
                                    let translation = other_reference.translate(&reference_self);
                                    Some((other_reference, translation))
                                } else {
                                    None
                                }
                            })
                            .next();

                        match overlaps {
                            Some((reference, translation)) => Some((ori, reference, translation)),
                            None => None,
                        }
                    })
                    .next()
            })
            .next();
        let (ori, _other_reference, translation) = if let Some(overlap) = overlap {
            overlap
        } else {
            return None;
        };

        //other reference is already translated into space of the first scanner
        // println!("Overlaps {:?} {:?}", other_reference, ori);
        let translated_points: Vec<Point> = other
            .beacons
            .iter()
            .map(|b| {
                let b = b.rotate(ori);
                b.translate(&translation)
            })
            .collect();
        // println!("Points {:?}", translated_points);
        Some((translation, translated_points))
    }

    pub fn new(name: String, beacons: Vec<Point>) -> Self {
        Scanner {
            name,
            beacons,
            position: None,
        }
    }
}

impl Solution {
    fn max_distance(&self) -> u64 {
        self.scanners
            .iter()
            .filter_map(|s| s.position)
            .combinations(2)
            .map(|points| points[0].manhattan_dist(&points[1]))
            .max()
            .unwrap() as u64
    }

    fn solve(mut scanners: Vec<Scanner>, overlap_threshold: usize) -> Self {
        //Weird pop front
        scanners.reverse();
        let mut first = scanners.pop().unwrap();
        scanners.reverse();
        let beacons: HashSet<Point> = first.beacons.clone().into_iter().collect();
        first.position = Some(Point::new(0, 0, 0));
        let mut solution = Self {
            scanners: vec![first],
            beacons,
        };

        while !scanners.is_empty() {
            let overlap = {
                scanners
                    .iter()
                    .filter_map(|scanner_b| {
                        let overlap = solution
                            .scanners
                            .iter()
                            .filter_map(|scanner_a| {
                                scanner_a.overlapping_beacons(scanner_b, overlap_threshold)
                            })
                            .next();
                        match overlap {
                            None => None,
                            Some(overlap) => Some((overlap, scanner_b)),
                        }
                    })
                    .next()
            };

            //must have overlap
            let ((translation, overlap), scanner_b) = overlap.unwrap();
            solution.beacons.extend(overlap.iter());
            //Overlapping points are in 'scanner_a' translation, just add the relative translation of that scanner
            let solved_scanner_b = Scanner {
                name: scanner_b.name.clone(),
                beacons: overlap,
                position: Some(translation),
            };
            scanners.retain(|s| s.name != solved_scanner_b.name);
            solution.scanners.push(solved_scanner_b);
            println!("Scanners {:?}", scanners);
        }

        solution
    }
}

//Orientations, first is index second is sign
fn make_orientations() -> Vec<Orientation> {
    vec![
        [PosX, PosY, PosZ],
        [PosY, NegX, PosZ],
        [NegX, NegY, PosZ],
        [NegY, PosX, PosZ],
        [PosX, NegY, NegZ],
        [NegY, NegX, NegZ],
        [NegX, PosY, NegZ],
        [PosY, PosX, NegZ],
        [NegZ, PosY, PosX],
        [PosY, PosZ, PosX],
        [PosZ, NegY, PosX],
        [NegY, NegZ, PosX],
        [PosZ, PosY, NegX],
        [PosY, NegZ, NegX],
        [NegZ, NegY, NegX],
        [NegY, PosZ, NegX],
        [PosX, NegZ, PosY],
        [NegZ, NegX, PosY],
        [NegX, PosZ, PosY],
        [PosZ, PosX, PosY],
        [PosX, PosZ, NegY],
        [PosZ, NegX, NegY],
        [NegX, NegZ, NegY],
        [NegZ, PosX, NegY],
    ]
}

fn parse_beacon(input: &str) -> IResult<&str, Point> {
    let (input, mut coords) = separated_list1(tag(","), nom::character::complete::i64)(input)?;
    if coords.len() == 2 {
        coords.push(0);
    }
    Ok((
        input,
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        },
    ))
}

fn parse_beacons(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, payload) = many1(terminated(parse_beacon, opt(line_ending)))(input)?;
    Ok((input, payload))
}

fn parse_header(input: &str) -> IResult<&str, String> {
    let (input, (name, _, number)) = delimited(
        tag("--- "),
        tuple((alphanumeric1, space1, alphanumeric1)),
        pair(tag(" ---"), line_ending),
    )(input)?;
    Ok((input, format!("{} {}", name, number)))
}

fn parse_scanners(input: &str) -> IResult<&str, Vec<Scanner>> {
    let (input, parsed_pairs) = many1(terminated(
        pair(parse_header, parse_beacons),
        many0(line_ending),
    ))(input)?;

    Ok((
        input,
        parsed_pairs
            .into_iter()
            .map(|(name, beacons)| Scanner::new(name, beacons))
            .collect(),
    ))
}
fn parse_primary(input: String) -> Vec<Scanner> {
    let (_, scanners) = parse_scanners(input.as_str()).unwrap();
    scanners
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn part_one_small_no_rotate() {
        let input = "--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1";
        let scanners = parse_primary(input.to_string());
        let overlapping_beacons = scanners[0].overlapping_beacons(&scanners[1], 3);
        assert_eq!(3, overlapping_beacons.unwrap().1.len());
        let solution = Solution::solve(scanners, 3);
        assert_eq!(3, solution.beacons.len())
    }

    #[test]
    fn orientations() {
        let orientations = make_orientations();
        println!("{:?}", orientations);
        assert_eq!(24, orientations.len());
    }

    #[test]
    fn rotation() {
        let orientations = make_orientations();
        let point = Point { x: 1, y: 2, z: 3 };

        let transformed = orientations
            .iter()
            .map(|o| point.rotate(o))
            .collect::<Vec<_>>();

        let expected = vec![
            Point::new(1, 2, 3),
            Point::new(2, -1, 3),
            Point::new(-1, -2, 3),
            Point::new(-2, 1, 3),
            Point::new(1, -2, -3),
            Point::new(-2, -1, -3),
            Point::new(-1, 2, -3),
            Point::new(2, 1, -3),
            //Y
            Point::new(-3, 2, 1),
            Point::new(2, 3, 1),
            Point::new(3, -2, 1),
            Point::new(-2, -3, 1),
            Point::new(3, 2, -1),
            Point::new(2, -3, -1),
            Point::new(-3, -2, -1),
            Point::new(-2, 3, -1),
            //Z
            Point::new(1, -3, 2),
            Point::new(-3, -1, 2),
            Point::new(-1, 3, 2),
            Point::new(3, 1, 2),
            Point::new(1, 3, -2),
            Point::new(3, -1, -2),
            Point::new(-1, -3, -2),
            Point::new(-3, 1, -2),
        ];

        assert_eq!(expected, transformed);

        let p = Point { x: 2, y: -1, z: 3 };
        let r = [NegX, NegZ, NegY];
        assert_eq!(Point::new(-2, -3, 1), p.rotate(&r));
    }

    #[test]
    fn part_one_small_rotate_2() {
        let input = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 1 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0";

        let mut scanners = parse_primary(input.to_string());
        scanners.reverse();
        let solution = Solution::solve(scanners, 6);
        assert_eq!(6, solution.beacons.len());
    }

    #[test]
    fn part_one_small_rotate() {
        let input = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 1 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 2 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 3 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 4 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";
        let scanners = parse_primary(input.to_string());
        let solution = Solution::solve(scanners, 6);
        assert_eq!(6, solution.beacons.len());
    }

    #[test]
    fn part_one_small() {
        let input = fs::read_to_string("input/day19-small").unwrap();
        let scanners = parse_primary(input.to_string());
        let solution = Solution::solve(scanners, 12);
        println!("solution: {:?}", solution);
        assert_eq!(79, solution.beacons.len());
        assert_eq!(3621, solution.max_distance());
    }
    #[test]
    fn part_one_small02() {
        let input = fs::read_to_string("input/day19-small02").unwrap();
        let scanners = parse_primary(input.to_string());
        let solution = Solution::solve(scanners, 12);

        assert_eq!(39, solution.beacons.len());
    }
    #[test]
    fn part_one_two() {
        let input = fs::read_to_string("input/day19").unwrap();
        let scanners = parse_primary(input.to_string());
        let solution = Solution::solve(scanners, 12);
        assert_eq!(438, solution.beacons.len());
        assert_eq!(3621, solution.max_distance());
    }
}
