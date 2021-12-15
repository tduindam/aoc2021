pub fn neighbors(
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

pub fn neighbors_straight(
    (x, y): (i32, i32),
    pos_type: PosType,
) -> Box<dyn Iterator<Item = (i32, i32)> + 'static> {
    let neighbors = match pos_type {
        PosType::Gen => GEN_NEIGHBORS_S.iter(),
        PosType::L => L_NEIGHBORS_S.iter(),
        PosType::R => R_NEIGHBORS_S.iter(),
        PosType::LT => LT_NEIGHBORS_S.iter(),
        PosType::RT => RT_NEIGHBORS_S.iter(),
        PosType::LB => LB_NEIGHBORS_S.iter(),
        PosType::RB => RB_NEIGHBORS_S.iter(),
        PosType::B => B_NEIGHBORS_S.iter(),
        PosType::T => T_NEIGHBORS_S.iter(),
    };
    Box::new(neighbors.map(move |(o_x, o_y)| (x + *o_x as i32, y + *o_y as i32)))
}

pub enum PosType {
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
    pub fn from_index(index: u32, (row_size, col_size): (u32, u32)) -> ((i32, i32), Self) {
        let x = (index % row_size) as i32;
        let y = (index / row_size) as i32;
        ((x, y), Self::from((x, y), (row_size, col_size)))
    }
    pub fn from(pos: (i32, i32), (row_size, col_size): (u32, u32)) -> Self {
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

///YES DIAGONALS
pub const GEN_NEIGHBORS: [(i8, i8); 8] = [
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

///NO DIAGONALS

pub const GEN_NEIGHBORS_S: [(i8, i8); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

const R_NEIGHBORS_S: [(i8, i8); 3] = [(0, -1), (-1, 0), (0, 1)];
const L_NEIGHBORS_S: [(i8, i8); 3] = [(0, -1), (1, 0), (0, 1)];
const T_NEIGHBORS_S: [(i8, i8); 3] = [(-1, 0), (1, 0), (0, 1)];
const B_NEIGHBORS_S: [(i8, i8); 3] = [(-1, 0), (1, 0), (0, -1)];
const LT_NEIGHBORS_S: [(i8, i8); 2] = [(1, 0), (0, 1)];
const RT_NEIGHBORS_S: [(i8, i8); 2] = [(-1, 0), (0, 1)];
const RB_NEIGHBORS_S: [(i8, i8); 2] = [(0, -1), (-1, 0)];
const LB_NEIGHBORS_S: [(i8, i8); 2] = [(0, -1), (1, 0)];
