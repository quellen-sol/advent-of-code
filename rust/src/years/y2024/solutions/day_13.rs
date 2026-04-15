use regex::Regex;

use crate::defs::Solution;

pub struct Day13Solution {
    input: String,
}

impl Solution<Unit, Unit> for Day13Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> Unit {
        let games = self.make_games(false);
        let total_cost = games.iter().map(|g| g.find_min_tokens()).sum();

        total_cost
    }

    fn get_part_2_solution(&self) -> Unit {
        let games = self.make_games(true);
        let total_cost = games.iter().map(|g| g.find_min_tokens()).sum();

        total_cost
    }
}

impl Day13Solution {
    fn make_games(&self, is_p2: bool) -> Vec<Game> {
        let button_regex = Regex::new(r".*X[=+](\d+)\,\sY[=+](\d+)").unwrap();
        let game_s_iter = self.get_input().split('\n').enumerate();
        let mut games: Vec<Game> = Vec::new();
        let mut cur_game = Game::default();
        for (i, line) in game_s_iter {
            let info = if line.trim().len() > 0 {
                let caps = button_regex.captures(line).unwrap();
                let num_x = caps.get(1).unwrap().as_str().parse::<Unit>().unwrap();
                let num_y = caps.get(2).unwrap().as_str().parse::<Unit>().unwrap();

                Some((num_x, num_y))
            } else {
                None
            };
            match i % 4 {
                0 => {
                    // Button A info
                    let info = info.unwrap();
                    cur_game.a_adv_x = info.0;
                    cur_game.a_adv_y = info.1;
                }
                1 => {
                    // Button B info
                    let info = info.unwrap();
                    cur_game.b_adv_x = info.0;
                    cur_game.b_adv_y = info.1;
                }
                2 => {
                    // Prize info
                    let info = info.unwrap();
                    cur_game.prize_x = info.0 + if is_p2 { 10000000000000 } else { 0 };
                    cur_game.prize_y = info.1 + if is_p2 { 10000000000000 } else { 0 };
                }
                3 => {
                    // newline, make new game
                    games.push(cur_game);
                    cur_game = Game::default();
                }
                _ => {}
            }
        }

        // Push last game
        games.push(cur_game);

        games
    }
}

type Unit = u64;

#[derive(Default, Clone)]
struct Game {
    a_adv_x: Unit,
    a_adv_y: Unit,
    b_adv_x: Unit,
    b_adv_y: Unit,
    prize_x: Unit,
    prize_y: Unit,
}

impl Game {
    pub fn get_max_a(&self) -> Unit {
        let max_by_x = (self.prize_x / self.a_adv_x);
        let max_by_y = (self.prize_y / self.a_adv_y);
        max_by_x.min(max_by_y)
    }

    pub fn get_max_b(&self) -> Unit {
        let max_by_x = (self.prize_x / self.b_adv_x);
        let max_by_y = (self.prize_y / self.b_adv_y);
        max_by_x.min(max_by_y)
    }

    pub fn find_min_tokens(&self) -> Unit {
        let max_a = self.get_max_a();
        let max_b = self.get_max_b();

        let mut pairs = Vec::new();

        for c_a in 0..=max_a {
            for c_b in 0..=max_b {
                let total_x = c_a * self.a_adv_x + c_b * self.b_adv_x;
                if total_x != self.prize_x {
                    continue;
                }

                let total_y = c_a * self.a_adv_y + c_b * self.b_adv_y;
                if total_y != self.prize_y {
                    continue;
                }

                pairs.push((c_a, c_b))
            }
        }

        pairs.iter().map(|(a, b)| a * 3 + b).min().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn making_games() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";

        let solution = Day13Solution {
            input: input.into(),
        };
        let games = solution.make_games(false);

        assert_eq!(games.len(), 2);
        let first = games.first().unwrap();
        assert_eq!(first.a_adv_x, 94);
        assert_eq!(first.a_adv_y, 34);
        assert_eq!(first.b_adv_x, 22);
        assert_eq!(first.b_adv_y, 67);
        assert_eq!(first.prize_x, 8400);
        assert_eq!(first.prize_y, 5400);

        let min_cost = first.find_min_tokens();
        assert_eq!(min_cost, 280);
    }
}
