pub fn main() {
    let input = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2";
    let input = parse_input(input);
    println!("Day 6-1 {}", run(input, 80).unwrap());

    let input = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2";
    let input = parse_input(input);
    println!("Day 6-2 {}", run(input, 256).unwrap());
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(",")
        .filter_map(|v| v.parse::<u32>().ok())
        .collect()
}

fn run(fishes: Vec<u32>, days: u32) -> Result<u64, Day6Err> {
    let mut histogram = [0u64; 9];
    for f in fishes {
        histogram[f as usize] += 1u64;
    }

    let mut t = 0u32;
    while t < days {
        let prev_state = histogram.clone();

        histogram[0] = prev_state[1];
        histogram[1] = prev_state[2];
        histogram[2] = prev_state[3];
        histogram[3] = prev_state[4];
        histogram[4] = prev_state[5];
        histogram[5] = prev_state[6];
        histogram[6] = prev_state[7] + prev_state[0];
        histogram[7] = prev_state[8];
        histogram[8] = prev_state[0];
        t += 1;
    }

    Ok(histogram.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "3,4,3,1,2";
        let input = parse_input(input);

        assert_eq!(26, run(input, 18).unwrap());
        // assert_eq!(26, run_bruteforce(input, 18).unwrap());

        let input = "3,4,3,1,2";
        let input: Vec<u32> = input
            .split(",")
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();
        assert_eq!(5934, run(input, 80).unwrap());

        let input = "3,4,3,1,2";
        let input: Vec<u32> = input
            .split(",")
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();
        assert_eq!(26984457539, run(input, 256).unwrap());
    }
}
