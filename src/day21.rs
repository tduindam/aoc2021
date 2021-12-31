use itertools::{iproduct, Itertools};
use std::collections::HashMap;

pub fn main() {
    let (one_wins, two_wins) = simulate(4, 8, 21);
    println!("Day 21 - 2: 1 wins: {} 2 wins {}", one_wins, two_wins);
}

static THREE_ROLLS: [u8; 27] = [
    3, //111
    4, //112
    5, //113
    4, //121
    5, //122
    6, //123
    5, //131
    6, //132
    7, //133
    4, //211
    5, //212
    6, //213
    5, //221
    6, //222
    7, //223
    6, //231
    7, //232
    8, //233
    5, //311
    6, //312
    7, //313
    6, //321
    7, //322
    8, //323
    7, //331
    8, //332
    9, //333
];

#[derive(Debug)]
struct Die {
    num_rolls: u64,
}

impl Die {
    pub fn new() -> Self {
        Die { num_rolls: 0 }
    }

    fn practice_roll(&mut self) -> u64 {
        let next_roll = self.num_rolls % 100 + 1;
        self.num_rolls = self.num_rolls + 1u64;
        next_roll
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Player {
    pos: u8,
    score: u64,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct GameState {
    player_1: Player,
    player_2: Player,
}

fn simulate(p1_start: u64, p2_start: u64, final_score: u64) -> (u64, u64) {
    let all_rolls = THREE_ROLLS
        .iter()
        .cartesian_product(THREE_ROLLS.iter())
        .collect::<Vec<_>>();

    println!("all rolls {}", all_rolls.len());

    type GameMap = HashMap<GameState, u64>;
    let mut player_one_wins = 0u64;
    let mut player_two_wins = 0u64;

    let player_1 = Player {
        pos: p1_start as u8 - 1,
        score: 0,
    };
    let player_2 = Player {
        pos: p2_start as u8 - 1,
        score: 0,
    };

    let initial_state = GameState { player_1, player_2 };
    let mut games = GameMap::new();
    games.insert(initial_state, 1);
    while !games.is_empty() {
        let cur_games = games.clone().into_iter().collect::<Vec<_>>();
        // println!("Cur games {:?}", cur_games);
        games.clear();

        for (game, count) in cur_games.into_iter() {
            if game.player_1.score >= final_score {
                player_one_wins += count;
                continue;
            }
            if game.player_2.score >= final_score {
                player_two_wins += count;
                continue;
            }
            for (p1_move, p2_move) in all_rolls.iter() {
                let player_1 = move_player(game.player_1, **p1_move);
                let player_2 = move_player(game.player_2, **p2_move);
                let state = GameState { player_1, player_2 };
                *games.entry(state).or_insert(0) += count;
            }
        }
    }

    (player_one_wins, player_two_wins)
}

fn move_player(mut player: Player, dist: u8) -> Player {
    player.pos = (player.pos + dist) % 10;
    player.score += player.pos as u64 + 1;
    player
}

fn play_game(p1_start: u64, p2_start: u64) -> u64 {
    let mut player_1 = Player {
        pos: p1_start as u8 - 1,
        score: 0,
    };
    let mut player_2 = Player {
        pos: p2_start as u8 - 1,
        score: 0,
    };
    let mut die = Die::new();
    fn roll_three_times(die: &mut Die) -> u64 {
        let a = die.practice_roll();
        let b = die.practice_roll();
        let c = die.practice_roll();
        a + b + c
    }

    loop {
        let move_1 = roll_three_times(&mut die);
        player_1 = move_player(player_1, move_1 as u8);
        if player_1.score >= 1000 {
            return player_2.score as u64 * &die.num_rolls;
        }

        let move_2 = roll_three_times(&mut die);
        player_2 = move_player(player_2, move_2 as u8);
        if player_2.score >= 1000 {
            return player_1.score as u64 * &die.num_rolls;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_small() {
        let mut die = Die::new();
        let mut result = Vec::<u64>::new();
        for _ in 0..100 {
            result.push(die.practice_roll());
        }
        let expected: Vec<u64> = (1..=100).collect();
        assert_eq!(expected, result);

        assert_eq!(739785, play_game(4, 8));
    }

    #[test]
    fn part_one() {
        assert_eq!(518418, play_game(8, 1));
    }

    #[test]
    fn part_two_small() {
        let (one_wins, two_wins) = simulate(4, 8, 21);
        println!("One wins {} two wins {}", one_wins, two_wins);
        assert_eq!(444356092776315, one_wins);
        assert_eq!(341960390180808, two_wins);
    }

    #[test]
    fn part_two() {
        let (one_wins, two_wins) = simulate(8, 1, 21);
        println!("One wins {} two wins {}", one_wins, two_wins);
        assert_eq!(444356092776315, one_wins);
        assert_eq!(341960390180808, two_wins);
    }
}
