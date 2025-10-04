use crate::day::Solution;

mod day;
mod day1;
mod day2;

fn print_solutions(sols: Vec<&dyn Solution>) {
    for sol in sols {
        println!("day '{}' part 1 result: {}", sol.get_day(), sol.result_p1());
        println!("day '{}' part 2 result: {}", sol.get_day(), sol.result_p2());
    }
}

fn main() {
    print_solutions(vec![&day1::Day1::new(), &day2::Day2::new()]);
}
