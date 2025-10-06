use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
};

use crate::day::Solution;

pub struct Day1 {
    name: &'static str,
}

impl Day1 {
    pub fn new() -> Self {
        Self { name: "one" }
    }
}

impl Solution for Day1 {
    fn get_day(&self) -> &'static str {
        self.name
    }
    fn result_p1(&self) -> String {
        let file = fs::File::open("src/day1/input_p1.txt").expect("oops0");

        let reader = BufReader::new(file);

        let mut list1: Vec<i32> = vec![];
        let mut list2: Vec<i32> = vec![];

        for line in reader.lines() {
            let line = line.expect("oops3");
            let split = line.split_once("   ").expect("oops");
            list1.push(split.0.parse().expect("oops2"));
            list2.push(split.1.parse().expect("oops2"));
        }

        list1.sort();
        list2.sort();

        let mut sum = 0;

        for (idx, num) in list1.iter().enumerate() {
            sum += (num - list2[idx]).abs()
        }

        sum.to_string()
    }

    fn result_p2(&self) -> String {
        let file = fs::File::open("src/day1/input_p2.txt").expect("oops0");

        let reader = BufReader::new(file);

        let mut appearance_counts: HashMap<i32, i32> = HashMap::new();
        let mut list1: Vec<i32> = vec![];
        let mut list2: Vec<i32> = vec![];

        for line in reader.lines() {
            let line = line.expect("oops3");
            let split = line.split_once("   ").expect("oops");
            list1.push(split.0.parse().expect("oops2"));
            list2.push(split.1.parse().expect("oops2"));
        }

        for num in list2 {
            *appearance_counts.entry(num).or_insert(0) += 1;
        }

        let mut sum = 0;

        for num in list1 {
            if let Some(count) = appearance_counts.get(&num) {
                sum += num * count
            }
        }

        sum.to_string()
    }
}
