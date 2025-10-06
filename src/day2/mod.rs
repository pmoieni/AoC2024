use std::{
    fs,
    io::{BufRead, BufReader},
};

use crate::day::Solution;

pub struct Day2 {
    name: &'static str,
}

impl Day2 {
    pub fn new() -> Self {
        Self { name: "two" }
    }
}

enum State {
    Increasing,
    Decreasing,
    Neutral,
}

impl Solution for Day2 {
    fn get_day(&self) -> &'static str {
        self.name
    }
    fn result_p1(&self) -> String {
        let file = fs::File::open("src/day2/input_p1.txt").expect("file didn't open");

        let reader = BufReader::new(file);
        let mut safe_count = 0;

        for line in reader.lines() {
            let mut state = State::Neutral;
            let mut last_digit = None;
            let mut is_safe = true;
            let line = line.expect("no line");

            for digit in line.split_whitespace() {
                let digit: i32 = digit.parse().expect("couldn't parse number");
                let Some(last_digit_copy) = last_digit else {
                    last_digit = Some(digit);
                    continue;
                };

                state = match (last_digit_copy - digit, &state) {
                    (1..=3, State::Neutral | State::Decreasing) => State::Decreasing,
                    (-3..=-1, State::Neutral | State::Increasing) => State::Increasing,
                    _ => {
                        is_safe = false;
                        break;
                    }
                };

                last_digit = Some(digit);
            }

            if is_safe {
                safe_count += 1;
            }
        }

        safe_count.to_string()
    }

    fn result_p2(&self) -> String {
        String::new()
    }
}

pub fn is_safe(line: &str) -> bool {
    #[derive(Debug, Clone, Copy)]
    enum State {
        Initial,
        ParsedFirstNumber(i64),
        Increasing { prev: i64 },
        Decreasing { prev: i64 },
    }

    struct LineIsUnsafe;

    line.split_whitespace()
        .try_fold(State::Initial, |state, num| {
            let num = num.parse::<i64>().unwrap_or_else(|err| {
                unreachable!("invalid number in line: {err}");
            });

            match state {
                State::Initial => Ok(State::ParsedFirstNumber(num)),
                State::ParsedFirstNumber(first_num) => match first_num - num {
                    -3..=-1 => Ok(State::Increasing { prev: num }),
                    1..=3 => Ok(State::Decreasing { prev: num }),
                    _ => Err(LineIsUnsafe),
                },
                State::Increasing { prev } => match prev - num {
                    -3..=-1 => Ok(State::Increasing { prev: num }),
                    _ => Err(LineIsUnsafe),
                },
                State::Decreasing { prev } => match prev - num {
                    1..=3 => Ok(State::Decreasing { prev: num }),
                    _ => Err(LineIsUnsafe),
                },
            }
        })
        .is_ok()
}
