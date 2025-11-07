use std::{env, time::Instant};

use aoc::runner::{Runner, run_solution};

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
mod aoc202113;
mod aoc202114;
mod aoc202115;
mod aoc202116;
mod aoc202117;
mod aoc202118;
mod aoc202119;
mod aoc202120;
mod aoc202121;
mod aoc202122;
mod aoc202123;
mod aoc202124;
mod aoc202125;

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
    let mut day13 = aoc202113::AocDay::new("aoc2021/inputs/day13.txt");
    let mut day14 = aoc202114::AocDay::new("aoc2021/inputs/day14.txt");
    let mut day15 = aoc202115::AocDay::new("aoc2021/inputs/day15.txt");
    let mut day16 = aoc202116::AocDay::new("aoc2021/inputs/day16.txt");
    let mut day17 = aoc202117::AocDay::new("aoc2021/inputs/day17.txt");
    let mut day18 = aoc202118::AocDay::new("aoc2021/inputs/day18.txt");
    let mut day19 = aoc202119::AocDay::new("aoc2021/inputs/day19.txt");
    let mut day20 = aoc202120::AocDay::new("aoc2021/inputs/day20.txt");
    let mut day21 = aoc202121::AocDay::new("aoc2021/inputs/day21.txt");
    let mut day22 = aoc202122::AocDay::new("aoc2021/inputs/day22.txt");
    let mut day23 = aoc202123::AocDay::new("aoc2021/inputs/day23.txt");
    let mut day24 = aoc202124::AocDay::new("aoc2021/inputs/day24.txt");
    let mut day25 = aoc202125::AocDay::new("aoc2021/inputs/day25.txt");
    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
        &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
        &mut day22, &mut day23, &mut day24, &mut day25,
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
