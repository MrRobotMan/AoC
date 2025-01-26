use std::{env, time::Instant};

use aoc::runner::{run_solution, Runner};

mod aoc202101;
mod aoc202102;
mod aoc202103;
mod aoc202104;
mod aoc202105;
mod aoc202106;
mod aoc202107;
mod aoc202108;
mod aoc202109;
mod aoc202110;
mod aoc202111;
mod aoc202112;

#[cfg(test)]
mod tests;

fn main() {
    let mut day01 = aoc202101::AocDay::new("aoc2021/inputs/day01.txt");
    let mut day02 = aoc202102::AocDay::new("aoc2021/inputs/day02.txt");
    let mut day03 = aoc202103::AocDay::new("aoc2021/inputs/day03.txt");
    let mut day04 = aoc202104::AocDay::new("aoc2021/inputs/day04.txt");
    let mut day05 = aoc202105::AocDay::new("aoc2021/inputs/day05.txt");
    let mut day06 = aoc202106::AocDay::new("aoc2021/inputs/day06.txt");
    let mut day07 = aoc202107::AocDay::new("aoc2021/inputs/day07.txt");
    let mut day08 = aoc202108::AocDay::new("aoc2021/inputs/day08.txt");
    let mut day09 = aoc202109::AocDay::new("aoc2021/inputs/day09.txt");
    let mut day10 = aoc202110::AocDay::new("aoc2021/inputs/day10.txt");
    let mut day11 = aoc202111::AocDay::new("aoc2021/inputs/day11.txt");
    let mut day12 = aoc202112::AocDay::new("aoc2021/inputs/day12.txt");
    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07, 
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, 
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
