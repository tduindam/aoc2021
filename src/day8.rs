use crate::reader::filter_ok_lines;

pub fn main() {
    let input = filter_ok_lines("input/day8")
        .iter()
        .filter_map(|s| parse_line(s).ok())
        .collect();
    println!("Day 8-1: {}", count_easies(input));
}

enum Day8Error {
    ParseError,
}

type Input = (Vec<String>, Vec<String>);

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

fn parse_line(line: &str) -> Result<Input, Day8Error> {
    let patterns = line.split("|").next().ok_or(Day8Error::ParseError)?;
    let values = line
        .split("|")
        .skip(1)
        .next()
        .ok_or(Day8Error::ParseError)?;
    let patterns: Vec<String> = patterns
        .split(" ")
        .filter(|p| p.len() > 0)
        .take(10)
        .map(|v| v.to_string())
        .collect();
    let values: Vec<String> = values
        .split(" ")
        .filter(|p| p.len() > 0)
        .take(4)
        .map(|v| v.to_string())
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
}
