use crate::reader::read_lines;

pub fn main() {
    let lines: Vec<String> = read_lines("input/day3")
        .unwrap()
        .filter_map(|l| l.ok())
        .collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (gamma, epsilon) = compute_gamma_epsilon(&lines);
    println!("Solution day 3-1 {}", gamma * epsilon);

    let (num_chars, ints) = parse_input(&lines[..]);
    let (oxygen, co2) = (
        compute_oxygen(&ints[..], num_chars),
        compute_co2_scrubber(&ints[..], num_chars),
    );
    println!("Solution day 3-2 {}", oxygen * co2);
}

fn compute_oxygen(inputs: &[u32], num_chars: usize) -> u32 {
    compute_diagnostic(inputs, true, num_chars)
}

fn compute_co2_scrubber(inputs: &[u32], num_chars: usize) -> u32 {
    compute_diagnostic(inputs, false, num_chars)
}

fn count_ones_at(inputs: &[u32], bit: usize) -> u32 {
    inputs
        .iter()
        .filter(|val| get_bit_at(**val, bit) != 0)
        .count() as u32
}

fn compute_diagnostic(inputs: &[u32], ones_and_common: bool, num_chars: usize) -> u32 {
    let mut inputs = Vec::from(inputs);
    let mut index = num_chars - 1;
    while inputs.len() > 1 {
        let ones = count_ones_at(&inputs, index);
        let zeroes = inputs.len() as u32 - ones;
        let keep_value = match (ones, zeroes, ones_and_common) {
            (ones, zeroes, ones_and_common) if ones > zeroes => ones_and_common,
            (ones, zeroes, ones_and_common) if ones < zeroes => !ones_and_common,
            (_, _, ones_and_common) => ones_and_common,
        } as u32;

        inputs.retain(|line| {
            let bit_at = get_bit_at(*line, index);
            let retained = bit_at == keep_value;
            retained
        });
        if index > 0 {
            index -= 1;
        }
    }
    inputs[0]
}

fn compute_gamma_epsilon(input: &[&str]) -> (u32, u32) {
    let (num_chars, ints) = parse_input(input);

    let mut counts = vec![0u32; num_chars];
    for number in ints {
        for i in 0..num_chars {
            counts[i] += get_bit_at(number, num_chars - 1 - i);
        }
    }
    let mut index = (num_chars - 1) as u32;
    let mut gamma = 0u32;
    let threshold: u32 = (input.len() / 2) as u32;
    for count in counts.iter() {
        let bit = if count > &threshold { 1 } else { 0 };
        gamma |= bit << index;
        if index > 0 {
            index -= 1;
        }
    }
    let shifts = num_chars;
    let mask = !((!0u32) >> shifts << shifts);
    (gamma, (!gamma & mask))
}

fn parse_input(input: &[&str]) -> (usize, Vec<u32>) {
    let num_chars = input[0].len();
    assert!(input.iter().all(|line| line.len() == num_chars));
    let ints: Vec<u32> = input
        .iter()
        .filter_map(|s| u32::from_str_radix(s, 2).ok())
        .collect();
    (num_chars, ints)
}

fn get_bit_at(input: u32, n: usize) -> u32 {
    if n < 32 {
        (input & (1 << n) != 0) as u32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bits() {
        let input = u32::from_str_radix("1001", 2).unwrap();
        assert_eq!(1, get_bit_at(input, 0));
        assert_eq!(0, get_bit_at(input, 1));
        assert_eq!(0, get_bit_at(input, 2));
        assert_eq!(1, get_bit_at(input, 3));
    }

    #[test]
    fn part_one() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let expected_gamma = u32::from_str_radix("10110", 2).unwrap();
        let expected_epsilon = u32::from_str_radix("01001", 2).unwrap();

        let (gamma, epsilon) = compute_gamma_epsilon(&input[..]);
        assert_eq!(gamma, expected_gamma);
        assert_eq!(epsilon, expected_epsilon);
    }

    #[test]
    fn part_two() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let expected_oxygen_rating = 23;
        let expected_co2_scrubber_rating = 10;
        let (num_chars, ints) = parse_input(&input[..]);
        assert_eq!(
            expected_co2_scrubber_rating,
            compute_co2_scrubber(&ints[..], num_chars)
        );
        assert_eq!(expected_oxygen_rating, compute_oxygen(&ints[..], num_chars));
    }
}
