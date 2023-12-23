use aoc::{
    common,
    io::{self, AOCError},
};
use std::str::FromStr;

#[derive(Debug)]
struct Grab {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Grab {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grab: Self = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        let grabs_str = io::tokenize(s, ", ");
        for grab_str in grabs_str {
            let tuple = io::tokenize(grab_str, " ");
            let q = io::parse_num::<usize>(tuple[0]);
            match tuple[1] {
                "red" => grab.red = q,
                "green" => grab.green = q,
                "blue" => grab.blue = q,
                _ => (),
            }
        }
        Ok(grab)
    }
}

#[derive(Debug)]
struct Game {
    game_id: usize,
    pub grabs: Vec<Grab>,
}

impl Game {
    fn max_seen(&self) -> [usize; 3] {
        let mut max: [usize; 3] = [0; 3];
        for grab in &self.grabs {
            max[0] = max[0].max(grab.red);
            max[1] = max[1].max(grab.green);
            max[2] = max[2].max(grab.blue);
        }
        max
    }

    fn is_good(&self, min_grab: &Grab) -> bool {
        for grab in &self.grabs {
            if grab.red > min_grab.red || grab.green > min_grab.green || grab.blue > min_grab.blue {
                return false;
            }
        }
        true
    }

    fn power(&self) -> usize {
        let max = self.max_seen();
        max[0] * max[1] * max[2]
    }
}

impl FromStr for Game {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = io::tokenize(s, ": ");
        let game_id = io::parse_num::<usize>(tokens[0]);
        let mut game: Game = Game {
            game_id,
            grabs: vec![],
        };
        let games_str = io::tokenize(tokens[1], "; ");
        for grab_str in games_str {
            game.grabs.push(Grab::from_str(grab_str).unwrap());
        }
        Ok(game)
    }
}

fn part2(games: &[Game]) -> usize {
    games.iter().map(|g| g.power()).sum::<usize>()
}

fn part1(games: &[Game]) -> usize {
    const MIN_GRAB: Grab = Grab {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter_map(|g| {
            if g.is_good(&MIN_GRAB) {
                Some(g.game_id)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn process_games_and_solve<const PART1: bool>(input: &str) -> usize {
    let mut games = Vec::new();
    for line in input.lines() {
        games.push(Game::from_str(line).unwrap());
    }

    if PART1 {
        return part1(&games);
    }
    part2(&games)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, process_games_and_solve::<true>, true);
    common::timed(&input, process_games_and_solve::<false>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process_games_and_solve::<true>(sample_input), 8);
        assert_eq!(process_games_and_solve::<false>(sample_input), 2286);
    }
}
