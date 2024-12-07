use std::{env, time::Instant};

use aoc::runner::{run_solution, Runner};

mod aoc202401;
mod aoc202402;
mod aoc202403;
mod aoc202404;
mod aoc202405;
mod aoc202406;
mod aoc202407;

fn main() {
    let mut day01 = aoc202401::AocDay::new("aoc2024/inputs/day01.txt");
    let mut day02 = aoc202402::AocDay::new("aoc2024/inputs/day02.txt");
    let mut day03 = aoc202403::AocDay::new("aoc2024/inputs/day03.txt");
    let mut day04 = aoc202404::AocDay::new("aoc2024/inputs/day04.txt");
    let mut day05 = aoc202405::AocDay::new("aoc2024/inputs/day05.txt");
    let mut day06 = aoc202406::AocDay::new("aoc2024/inputs/day06.txt");
    let mut day07 = aoc202407::AocDay::new("aoc2024/inputs/day07.txt");
    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07, 
    ];
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
