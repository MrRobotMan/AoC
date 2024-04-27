use std::env;

use aoc::runner::run_solution;

mod aoc202101;

#[cfg(test)]
mod tests;

fn main() {
    let day01 = aoc202101::AocDay::new("inputs/day01.txt");
    let mut days = [day01];
    let len = days.len() - 1;
    match get_args() {
        Some(0) => {
            // Run all days
            for selected in days.iter_mut() {
                run_solution(selected);
            }
        }
        Some(d) => {
            // Run selected day
            let selected = &mut days[(d - 1).min(len)];
            run_solution(selected)
        }
        None => {
            // Run last day
            let selected = &mut days[len];
            run_solution(selected);
        }
    };
}

fn get_args() -> Option<usize> {
    let mut args = env::args();
    match args.len() {
        2 => {
            args.next();
            Some(args.next().unwrap().parse().unwrap())
        }
        _ => None,
    }
}
