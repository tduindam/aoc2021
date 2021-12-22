use std::cell::RefCell;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::day18::Number::{PairNumber, Regular};

type RcPair = Rc<RefCell<Pair>>;

#[derive(Debug)]
enum Number {
    Regular(i64),
    PairNumber(RcPair),
}

impl Default for Number {
    fn default() -> Self {
        Regular(0)
    }
}

#[derive(Debug, Default)]
struct Pair {
    numbers: (Number, Number),
}

fn traverse(pair: RcPair, steps: Vec<RcPair>) {
    if steps.len() == 4 {
        explode(pair.clone(), steps);
        return;
    }

    let (n1, n2) = &pair.borrow().numbers;
    fn traverse_number(number: &Number, mut steps: Vec<RcPair>) {
        match number {
            Number::Regular(n) => {
                println!("Found number {} at depth: {}", n, steps.len())
            }
            PairNumber(pair) => {
                steps.push(pair.clone());
                traverse(pair.clone(), steps);
            }
        }
    }

    traverse_number(n1, steps.clone());
    traverse_number(n2, steps.clone());
}

fn explode(pair: RcPair, steps: Vec<RcPair>) {
    let (n1, n2) = &pair.borrow().numbers;
    let _n1 = if let Regular(n1) = n1 { n1} else { panic!("Invalid explode")};
    let _n2 = if let Regular(n2) = n2 { n2} else { panic!("Invalid explode")};
    pair.borrow_mut().
}

impl Pair {
    fn apply(mut self) -> Self {
        let wrapped = Rc::new(RefCell::new(self));
        traverse(wrapped.clone(), Vec::new());
        wrapped.take()
    }
}

fn parse_number_digits(input: &str) -> IResult<&str, Number> {
    let (input, digits) = nom::character::complete::i64(input)?;
    Ok((input, Number::Regular(digits)))
}

fn parse_number_pair(input: &str) -> IResult<&str, Number> {
    let (input, pair) = parse_pair(input)?;
    Ok((input, Number::PairNumber(Rc::new(RefCell::new(pair)))))
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((parse_number_digits, parse_number_pair))(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (rest, (n1, n2)) = separated_pair(
        preceded(tag("["), parse_number),
        tag(","),
        terminated(parse_number, tag("]")),
    )(input)?;

    let pair = Pair { numbers: (n1, n2) };
    Ok((rest, pair))
}

fn parse_pair_primary(input: &str) -> Pair {
    let (_, pair) = parse_pair(input).unwrap();
    pair
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number::Regular(number) => {
                write!(f, "{}", number)
            }
            Number::PairNumber(pair) => {
                write!(f, "{}", pair.borrow())
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
            parse_pair_primary("[[[[[9,8],1],2],3],4]")
                .apply()
                .to_string()
        );
        assert_eq!(
            "[7,[6,[5,[4,[3,2]]]]]",
            parse_pair_primary("[7,[6,[5,[7,0]]]]").apply().to_string()
        );
        assert_eq!(
            "[[6,[5,[4,[3,2]]]],1]",
            parse_pair_primary("[[6,[5,[7,0]]],3]").apply().to_string()
        );
        assert_eq!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            parse_pair_primary("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
                .apply()
                .to_string()
        );
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            parse_pair_primary("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").to_string()
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
