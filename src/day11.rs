use std::cmp::max;

fn simulate(days: u32, mut grid: Vec<u32>, (row_size, col_size): (u32, u32)) -> u32 {
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
    for _ in 0..days {
        glow_queue.clear();
        //increment all elements in grid
        for index in 0..grid.len() {
            let count = grid.get_mut(index).unwrap();
            *count += 1;
            if *count == 9 {
                glow_queue.push(index);
            }
        }
        all_glows += glow_queue.len() as u32;
        while !glow_queue.is_empty() {
            let index = glow_queue.pop().unwrap();
            let value = grid.get_mut(index).unwrap();
            if *value == 9 {
                continue;
            }
            *value += 1;
            all_glows += 1;
            if *value == 9 {
                let neighbors = neighbor_map.get(index).unwrap();
                glow_queue.extend(neighbors);
            }
        }
        for index in 0..grid.len() {
            let count = grid.get_mut(index).unwrap();
            if *count == 9 {
                *count = 0;
            }
        }
    }
    all_glows
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
            (_, y) if y == max_y => PosType::RB,
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
const LB_NEIGHBORS: [(i8, i8); 3] = [(0, -1), (1, -1), (0, -0)];

#[cfg(test)]
mod test {
    use crate::day11::simulate;
    use crate::reader::parse_grid;

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
            9,
            simulate(1, parsed.clone(), (row_size as u32, col_size as u32))
        );

        assert_eq!(
            1656,
            simulate(100, parsed, (row_size as u32, col_size as u32))
        );
    }
}
