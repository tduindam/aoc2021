use crate::reader::filter_ok_lines;

pub fn main() {
    let input = filter_ok_lines("input/day8")
        .iter()
        .filter_map(|s| parse_line(s).ok())
        .collect();
    println!("Day 8-1: {}", count_easies(input));
    let input = filter_ok_lines("input/day8")
        .iter()
        .filter_map(|s| parse_line(s).ok())
        .collect();

    println!("Day 8-2: {}", sum_full_parsed(input));
}

#[derive(Debug)]
enum Day8Error {
    ParseError,
    DeductionFailed,
}

type Input = (Vec<String>, Vec<String>);
type DeductionState = Vec<Option<String>>;

fn is_easy(input: &str) -> bool {
    match input.len() {
        2 => true, //1
        3 => true, //7
        4 => true, //4,
        7 => true, //8,
        _ => false,
    }
}

fn count_easies(inputs: Vec<Input>) -> u32 {
    inputs
        .iter()
        .map(|(_, values)| values.iter().filter(|v| is_easy(v.as_str())).count() as u32)
        .sum::<u32>()
}

fn sum_full_parsed(inputs: Vec<Input>) -> u32 {
    inputs
        .iter()
        .filter_map(|input| deduce_digits(input).ok())
        .sum::<u32>()
}

fn parse_zero_six_nine(input: &str, state: &DeductionState) -> Option<u32> {
    assert_eq!(input.len(), 6);
    if let Some(nine) = parse_nine(input, state) {
        return Some(nine);
    }
    if let Some(six) = parse_six(input, state) {
        return Some(six);
    }
    if state[6].is_some() && state[9].is_some() {
        //Length is 6, but not nine or six, must be 0
        Some(0)
    } else {
        None
    }
}

fn parse_nine(input: &str, state: &DeductionState) -> Option<u32> {
    if let Some(parsed_4) = state[4].as_ref() {
        let mut parsed_4 = parsed_4.chars();
        //9 must overlap completely with 4
        if parsed_4.all(|c| input.chars().any(|a| a == c)) {
            Some(9)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_six(input: &str, state: &DeductionState) -> Option<u32> {
    if let Some(parsed_1) = state[1].as_ref() {
        let parsed_1 = parsed_1.chars();
        //6 can't overlap with 1 completely
        if parsed_1.filter(|c| input.chars().any(|a| a == *c)).count() == 1 {
            Some(6)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_two_three_five(input: &str, state: &DeductionState) -> Option<u32> {
    if let Some(three) = parse_three(input, state) {
        return Some(three);
    }
    if let Some(five) = parse_five(input, state) {
        return Some(five);
    }
    if state[3].is_some() && state[5].is_some() {
        //Length is 5, but not 3 or 5, must be 2
        Some(2u32)
    } else {
        None
    }
}

fn parse_three(input: &str, state: &DeductionState) -> Option<u32> {
    if let Some(parsed_1) = state[1].as_ref() {
        let mut parsed_1 = parsed_1.chars();
        //3 must overlap with 1 completely
        if parsed_1.all(|c| input.chars().any(|a| a == c)) {
            Some(3)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_five(input: &str, state: &DeductionState) -> Option<u32> {
    if let Some(parsed_6) = state[6].as_ref() {
        let parsed_6 = parsed_6.chars();
        //6 has exactly 1 char not in 5
        if parsed_6.filter(|c| !input.chars().any(|a| a == *c)).count() == 1 {
            Some(5)
        } else {
            None
        }
    } else {
        None
    }
}

fn deduce(input: &str, state: &DeductionState) -> Option<u32> {
    match (input, input.len()) {
        (_, 2) => Some(1), //1
        (_, 4) => Some(4), //4,
        (i, 6) => parse_zero_six_nine(i, state),
        (i, 5) => parse_two_three_five(i, state),
        (_, 3) => Some(7), //7
        (_, 7) => Some(8), //8,
        _ => None,
    }
}

fn deduce_digits((patterns, values): &Input) -> Result<u32, Day8Error> {
    let mut state: DeductionState = vec![None; 10];

    let mut undeduced = patterns.clone();
    while undeduced.len() > 0 {
        let count_before = undeduced.len();
        undeduced.retain(|pattern| {
            let deduction = deduce(pattern, &state);
            match deduction {
                None => true,
                Some(deduction) => {
                    state[deduction as usize] = Some(pattern.clone());
                    false
                }
            }
        });
        let count_after = undeduced.len();
        if count_before == count_after {
            return Err(Day8Error::DeductionFailed);
        }
    }
    if state.iter().any(|v| v.is_none()) {
        return Err(Day8Error::DeductionFailed);
    }
    let state: Vec<_> = state.into_iter().filter_map(|f| f).collect();

    let digits: Vec<usize> = values
        .iter()
        .map(|v| state.iter().position(|parsed| parsed == v).unwrap())
        .collect();

    Ok((digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]) as u32)
}

fn parse_line(line: &str) -> Result<Input, Day8Error> {
    let patterns = line.split("|").next().ok_or(Day8Error::ParseError)?;
    let values = line
        .split("|")
        .skip(1)
        .next()
        .ok_or(Day8Error::ParseError)?;

    fn sort(v: &str) -> String {
        let mut l: Vec<char> = v.chars().collect();
        l.sort_unstable();
        l.into_iter().collect()
    }
    let patterns: Vec<String> = patterns
        .split(" ")
        .filter(|p| p.len() > 0)
        .take(10)
        .map(sort)
        .collect();
    let values: Vec<String> = values
        .split(" ")
        .filter(|p| p.len() > 0)
        .take(4)
        .map(sort)
        .collect();
    if patterns.len() != 10 || values.len() != 4 {
        Err(Day8Error::ParseError)
    } else {
        Ok((patterns, values))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<&'static str> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
    }

    #[test]
    fn part_one_parse() {
        assert!(test_input()
            .iter()
            .map(|s| parse_line(&s))
            .all(|r| r.is_ok()));
    }

    #[test]
    fn part_one() {
        let input: Vec<Input> = test_input()
            .iter()
            .filter_map(|p| parse_line(&p).ok())
            .collect();
        assert_eq!(26, count_easies(input));
    }

    #[test]
    fn part_two_one_line() {
        let input = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();
        assert_eq!(5353, deduce_digits(&input).unwrap());
    }

    #[test]
    fn part_two_all_parts() {
        let input: Vec<Input> = test_input()
            .iter()
            .filter_map(|p| parse_line(&p).ok())
            .collect();
        assert_eq!(8394, deduce_digits(&input[0]).unwrap());
        assert_eq!(9781, deduce_digits(&input[1]).unwrap());
        assert_eq!(1197, deduce_digits(&input[2]).unwrap());
        assert_eq!(9361, deduce_digits(&input[3]).unwrap());
        assert_eq!(4873, deduce_digits(&input[4]).unwrap());
        assert_eq!(8418, deduce_digits(&input[5]).unwrap());
        assert_eq!(4548, deduce_digits(&input[6]).unwrap());
        assert_eq!(1625, deduce_digits(&input[7]).unwrap());
        assert_eq!(8717, deduce_digits(&input[8]).unwrap());
        assert_eq!(4315, deduce_digits(&input[9]).unwrap());
    }

    #[test]
    fn part_two() {
        let input: Vec<Input> = test_input()
            .iter()
            .filter_map(|p| parse_line(&p).ok())
            .collect();
        assert_eq!(61229, sum_full_parsed(input));
    }
}
