use crate::neighbors;
use crate::neighbors::{neighbors, PosType};
use crate::reader::parse_grid;

pub fn main() {
    println!("Day 11 - 1: {}", run_real_input_part_one());
    println!("Day 11 - 2: {}", run_real_input_part_two());
}

enum StopCriterium {
    Days(u32),
    Synchronized,
}

fn run_real_input_part_one() -> u32 {
    let input = "1172728874
    6751454281
    2612343533
    1884877511
    7574346247
    2117413745
    7766736517
    4331783444
    4841215828
    6857766273";
    let ((row_size, col_size), parsed) = parse_grid(input);
    let (glows, _) = simulate(
        StopCriterium::Days(100),
        parsed,
        (row_size as u32, col_size as u32),
    );
    glows
}

fn run_real_input_part_two() -> u32 {
    let input = "1172728874
    6751454281
    2612343533
    1884877511
    7574346247
    2117413745
    7766736517
    4331783444
    4841215828
    6857766273";
    let ((row_size, col_size), parsed) = parse_grid(input);
    let (_, days) = simulate(
        StopCriterium::Synchronized,
        parsed,
        (row_size as u32, col_size as u32),
    );
    days
}

fn simulate(
    stop: StopCriterium,
    mut grid: Vec<u32>,
    (row_size, col_size): (u32, u32),
) -> (u32, u32) {
    let neighbor_map: Vec<Vec<usize>> = (0..grid.len())
        .map(|i| PosType::from_index(i as u32, (row_size, col_size)))
        .map(|((x, y), t)| {
            neighbors((x, y), t)
                .map(|(x, y)| (x as usize + y as usize * row_size as usize))
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut all_glows = 0u32;
    let mut glow_queue = Vec::<usize>::new();
    let mut day = 0u32;
    loop {
        glow_queue.clear();
        let mut glow_this_day = 0u32;
        //increment all elements in grid
        for index in 0..grid.len() {
            let count = grid.get_mut(index).unwrap();
            *count += 1;
            if *count > 9 {
                glow_queue.push(index);
                let neighbors = neighbor_map.get(index).unwrap();
                glow_queue.extend(neighbors);
                glow_this_day += 1;
            }
        }

        while !glow_queue.is_empty() {
            let index = glow_queue.pop().unwrap();
            let value = grid.get_mut(index).unwrap();
            if *value > 9 {
                continue;
            }
            *value += 1;

            if *value > 9 {
                let neighbors = neighbor_map.get(index).unwrap();
                glow_queue.extend(neighbors);
                glow_this_day += 1;
            }
        }
        for index in 0..grid.len() {
            let count = grid.get_mut(index).unwrap();
            if *count > 9 {
                *count = 0;
            }
        }
        all_glows += glow_this_day;
        day += 1;
        match stop {
            StopCriterium::Days(criterium) => {
                if day >= criterium {
                    break;
                }
            }
            StopCriterium::Synchronized => {
                if glow_this_day == row_size * col_size {
                    break;
                }
            }
        }
    }
    (all_glows, day)
}

#[cfg(test)]
mod test {
    use crate::day11::{
        neighbors, run_real_input_part_one, run_real_input_part_two, simulate, PosType,
        StopCriterium,
    };
    use crate::reader::parse_grid;

    #[test]
    fn count_neighbors() {
        let expected: [u32; 9] = [3, 5, 3, 5, 8, 5, 3, 5, 3];
        let neighbors = (0u32..9)
            .map(|i| PosType::from_index(i, (3, 3)))
            .map(|(p, t)| neighbors(p, t))
            .flatten();

        let mut result: [u32; 9] = [0u32; 9];
        for (x, y) in neighbors {
            result[(x + y * 3) as usize] += 1;
        }

        assert_eq!(expected, result);
    }

    #[test]
    fn part_one_simple() {
        let input = "11111
19991
19191
19991
11111";
        let ((row_size, col_size), parsed) = parse_grid(input);
        let (glows, _) = simulate(
            StopCriterium::Days(1),
            parsed.clone(),
            (row_size as u32, col_size as u32),
        );
        assert_eq!(9, glows);
    }

    #[test]
    fn part_one() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let ((row_size, col_size), parsed) = parse_grid(input);
        assert_eq!(
            0,
            simulate(
                StopCriterium::Days(1),
                parsed.clone(),
                (row_size as u32, col_size as u32)
            )
            .0
        );

        assert_eq!(
            1656,
            simulate(
                StopCriterium::Days(100),
                parsed,
                (row_size as u32, col_size as u32)
            )
            .0
        );
    }

    #[test]
    fn part_one_real_input() {
        assert_eq!(1644, run_real_input_part_one());
    }
    #[test]
    fn part_two_real_input() {
        assert_eq!(229, run_real_input_part_two());
    }
}
