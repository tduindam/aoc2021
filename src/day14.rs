use std::collections::{HashMap, VecDeque};

use crate::reader::read_lines_filter_ok;

type InstructionMap = HashMap<(char, char), char>;

pub fn main() {
    let lines = read_lines_filter_ok("input/day14");
    let (start, instructions) = parse_lines(&lines);
    println!("Day 14-2 {}", expansion_score_2(&start, &instructions, 40));
}

fn expansion_score_2(input: &String, instructions: &InstructionMap, iterations: u32) -> u128 {
    let counts = expand_3(input, instructions, iterations);
    let mut vec: Vec<u128> = counts.into_values().collect();
    vec.sort();
    vec.last().unwrap() - vec[0]
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

fn expand_r(
    elem: (char, char),
    instructions: &InstructionMap,
    counts: &mut HashMap<char, u128>,
    cur_depth: u32,
    max_depth: u32,
) {
    let c = *instructions.get(&elem).unwrap();
    *counts.entry(c).or_insert(0) += 1;
    let (c0, c1) = elem;
    if cur_depth != max_depth {
        expand_r((c0, c), instructions, counts, cur_depth + 1, max_depth);
        // expand_r((c, c1), instructions, counts, cur_depth + 1, max_depth);
    }
}

fn expand_3(input: &String, instructions: &InstructionMap, iterations: u32) -> HashMap<char, u128> {
    let mut counts = HashMap::<char, u128>::new();

    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut start: Vec<(char, char)> = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|pair| {
            let mut it = pair.iter();
            (*it.next().unwrap(), *it.next().unwrap())
        })
        .collect();

    for v in start {
        expand_r(v, instructions, &mut counts, 0, iterations);
        println!("counts {:?}", counts);
        println!("b");
    }
    counts
}

fn expand_2(input: &String, instructions: &InstructionMap, iterations: u32) -> HashMap<char, u128> {
    let mut counts = HashMap::<char, u128>::new();

    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut queue: VecDeque<(char, char)> = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|pair| {
            let mut it = pair.iter();
            (*it.next().unwrap(), *it.next().unwrap())
        })
        .collect();
    queue.push_back(('X', 'X'));
    let mut iteration_counter = 0;
    while iteration_counter < iterations {
        let next = queue.pop_front().unwrap();
        if next == ('X', 'X') {
            queue.push_back(('X', 'X'));
            iteration_counter += 1;
            println!("Round {}", iteration_counter);
            continue;
        }
        let new_char = instructions.get(&next).unwrap();
        *counts.entry(*new_char).or_insert(0) += 1;
        let (c0, c1) = next;
        queue.push_back((c0, *new_char));
        queue.push_back((*new_char, c1));
        // println!("Q {:?}", queue);
    }
    counts
}

fn expand(input: &String, instructions: &InstructionMap) -> String {
    let mut exp = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|pair| {
            let new_elem = instructions.get(&(pair[0], pair[1])).unwrap();
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
            let mut in_chars = chunks.next().unwrap().trim().chars();
            let pair = ((in_chars.next().unwrap(), in_chars.next().unwrap()));
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
        assert_eq!(1, expansion_score_2(&start, &instructions, 1));
        assert_eq!(1588, expansion_score_2(&start, &instructions, 10));
    }

    #[test]
    fn part_one() {
        let lines = read_lines_filter_ok("input/day14");
        let (start, instructions) = parse_lines(&lines);
        assert_eq!(3587, expansion_score(&start, &instructions, 10));
        assert_eq!(3587, expansion_score_2(&start, &instructions, 10));
    }

    #[test]
    fn part_two() {
        let lines = read_lines_filter_ok("input/day14");
        let (start, instructions) = parse_lines(&lines);
        assert_eq!(3587, expansion_score_2(&start, &instructions, 25));
    }
}
