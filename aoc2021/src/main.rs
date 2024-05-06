use std::{env, time::Instant};

use aoc::runner::{run_solution, Runner};

mod aoc202101;
mod aoc202102;

#[cfg(test)]
mod tests;

fn main() {
    let mut day01 = aoc202101::AocDay::new("aoc2021/inputs/day01.txt");
    let mut day02 = aoc202102::AocDay::new("aoc2021/inputs/day02.txt");
    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02];
    let len = days.len() - 1;
    match get_args() {
        Some(0) => {
            // Run all days
            let start = Instant::now();
            for selected in days.iter_mut() {
                run_solution(*selected);
            }
            let duration = start.elapsed().as_millis();
            let millis = duration % 1000;
            let seconds = duration / 1000;
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            println!("\nTotal: {minutes:3}:{seconds:02}.{millis:03}");
        }
        Some(d) => {
            // Run selected day
            let selected = &mut days[(d - 1).min(len)];
            run_solution(*selected);
        }
        None => {
            // Run last day
            let selected = &mut days[len];
            run_solution(*selected);
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
