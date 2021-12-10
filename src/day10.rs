use std::collections::HashMap;

use crate::reader::read_lines_filter_ok;

pub fn main() {
    println!(
        "Day 10-1 {}",
        syntax_error_score(read_lines_filter_ok("input/day10"))
    );
    println!(
        "Day 10-2 {}",
        incomplete_score(read_lines_filter_ok("input/day10"))
    );
}

fn syntax_error_score(input: Vec<String>) -> u32 {
    input
        .iter()
        .filter_map(|s| find_corrupt(&s))
        .filter_map(|c| score(c))
        .sum::<u32>()
}

fn incomplete_score(input: Vec<String>) -> u64 {
    let mut all_scores: Vec<_> = input
        .iter()
        .filter_map(|s| auto_complete(&s))
        .map(|c| incomplete_score_single(c))
        .collect();
    all_scores.sort();
    all_scores[all_scores.len() / 2]
}

fn incomplete_score_single(line: Vec<char>) -> u64 {
    fn score(c: char) -> Option<u64> {
        match c {
            ')' => Some(1),
            ']' => Some(2),
            '}' => Some(3),
            '>' => Some(4),
            _ => None,
        }
    }
    let mut result = 0;
    for c in line {
        result = result * 5 + score(c).unwrap();
    }
    result
}

fn auto_complete(line: &str) -> Option<Vec<char>> {
    let (mut stack, corrupt) = scan(line);
    stack.reverse();
    match corrupt {
        Some(_) => None,
        None => Some(stack),
    }
}

fn scan(line: &str) -> (Vec<char>, Option<char>) {
    let mut stack = Vec::<char>::new();
    let tokens: HashMap<char, char> =
        HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);

    for i in line.chars() {
        let opening = tokens.get(&i);

        match opening {
            Some(closing) => stack.push(*closing),
            None => match stack.pop() {
                None => {
                    return (vec![], None);
                }
                Some(expected) if i != expected => return (stack, Some(i)),
                _ => continue,
            },
        }
    }
    (stack, None)
}

fn find_corrupt(line: &str) -> Option<char> {
    let (_, corrupt) = scan(line);
    corrupt
}

fn score(c: char) -> Option<u32> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = vec![
            ("{([(<{}[<>[]}>{[]{[(<()>", '}'),
            ("[[<[([]))<([[{}[[()]]]", ')'),
            ("[{[{({}]{}}([{[{{{}}([]", ']'),
            ("[<(<(<(<{}))><([]([]()", ')'),
            ("<{([([[(<>()){}]>(<<{{", '>'),
        ];

        for (i, corrupt) in input.iter() {
            assert_eq!(Some(*corrupt), find_corrupt(i));
        }

        assert_eq!(
            26397u32,
            syntax_error_score(input.iter().map(|(i, _)| i.to_string()).collect())
        );
    }

    #[test]
    fn part_two() {
        let input = vec![
            ("[({(<(())[]>[[{[]{<()<>>", 288957),
            ("[(()[<>])]({[<{<<[]>>(", 5566),
            ("(((({<>}<{<{<>}{[]{[]{}", 1480781),
            ("{<[[]]>}<{[{[{[]{()[[[]", 995444),
            ("<{([{{}}[<[[[<>{}]]]>[]]", 294),
        ];
        for (i, incomplete) in input.iter() {
            assert_eq!(
                *incomplete,
                incomplete_score_single(auto_complete(i).unwrap())
            );
        }
        assert_eq!(
            288957,
            incomplete_score(input.iter().map(|(i, _)| i.to_string()).collect())
        );
    }
}
