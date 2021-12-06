use crate::reader::{parse_ints, read_lines};

pub fn main() {
    let input: Vec<String> = read_lines("input/day4")
        .unwrap()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .collect();
    let input: Vec<&str> = input.iter().map(AsRef::as_ref).collect();
    let mut game = Game::new(input);
    println!("Day 4-1 result: {}", game.run_game(true));
    println!("Day 4-2 result: {}", game.run_game(false));
}

struct Field {
    pub marked: bool,
    pub value: u32,
}

struct Board {
    pub fields: Vec<Field>,
    pub finished: bool,
}

impl Board {
    pub fn new(input: &[u32]) -> Self {
        assert_eq!(input.len(), 25);
        Self {
            fields: input
                .iter()
                .map(|i| Field {
                    marked: false,
                    value: *i,
                })
                .collect(),
            finished: false,
        }
    }

    fn row_iter(&self, row: u8) -> Box<dyn Iterator<Item = &Field> + '_> {
        Box::new(self.fields.iter().skip(row as usize * 5).take(5))
    }

    fn col_iter(&self, col: u8) -> Box<dyn Iterator<Item = &Field> + '_> {
        Box::new(self.fields.iter().skip(col as usize).step_by(5))
    }

    pub fn run_number(&mut self, next_number: u32) {
        if let Some(field) = self
            .fields
            .iter_mut()
            .filter(|f| f.value == next_number)
            .last()
        {
            field.marked = true;
        }
        let is_finished = (0..5).any(|row| self.row_iter(row).all(|f| f.marked))
            || (0..5).any(|col| self.col_iter(col).all(|f| f.marked));
        self.finished = is_finished;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

struct Game {
    pub numbers: Vec<u32>,
    pub boards: Vec<Board>,
}

impl Game {
    pub fn new(input: Vec<&str>) -> Self {
        let numbers = parse_ints(input[0], ",");
        let num_boards = ((input.len()) - 1) / 5;
        let boards: Vec<Board> = (0..num_boards)
            .map(|board| {
                let all_board = input
                    .iter()
                    .skip((1 + board * 5) as usize)
                    .take(5)
                    .map(|i| String::from(*i))
                    .collect::<Vec<String>>()
                    .join(" ");
                Board::new(&parse_ints(&all_board, " "))
            })
            .collect();
        Self { numbers, boards }
    }

    pub fn run_game(&mut self, finish_first: bool) -> u32 {
        for number in &self.numbers {
            let mut remaining = self.boards.iter().filter(|b| !b.is_finished()).count();
            for board in &mut self.boards {
                if board.finished {
                    continue;
                }
                board.run_number(*number);
                let finished = board.is_finished();
                if finished {
                    remaining -= 1;
                }
                if (finish_first || remaining == 0) && finished {
                    return board
                        .fields
                        .iter()
                        .filter(|f| !f.marked)
                        .map(|f| f.value)
                        .sum::<u32>()
                        * number;
                }
            }
        }
        panic!("Game didn't finish!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board() {
        #[rustfmt::skip]
            let input = vec![
            22u32, 13, 17, 11, 0,
            8, 2, 23, 4, 24,
            21, 9, 14, 16, 7,
            6, 10, 3, 18, 5,
            1, 12, 20, 15, 19,
        ];
        let mut board = Board::new(&input);
        assert_eq!(
            board.row_iter(0).map(|f| f.value).collect::<Vec<_>>(),
            vec![22u32, 13, 17, 11, 0]
        );
        assert_eq!(
            board.row_iter(4).map(|f| f.value).collect::<Vec<_>>(),
            vec![1u32, 12, 20, 15, 19]
        );
        assert_eq!(
            board.col_iter(2).map(|f| f.value).collect::<Vec<_>>(),
            vec![17u32, 23, 14, 3, 20]
        );

        board.run_number(17);
        assert_eq!(false, board.is_finished());
        board.run_number(23);
        assert_eq!(false, board.is_finished());
        board.run_number(14);
        assert_eq!(false, board.is_finished());
        board.run_number(3);
        assert_eq!(false, board.is_finished());
        board.run_number(20);
        assert_eq!(false, board.is_finished());
    }

    #[test]
    fn part_one() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "3 15  0  2 22",
            "9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ];

        let mut game = Game::new(input);
        assert_eq!(27, game.numbers.len());
        assert_eq!(3, game.boards.len());
        assert_eq!(4512, game.run_game(true));
        assert_eq!(1924, game.run_game(false));
    }
}
