fn compute_gamma_epsilon(input: &[&str]) -> (u32, u32) {
    let num_chars = input[0].len();
    assert!(input.iter().all(|line| line.len() == num_chars));
    let ints = input
        .iter()
        .filter_map(|s| u32::from_str_radix(s, 2).ok())
        .inspect(|v| println!("int: {}", v));

    let mut counts = vec![0u32; num_chars];
    for number in ints {
        for i in 0..num_chars {
            counts[i] += get_bit_at(number, i);
        }
    }
    let mut index = 0u32;
    let mut gamma = 0u32;
    let threshold: u32 = (input.len() / 2) as u32;
    for count in counts.iter().rev() {
        let bit = if count > &threshold { 1 } else { 0 };
        gamma |= bit << index;
        index += 1;
    }
    (gamma, !gamma)
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

        let expected_power_consumption = 198u32;

        let (gamma, epsilon) = compute_gamma_epsilon(&input[..]);
        assert_eq!(gamma, expected_gamma);
        assert_eq!(epsilon, expected_epsilon);
        // let power_consumption = compute_power_consuption(gamma, epsilon);
        // assert_eq(power_consumption);
    }
}
