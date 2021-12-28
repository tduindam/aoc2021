use crate::day19::Direction::*;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::character::streaming::space1;
use nom::combinator::opt;
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, terminated, tuple};
use nom::IResult;
use std::collections::HashSet;

enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
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

    fn rotate(&self, ori: &Vec<(i64, i64)>) -> Self {
        fn single_transform(p: &Point, (index, sign): (i64, i64)) -> i64 {
            let p = match index {
                0 => p.x,
                1 => p.y,
                2 => p.z,
                _ => {
                    unreachable!()
                }
            };
            p * sign
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
    fn delta(&self, index: usize, ori: &Vec<(i64, i64)>) -> (Point, Vec<Point>) {
        let mut deltas = Vec::<Point>::new();
        let origin = self.beacons[index];
        for i in 0..self.beacons.len() {
            if i == index {
                continue;
            }
            let rotated = self.beacons[i].rotate(ori);
            println!(
                "origin {:?} rotated {:?} beacon {:?}",
                origin, rotated, self.beacons[i]
            );
            deltas.push(origin.translate(&rotated));
        }
        (origin, deltas)
    }

    fn overlapping_beacons(&self, other: &Scanner) -> HashSet<Point> {
        let orientations = make_orientations();
        let (origin, deltas) = self.delta(0, &orientations[0]);
        let overlap: Option<(&Vec<(i64, i64)>, Point, i64)> = orientations
            .iter()
            .filter_map(|ori| {
                println!("Examining {:?}", ori);
                let overlaps = (0..other.beacons.len())
                    .map(|origin_index| {
                        let (other_origin, other_deltas) = other.delta(origin_index, ori);
                        assert_eq!(deltas.len(), other_deltas.len());
                        let dist: i64 = deltas
                            .iter()
                            .zip(other_deltas.iter())
                            .map(|(a, b)| a.manhattan_dist(b))
                            .sum();
                        (other_origin, dist)
                    })
                    .filter(|(_, cost)| *cost == 0)
                    .next();

                match overlaps {
                    Some((origin, distance)) => Some((ori, origin, distance)),
                    None => None,
                }
            })
            .next();
        let (_ori, other_origin, distance) = overlap.unwrap();
        //TODO: insert size of scanner
        assert_eq!(distance, 0);
        println!("Overlaps {:?} {}", other_origin, distance);
        let delta = other_origin.rotate(_ori).translate(&origin);
        let translated_points: Vec<Point> = other
            .beacons
            .iter()
            .map(|b| {
                let b = b.rotate(_ori);
                b.translate(&delta)
            })
            .collect();
        println!("Points {:?}", translated_points);

        let mut unique_beacons: HashSet<Point> = self.beacons.clone().into_iter().collect();
        unique_beacons.extend(translated_points.iter());

        println!("Unique Beacons: {:?}", unique_beacons);
        unique_beacons
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
    fn solve(mut scanners: Vec<Scanner>, overlap_threshold: usize) -> Self {
        let mut first = scanners.pop().unwrap();
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
                                let overlap = scanner_a.overlapping_beacons(scanner_b);

                                if overlap.len() >= overlap_threshold {
                                    Some((overlap, scanner_a))
                                } else {
                                    None
                                }
                            })
                            .next();
                        match overlap {
                            None => None,
                            Some((overlap, scanner_a)) => Some((overlap, scanner_a, scanner_b)),
                        }
                    })
                    .next()
            };

            //must have overlap
            let (overlap, scanner_a, scanner_b) = overlap.unwrap();
            //Overlapping points are in 'scanner_a' translation, just add the relative translation of that scanner
            let name = scanner_b.name.clone();
            solution.scanners.push(scanner_b.clone());
            //ignore scanner positions
            solution.beacons.extend(overlap.iter());
            scanners.retain(|s| s.name != name);
        }

        solution
    }
}

fn overlapping(scanners: Vec<Scanner>) -> HashSet<Point> {
    let first = scanners[0].clone();
    scanners
        .iter()
        .fold(HashSet::<Point>::new(), |mut acc, scanner| {
            if first.name == scanner.name {
                acc.extend(first.beacons.iter());
            } else {
                acc.extend(first.overlapping_beacons(scanner).iter());
            }
            acc
        })
}

