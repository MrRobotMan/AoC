use aoc::runner::{Runner, run_solution};
use std::env;
use std::time::Instant;

mod aoc202301;
mod aoc202302;
mod aoc202303;
mod aoc202304;
mod aoc202305;
mod aoc202306;
mod aoc202307;
mod aoc202308;
mod aoc202309;
mod aoc202310;
mod aoc202311;
mod aoc202312;
mod aoc202313;
mod aoc202314;
mod aoc202315;
mod aoc202316;
mod aoc202317;
mod aoc202318;
mod aoc202319;
mod aoc202320;
mod aoc202321;
mod aoc202322;
mod aoc202323;
mod aoc202324;
mod aoc202325;

fn main() {
    let mut day01 = aoc202301::AocDay::new("aoc2023/inputs/day01.txt");
    let mut day02 = aoc202302::AocDay::new("aoc2023/inputs/day02.txt");
    let mut day03 = aoc202303::AocDay::new("aoc2023/inputs/day03.txt");
    let mut day04 = aoc202304::AocDay::new("aoc2023/inputs/day04.txt");
    let mut day05 = aoc202305::AocDay::new("aoc2023/inputs/day05.txt");
    let mut day06 = aoc202306::AocDay::new("aoc2023/inputs/day06.txt");
    let mut day07 = aoc202307::AocDay::new("aoc2023/inputs/day07.txt");
    let mut day08 = aoc202308::AocDay::new("aoc2023/inputs/day08.txt");
    let mut day09 = aoc202309::AocDay::new("aoc2023/inputs/day09.txt");
    let mut day10 = aoc202310::AocDay::new("aoc2023/inputs/day10.txt");
    let mut day11 = aoc202311::AocDay::new("aoc2023/inputs/day11.txt");
    let mut day12 = aoc202312::AocDay::new("aoc2023/inputs/day12.txt");
    let mut day13 = aoc202313::AocDay::new("aoc2023/inputs/day13.txt");
    let mut day14 = aoc202314::AocDay::new("aoc2023/inputs/day14.txt");
    let mut day15 = aoc202315::AocDay::new("aoc2023/inputs/day15.txt");
    let mut day16 = aoc202316::AocDay::new("aoc2023/inputs/day16.txt");
    let mut day17 = aoc202317::AocDay::new("aoc2023/inputs/day17.txt");
    let mut day18 = aoc202318::AocDay::new("aoc2023/inputs/day18.txt");
    let mut day19 = aoc202319::AocDay::new("aoc2023/inputs/day19.txt");
    let mut day20 = aoc202320::AocDay::new("aoc2023/inputs/day20.txt");
    let mut day21 = aoc202321::AocDay::new("aoc2023/inputs/day21.txt");
    let mut day22 = aoc202322::AocDay::new("aoc2023/inputs/day22.txt");
    let mut day23 = aoc202323::AocDay::new("aoc2023/inputs/day23.txt");
    let mut day24 = aoc202324::AocDay::new("aoc2023/inputs/day24.txt");
    let mut day25 = aoc202325::AocDay::new("aoc2023/inputs/day25.txt");
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
