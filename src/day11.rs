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

fn neighbors(
    (x, y): (i32, i32),
    pos_type: PosType,
) -> Box<dyn Iterator<Item = (i32, i32)> + 'static> {
    let neighbors = match pos_type {
        PosType::Gen => GEN_NEIGHBORS.iter(),
        PosType::L => L_NEIGHBORS.iter(),
        PosType::R => R_NEIGHBORS.iter(),
        PosType::LT => LT_NEIGHBORS.iter(),
        PosType::RT => RT_NEIGHBORS.iter(),
        PosType::LB => LB_NEIGHBORS.iter(),
        PosType::RB => RB_NEIGHBORS.iter(),
        PosType::B => B_NEIGHBORS.iter(),
        PosType::T => T_NEIGHBORS.iter(),
    };
    Box::new(neighbors.map(move |(o_x, o_y)| (x + *o_x as i32, y + *o_y as i32)))
}

enum PosType {
    Gen,
    L,
    R,
    LT,
    RT,
    LB,
    RB,
    B,
    T,
}

impl PosType {
    fn from_index(index: u32, (row_size, col_size): (u32, u32)) -> ((i32, i32), Self) {
        let x = (index % row_size) as i32;
        let y = (index / row_size) as i32;
        ((x, y), Self::from((x, y), (row_size, col_size)))
    }
    fn from(pos: (i32, i32), (row_size, col_size): (u32, u32)) -> Self {
        let max_x = (row_size - 1) as i32;
        let max_y = (col_size - 1) as i32;

        match pos {
            (0, 0) => PosType::LT,
            (x, 0) if x == max_x => PosType::RT,
            (_, 0) => PosType::T,
            (0, y) if y == max_y => PosType::LB,
            (x, y) if y == max_y && x == max_x => PosType::RB,
            (_, y) if y == max_y => PosType::B,
            (0, _) => PosType::L,
            (x, _) if x == max_x => PosType::R,
            (_, _) => PosType::Gen,
        }
    }
}

const GEN_NEIGHBORS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const R_NEIGHBORS: [(i8, i8); 5] = [(-1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
const L_NEIGHBORS: [(i8, i8); 5] = [(0, -1), (1, -1), (1, 0), (0, 1), (1, 1)];
const T_NEIGHBORS: [(i8, i8); 5] = [(-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
const B_NEIGHBORS: [(i8, i8); 5] = [(-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];
const LT_NEIGHBORS: [(i8, i8); 3] = [(1, 0), (0, 1), (1, 1)];
const RT_NEIGHBORS: [(i8, i8); 3] = [(-1, 0), (-1, 1), (0, 1)];
const RB_NEIGHBORS: [(i8, i8); 3] = [(-1, -1), (0, -1), (-1, 0)];
const LB_NEIGHBORS: [(i8, i8); 3] = [(0, -1), (1, -1), (1, 0)];

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
