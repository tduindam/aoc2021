use std::fmt;
use std::fmt::Formatter;
use std::path::Display;
use std::rc::Rc;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, u64};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::day18::Number::PairNumber;

#[derive(Debug)]
enum Number {
    Regular(i64),
    PairNumber(Rc<Pair>),
}

#[derive(Debug)]
struct Pair {
    numbers: (Number, Number),
}

impl Pair {
    fn new() -> Self {
        Self {
            numbers: (Number::Uninitialized, Number::Uninitialized),
        }
    }

    fn apply(&mut self, depth: u32) {
        let (n1, n2) = &self.numbers;
        match n1 {
            Number::Regular(_) => {}
            PairNumber(_) => {}
        }
    }
}

fn parse_number_digits(input: &str) -> IResult<&str, Number> {
    let (input, digits) = nom::character::complete::i64(input)?;
    Ok((input, Number::Regular(digits)))
}

fn parse_number_pair(input: &str) -> IResult<&str, Number> {
    let (input, pair) = parse_pair(input)?;
    Ok((input, Number::PairNumber(pair)))
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((parse_number_digits, parse_number_pair))(input)
}

fn parse_pair(input: &str) -> IResult<&str, Rc<Pair>> {
    let root = Rc::new(Pair::new());

    let (rest, (mut n1, mut n2)) = separated_pair(
        preceded(tag("["), parse_number),
        tag(","),
        terminated(parse_number, tag("]")),
    )(input)?;

    let pair = Rc::new(Pair { numbers: (n1, n2) });
    Ok((rest, pair))
}

fn parse_pair_primary(input: &str) -> Rc<Pair> {
    let (_, pair) = parse_pair(input).unwrap();
    pair
}

enum Day18Error {
    ParseError,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number::Regular(number) => {
                write!(f, "{}", number)
            }
            Number::PairNumber(pair) => {
                write!(f, "{}", *pair)
            }
            Number::Uninitialized => {
                write!(f, "Uninitialized")
            }
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.numbers.0, self.numbers.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        fn round_trip(input: &str) -> String {
            let pair = parse_pair_primary(input);
            pair.to_string()
        }

        assert_eq!("[1,2]", round_trip("[1,2]"));
        assert_eq!("[[1,2],3]", round_trip("[[1,2],3]"));
        assert_eq!("[9,[8,7]]", round_trip("[9,[8,7]]"));
        assert_eq!("[[1,9],[8,5]]", round_trip("[[1,9],[8,5]]"));
        assert_eq!(
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            round_trip("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")
        );
        assert_eq!(
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            round_trip("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")
        );
        assert_eq!(
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
            round_trip("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")
        );
    }

    #[test]
    fn explode() {
        assert_eq!(
            "[[[[[9,8],1],2],3],4]",
            parse_pair_primary("[[[[[9,8],1],2],3],4]").apply()
        );
        assert_eq!(
            "[7,[6,[5,[4,[3,2]]]]]",
            parse_pair_primary("[7,[6,[5,[7,0]]]]").apply()
        );
        assert_eq!(
            "[[6,[5,[4,[3,2]]]],1]",
            parse_pair_primary("[[6,[5,[7,0]]],3]").apply()
        );
        assert_eq!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            parse_pair_primary("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").apply()
        );
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            parse_pair_primary("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn part_one_small() {
        assert!(false);
    }

    #[test]
    fn part_one() {
        assert!(false);
    }

    #[test]
    fn part_two() {
        assert!(false);
    }
}
