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

struct Player {
    pos: u64,
    score: u64,
}

//Returns Die, Winner, Loser
fn play_game(p1_start: u64, p2_start: u64) -> u64 {
    let mut player_1 = Player {
        pos: p1_start - 1,
        score: 0,
    };
    let mut player_2 = Player {
        pos: p2_start - 1,
        score: 0,
    };
    let mut die = Die::new();
    fn roll_three_times(die: &mut Die) -> u64 {
        let a = die.practice_roll();
        let b = die.practice_roll();
        let c = die.practice_roll();
        a + b + c
    }

    fn move_player(mut player: Player, dist: u64) -> Player {
        player.pos = (player.pos + dist) % 10;
        player.score += player.pos + 1;
        player
    }

    loop {
        let move_1 = roll_three_times(&mut die);
        player_1 = move_player(player_1, move_1);
        if player_1.score >= 1000 {
            return player_2.score * &die.num_rolls;
        }

        let move_2 = roll_three_times(&mut die);
        player_2 = move_player(player_2, move_2);
        if player_2.score >= 1000 {
            return player_1.score * &die.num_rolls;
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
    fn part_two() {
        assert!(false);
    }
}