//Orientations, first is index second is sign
fn make_orientations() -> Vec<Vec<(i64, i64)>> {
    vec![
        vec![PosX, PosY, PosZ],
        vec![PosY, NegX, PosZ],
        vec![NegX, NegY, PosZ],
        vec![NegY, PosX, PosZ],
        vec![PosX, NegY, NegZ],
        vec![NegY, NegX, NegZ],
        vec![NegX, PosY, NegZ],
        vec![PosY, PosX, NegZ],
        vec![NegZ, PosY, PosX],
        vec![PosY, PosZ, PosX],
        vec![PosZ, NegY, PosX],
        vec![NegY, NegZ, PosX],
        vec![PosZ, PosY, NegX],
        vec![PosY, NegZ, NegX],
        vec![NegZ, NegY, NegX],
        vec![NegY, PosZ, NegX],
        vec![PosX, NegZ, PosY],
        vec![NegZ, NegX, PosY],
        vec![NegX, PosZ, PosY],
        vec![PosZ, PosX, PosY],
        vec![PosX, PosZ, NegY],
        vec![PosZ, NegX, NegY],
        vec![NegX, NegZ, NegY],
        vec![NegZ, PosX, NegY],
    ];

    let dirs = vec![0, 1, 2];
    let permutations = dirs.iter().permutations(3);
    permutations
        .map(|dirs| {
            vec![
                // vec![(*dirs[2], 1), (*dirs[1], 1), (*dirs[0], 1)],
                // vec![(*dirs[2], -1), (*dirs[1], -1), (*dirs[0], 1)],
                // vec![(*dirs[2], -1), (*dirs[1], 1), (*dirs[0], 1)],
                // vec![(*dirs[2], 1), (*dirs[1], -1), (*dirs[0], 1)],
                vec![(*dirs[0], 1), (*dirs[1], 1), (*dirs[2], 1)],
                vec![(*dirs[0], -1), (*dirs[1], -1), (*dirs[2], 1)],
                vec![(*dirs[0], -1), (*dirs[1], 1), (*dirs[2], 1)],
                vec![(*dirs[0], 1), (*dirs[1], -1), (*dirs[2], 1)],
            ]
        })
        .flatten()
        .collect::<Vec<_>>()
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
    let (input, payload) = many1(terminated(parse_beacon, opt(newline)))(input)?;
    Ok((input, payload))
}

fn parse_header(input: &str) -> IResult<&str, String> {
    let (input, (name, _, number)) = delimited(
        tag("--- "),
        tuple((alphanumeric1, space1, alphanumeric1)),
        tag(" ---\n"),
    )(input)?;
    Ok((input, format!("{} {}", name, number)))
}

fn parse_scanners(input: &str) -> IResult<&str, Vec<Scanner>> {
    let (input, parsed_pairs) = many1(terminated(
        pair(parse_header, parse_beacons),
        many0(newline),
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
        let overlapping_beacons = scanners[0].overlapping_beacons(&scanners[1]);
        assert_eq!(3, overlapping_beacons.len());
        assert_eq!(3, overlapping(scanners.clone()).len());
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
            //facing +x / -x
            Point::new(1, 2, 3),
            Point::new(-1, -2, 3),
            Point::new(-1, 2, 3),
            Point::new(1, -2, 3),
            Point::new(1, 3, 2),
            Point::new(-1, -3, 2),
            Point::new(-1, 3, 2),
            Point::new(1, -3, 2),
            //Y
            Point::new(2, 1, 3),
            Point::new(-2, -1, 3),
            Point::new(-2, 1, 3),
            Point::new(2, -1, 3),
            Point::new(2, 3, 1),
            Point::new(-2, -3, 1),
            Point::new(-2, 3, 1),
            Point::new(2, -3, 1),
            //Z
            Point::new(3, 1, 2),
            Point::new(-3, -1, 2),
            Point::new(-3, 1, 2),
            Point::new(3, -1, 2),
            Point::new(3, 2, 1),
            Point::new(-3, -2, 1),
            Point::new(-3, 2, 1),
            Point::new(3, -2, 1),
        ];

        assert_eq!(expected, transformed);
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
        assert_eq!(79, solution.beacons.len());
    }
    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/day19").unwrap();
        let scanners = parse_primary(input.to_string());
        let solution = Solution::solve(scanners, 12);
        assert_eq!(0, solution.beacons.len());
    }

    #[test]
    fn part_two() {
        assert!(false);
    }
}
