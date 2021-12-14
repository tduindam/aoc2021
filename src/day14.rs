use std::collections::HashMap;

use crate::reader::read_lines_filter_ok;

type InstructionMap = HashMap<(char, char), char>;

pub fn main() {
    let lines = read_lines_filter_ok("input/day14");
    let (start, instructions) = parse_lines(&lines);
    println!("Day 14-2 {}", expansion_score(&start, &instructions, 40));
}

fn expansion_score(input: &String, instructions: &InstructionMap, iterations: u32) -> u128 {
    let counts = expand(input, instructions, iterations);
    let mut vec: Vec<u128> = counts.into_values().collect();
    vec.sort();
    vec.last().unwrap() - vec[0]
}

fn expand(input: &String, instructions: &InstructionMap, iterations: u32) -> HashMap<char, u128> {
    type InvocationMap = HashMap<(char, char), u64>;
    let mut counts = HashMap::<char, u128>::new();
    fn add_invocation(key: (char, char), map: &mut InvocationMap, count: u64) {
        *map.entry(key).or_insert(0) += count;
    }

    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut invocations_next_round = InvocationMap::new();
    for starting_pair in input.chars().collect::<Vec<char>>().windows(2) {
        add_invocation(
            (starting_pair[0], starting_pair[1]),
            &mut invocations_next_round,
            1,
        );
    }
    for _ in 0..iterations {
        let mut next_invocations = InvocationMap::new();
        for ((c0, c1), count) in invocations_next_round {
            let c = *instructions.get(&(c0, c1)).unwrap();
            *counts.entry(c).or_insert(0) += count as u128;
            add_invocation((c0, c), &mut next_invocations, count);
            add_invocation((c, c1), &mut next_invocations, count);
        }
        invocations_next_round = next_invocations;
    }

    counts
}

fn parse_lines(lines: &Vec<String>) -> (String, InstructionMap) {
    let input = lines[0].to_string();
    let map: InstructionMap = lines
        .iter()
        .skip(2)
        .map(|l| {
            let mut chunks = l.split("->");
            let mut in_chars = chunks.next().unwrap().trim().chars();
            let pair = (in_chars.next().unwrap(), in_chars.next().unwrap());
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
        assert_eq!(3906445077999, expansion_score(&start, &instructions, 40));
    }
}
