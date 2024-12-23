use std::{env, time::Instant};

use aoc::runner::{run_solution, Runner};

mod aoc202401;
mod aoc202402;
mod aoc202403;
mod aoc202404;
mod aoc202405;
mod aoc202406;
mod aoc202407;
mod aoc202408;
mod aoc202409;
mod aoc202410;
mod aoc202411;
mod aoc202412;
mod aoc202413;
mod aoc202414;
mod aoc202415;
mod aoc202416;
mod aoc202417;
mod aoc202418;
mod aoc202419;
mod aoc202420;
mod aoc202421;
mod aoc202422;
mod aoc202423;

fn main() {
    let mut day01 = aoc202401::AocDay::new("aoc2024/inputs/day01.txt");
    let mut day02 = aoc202402::AocDay::new("aoc2024/inputs/day02.txt");
    let mut day03 = aoc202403::AocDay::new("aoc2024/inputs/day03.txt");
    let mut day04 = aoc202404::AocDay::new("aoc2024/inputs/day04.txt");
    let mut day05 = aoc202405::AocDay::new("aoc2024/inputs/day05.txt");
    let mut day06 = aoc202406::AocDay::new("aoc2024/inputs/day06.txt");
    let mut day07 = aoc202407::AocDay::new("aoc2024/inputs/day07.txt");
    let mut day08 = aoc202408::AocDay::new("aoc2024/inputs/day08.txt");
    let mut day09 = aoc202409::AocDay::new("aoc2024/inputs/day09.txt");
    let mut day10 = aoc202410::AocDay::new("aoc2024/inputs/day10.txt");
    let mut day11 = aoc202411::AocDay::new("aoc2024/inputs/day11.txt");
    let mut day12 = aoc202412::AocDay::new("aoc2024/inputs/day12.txt");
    let mut day13 = aoc202413::AocDay::new("aoc2024/inputs/day13.txt");
    let mut day14 = aoc202414::AocDay::new("aoc2024/inputs/day14.txt");
    let mut day15 = aoc202415::AocDay::new("aoc2024/inputs/day15.txt");
    let mut day16 = aoc202416::AocDay::new("aoc2024/inputs/day16.txt");
    let mut day17 = aoc202417::AocDay::new("aoc2024/inputs/day17.txt");
    let mut day18 = aoc202418::AocDay::new("aoc2024/inputs/day18.txt");
    let mut day19 = aoc202419::AocDay::new("aoc2024/inputs/day19.txt");
    let mut day20 = aoc202420::AocDay::new("aoc2024/inputs/day20.txt");
    let mut day21 = aoc202421::AocDay::new("aoc2024/inputs/day21.txt");
    let mut day22 = aoc202422::AocDay::new("aoc2024/inputs/day22.txt");
    let mut day23 = aoc202423::AocDay::new("aoc2024/inputs/day23.txt");
    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07, 
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14, 
        &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21, 
        &mut day22, &mut day23, 
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
