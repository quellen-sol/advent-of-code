use std::collections::HashSet;

use crate::{
    defs::Solution,
    utils::{constants::VOWELS, strings::StringChunksIter},
};

pub struct Day5Solution {
    input: String,
}

impl Solution<usize, usize> for Day5Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> usize {
        self.get_lines()
            .filter(|l| Self::string_good_p1(*l))
            .count()
    }

    fn get_part_2_solution(&self) -> usize {
        self.get_lines()
            .filter(|l| Self::string_good_p2(*l))
            .count()
    }
}

impl Day5Solution {
    pub fn string_good_p1(s: &str) -> bool {
        let mut num_vowels = 0;
        let mut has_three_vowels = false;
        let mut has_double = false;
        let mut has_banned_strings = false;
        let mut last_char: Option<char> = None;
        for c in s.chars() {
            if !has_three_vowels && VOWELS.contains(&c.to_string().as_str()) {
                num_vowels += 1;
                if num_vowels >= 3 {
                    has_three_vowels = true;
                }
            }

            if let Some(last) = last_char {
                if !has_double && last == c {
                    has_double = true
                }

                if !has_banned_strings {
                    match last {
                        'a' if c == 'b' => {
                            has_banned_strings = true;
                        }
                        'c' if c == 'd' => {
                            has_banned_strings = true;
                        }
                        'p' if c == 'q' => {
                            has_banned_strings = true;
                        }
                        'x' if c == 'y' => {
                            has_banned_strings = true;
                        }
                        _ => {}
                    }
                }
            }

            last_char.replace(c);
        }

        has_three_vowels && has_double && !has_banned_strings
    }

    pub fn string_good_p2(s: &str) -> bool {
        false
        // let mut pairs = HashSet::new();
        // let mut has_repeated_pair = false;
        // let mut has_divided_pair = false;
        // let mut second_last_char = None;
        // let mut last_char = None;

        // for window in StringWindowsIter::new(s, 3) {

        // }

        // has_repeated_pair && has_divided_pair
    }
}
