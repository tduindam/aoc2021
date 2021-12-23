use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::day18::NumberType::{PairNumber, Regular};

type RcNumber = Rc<RefCell<Number>>;

static NEXT_ID: AtomicI64 = AtomicI64::new(0);

#[derive(Debug)]
enum NumberType {
    Regular(i64),
    PairNumber(RcNumber, RcNumber),
}

impl Default for NumberType {
    fn default() -> Self {
        Regular(0)
    }
}

#[derive(Debug, Default)]
struct Number {
    id: i64,
    content: NumberType,
    depth: u64,
    parent: Option<RcNumber>,
}

impl Number {
    fn new_rc(content: NumberType) -> RcNumber {
        Rc::new(RefCell::new(Self::new(content)))
    }

    fn new(content: NumberType) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        ();
        Self {
            id,
            content,
            depth: 0,
            parent: None,
        }
    }

    fn make_string(&self, only_ids: bool) -> String {
        match &self.content {
            NumberType::Regular(number) => {
                format!("{}", if only_ids { self.id } else { *number })
            }
            NumberType::PairNumber(left, right) => {
                format!(
                    "[{},{}]",
                    left.borrow().make_string(only_ids),
                    right.borrow().make_string(only_ids)
                )
            }
        }
    }
}

fn flatten(number: RcNumber) -> Vec<RcNumber> {
    flatten_impl(number, 0, None)
}

fn flatten_impl(number: RcNumber, depth: u64, parent: Option<RcNumber>) -> Vec<RcNumber> {
    {
        let mut number = number.borrow_mut();
        number.depth = depth;
        number.parent = parent;
    }
    match &number.borrow().content {
        NumberType::Regular(_) => {
            vec![number.clone()]
        }
        NumberType::PairNumber(left, right) => {
            let mut left = flatten_impl(left.clone(), depth + 1, Some(number.clone()));
            left.append(&mut flatten_impl(
                right.clone(),
                depth + 1,
                Some(number.clone()),
            ));
            left
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.make_string(false))
    }
}

#[derive(Debug)]
enum Action {
    Explode(RcNumber),
    Split(RcNumber),
}

fn reduce(number: RcNumber) -> RcNumber {
    let mut actions = VecDeque::<Action>::new();

    let mut check_first = Vec::<usize>::new();
    let mut only_explosions = false;
    let mut only_splits = false;

    'outer: loop {
        let mut flat_map = flatten(number.clone(), 0, None);
        println!(
            "Reduction step {} {:?}",
            number.clone().borrow(),
            check_first
        );
        let extra_step = !check_first.is_empty();
        let range: Box<dyn Iterator<Item = usize>> = if !extra_step {
            only_explosions = false;
            only_splits = false;

            Box::new(0..flat_map.len())
        } else {
            Box::new(check_first.clone().into_iter())
        };
        check_first.clear();
        for i in range {
            let must_explode = { flat_map[i].borrow().depth } > 4 && !only_splits;

            let must_split = {
                if let Regular(x) = flat_map[i].borrow().content {
                    x >= 10
                } else {
                    false
                }
            } && !only_explosions;
            if extra_step {
                println!(
                    "Checking [{}]{} must_explode {} must_split {}",
                    i,
                    flat_map[i].borrow(),
                    must_explode,
                    must_split
                );
            }
            if must_explode {
                println!(
                    "Exploding: [{}] {} {}",
                    i,
                    flat_map[i].borrow(),
                    flat_map[i + 1].borrow()
                );
                check_first = explode(&flat_map, i);
                only_splits = true;
                only_explosions = false;
                if !extra_step {
                    continue 'outer;
                }
            }
            if must_split {
                println!("Split: {}", flat_map[i].borrow());
                check_first = split(&flat_map, i);
                only_explosions = true;
                only_splits = false;

                if !extra_step {
                    continue 'outer;
                }
            }
        }
        if extra_step {
            continue 'outer;
        }
        return number;
    }
}

fn magnitude(number: RcNumber) -> i64 {
    match &number.borrow().content {
        Regular(n) => *n,
        PairNumber(l, r) => 3 * magnitude(l.clone()) + 2 * magnitude(r.clone()),
    }
}

fn sum(numbers: Vec<String>) -> Number {
    numbers
        .iter()
        .map(|s| parse_pair_primary(s.as_str()))
        .fold(None, |acc, next| add(acc, next))
        .unwrap()
        .take()
}

fn add(left: Option<RcNumber>, right: RcNumber) -> Option<RcNumber> {
    match left {
        None => Some(right),
        Some(left) => {
            let num = Number::new_rc(PairNumber(left, right));
            reduce(num.clone());
            println!("Step: {}", num.borrow());
            Some(num)
        }
    }
}

fn split(flat_map: &Vec<RcNumber>, i: usize) -> Vec<usize> {
    let value = extract_value(flat_map[i].clone()) as f64;
    let left = Regular((value / 2f64).floor() as i64);
    let right = Regular((value / 2f64).ceil() as i64);
    let new_pair = PairNumber(Number::new_rc(left), Number::new_rc(right));
    flat_map[i].borrow_mut().content = new_pair;
    vec![i]
}

