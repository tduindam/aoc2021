use std::collections::HashMap;

use crate::reader::read_lines_filter_ok;

type InstructionMap = HashMap<String, char>;

pub fn main() {
    let lines = read_lines_filter_ok("input/day14");
    let (start, instructions) = parse_lines(&lines);
    println!("Day 14-2 {}", expansion_score(&start, &instructions, 40));
}

fn expansion_score(input: &String, instructions: &InstructionMap, iterations: u32) -> u32 {
    let mut expanded = input.to_string();
    for i in 0..iterations {
        expanded = expand(&expanded, &instructions);
        println!("Round {} size {}", i, expanded.len());
    }
    let mut counts = HashMap::<char, u32>::new();
    for c in expanded.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut vec: Vec<u32> = counts.into_values().collect();
    vec.sort();
    vec.last().unwrap() - vec[0]
}

fn expand(input: &String, instructions: &InstructionMap) -> String {
    let mut exp = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|pair| {
            let key = pair.iter().collect::<String>();
            let new_elem = instructions.get(&key).unwrap();
            vec![pair[0], *new_elem].iter().collect::<String>()
        })
        .collect::<String>();
    exp.push(input.chars().last().unwrap());
    exp
}

fn parse_lines(lines: &Vec<String>) -> (String, InstructionMap) {
    let input = lines[0].to_string();
    let map: InstructionMap = lines
        .iter()
        .skip(2)
        .map(|l| {
            let mut chunks = l.split("->");
            let pair = chunks.next().unwrap().trim().to_string();
            let output = chunks.next().unwrap().trim().chars().next().unwrap();
            (pair, output)
        })
        .collect();
    (input, map)
}

#[cfg(test)]
mod test {
    use crate::reader::{read_lines_filter_ok, split_lines};

    use super::*;

    #[test]
    fn part_one_small() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let lines = split_lines(input);
        let (start, instructions) = parse_lines(&lines);

        assert_eq!("NNCB", start);
        assert_eq!(16, instructions.len());
        println!("instructions {:?}", instructions);
        let expanded = expand(&start, &instructions);
        assert_eq!("NCNBCHB", expanded);
        let expanded = expand(&expanded, &instructions);
        assert_eq!("NBCCNBBBCBHCB", expanded);
        let expanded = expand(&expanded, &instructions);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", expanded);
        let expanded = expand(&expanded, &instructions);
        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            expanded
        );
        assert_eq!(1588, expansion_score(&start, &instructions, 10));
    }

    #[test]
    fn part_one() {
        let lines = read_lines_filter_ok("input/day14");
        let (start, instructions) = parse_lines(&lines);
        assert_eq!(3587, expansion_score(&start, &instructions, 10));
    }

    #[test]
    fn part_two() {
        let lines = read_lines_filter_ok("input/day14");
        let (start, instructions) = parse_lines(&lines);
        assert_eq!(3587, expansion_score(&start, &instructions, 40));
    }
}