fn explode(flat_map: &Vec<RcNumber>, i: usize) -> Vec<usize> {
    let parent = { flat_map[i].borrow().parent.as_ref().unwrap().clone() };
    let (l_val, r_val) = if let PairNumber(left, right) = &parent.borrow().content {
        (extract_value(left.clone()), extract_value(right.clone()))
    } else {
        unreachable!()
    };
    let mut to_check = Vec::<usize>::new();
    if i > 0 {
        let cur = extract_value(flat_map[i - 1].clone());
        flat_map[i - 1].borrow_mut().content = Regular(cur + l_val);
        to_check.push(i - 1);
    }
    //Add 2 since this function will trigger on left most of pair
    if i + 2 < flat_map.len() {
        let cur = extract_value(flat_map[i + 2].clone());
        flat_map[i + 2].borrow_mut().content = Regular(cur + r_val);
        to_check.push(i + 1);
    }
    parent.borrow_mut().content = Regular(0);
    to_check
}

fn extract_value(left: RcNumber) -> i64 {
    if let Regular(l) = &left.borrow().content {
        *l
    } else {
        unreachable!()
    }
}

impl Number {
    fn apply(self) -> Self {
        let wrapped = Rc::new(RefCell::new(self));
        reduce(wrapped.clone());
        wrapped.take()
    }
}

fn parse_number_digits(input: &str) -> IResult<&str, NumberType> {
    let (input, digits) = nom::character::complete::i64(input)?;
    Ok((input, NumberType::Regular(digits)))
}

fn parse_number_pair(input: &str) -> IResult<&str, NumberType> {
    let (input, (left, right)) = parse_pair(input)?;
    Ok((input, NumberType::PairNumber(left, right)))
}

fn parse_number(input: &str) -> IResult<&str, NumberType> {
    alt((parse_number_digits, parse_number_pair))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (RcNumber, RcNumber)> {
    let (rest, (n1, n2)) = separated_pair(
        preceded(tag("["), parse_number),
        tag(","),
        terminated(parse_number, tag("]")),
    )(input)?;

    Ok((
        rest,
        (
            Rc::new(RefCell::new(Number::new(n1))),
            Rc::new(RefCell::new(Number::new(n2))),
        ),
    ))
}

fn parse_pair_primary(input: &str) -> RcNumber {
    let (_, (l, r)) = parse_pair(input).unwrap();
    Rc::new(RefCell::new(Number::new(PairNumber(l, r))))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reader::{read_lines_filter_ok, split_lines};

    #[test]
    fn parse() {
        fn round_trip(input: &str) -> String {
            let pair = parse_pair_primary(input);
            {
                println!("{}", pair.borrow().make_string(true));
                let x = pair.borrow().to_string();
                x
            }
            .clone()
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
            "[[[[0,9],2],3],4]",
            parse_pair_primary("[[[[[9,8],1],2],3],4]")
                .take()
                .apply()
                .to_string()
        );
        assert_eq!(
            "[7,[6,[5,[7,0]]]]",
            parse_pair_primary("[7,[6,[5,[4,[3,2]]]]]")
                .take()
                .apply()
                .to_string()
        );
        assert_eq!(
            "[[6,[5,[7,0]]],3]",
            parse_pair_primary("[[6,[5,[4,[3,2]]]],1]")
                .take()
                .apply()
                .to_string()
        );
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            parse_pair_primary("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
                .take()
                .apply()
                .to_string()
        );
    }

    #[test]
    fn reduce() {
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            parse_pair_primary("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
                .take()
                .apply()
                .to_string()
        )
    }
    #[test]
    fn test_magnitude() {
        assert_eq!(143, magnitude(parse_pair_primary("[[1,2],[[3,4],5]]")));
        assert_eq!(
            1384,
            magnitude(parse_pair_primary("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))
        );
        assert_eq!(
            445,
            magnitude(parse_pair_primary("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
        );
        assert_eq!(
            3488,
            magnitude(parse_pair_primary(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ))
        );
    }

    #[test]
    fn add_simple() {
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            sum(split_lines(
                "[[[[4,3],4],4],[7,[[8,4],9]]]
    [1,1]"
            ))
            .to_string()
        );

        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            sum(split_lines(
                "[1,1]
[2,2]
[3,3]
[4,4]",
            ))
            .to_string()
        );

        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            sum(split_lines(
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]",
            ))
            .to_string()
        );

        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            sum(split_lines(
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
                [6,6]",
            ))
            .to_string()
        );
    }

    #[test]
    fn add_complex() {
        assert_eq!(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            sum(split_lines(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            ))
            .to_string()
        );

        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            sum(split_lines(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            ))
            .to_string()
        );

        assert_eq!(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            sum(split_lines(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ))
            .to_string()
        );
    }

    #[test]
    fn part_one_small() {
        let summed = Rc::new(RefCell::new(sum(split_lines(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ))));
        assert_eq!(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            summed.borrow().to_string()
        );
        assert_eq!(4140, magnitude(summed));
    }

    #[test]
    fn part_one() {
        assert_eq!(
            0,
            magnitude(Rc::new(RefCell::new(sum(read_lines_filter_ok(
                "input/day18"
            )))))
        );
    }

    #[test]
    fn part_two() {
        assert!(false);
    }
}
