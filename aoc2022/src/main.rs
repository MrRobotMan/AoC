use std::{env, time::Instant};

use aoc::runner::{run_solution, Runner};

mod aoc202201;
mod aoc202202;
mod aoc202203;
mod aoc202204;
mod aoc202205;
mod aoc202206;
mod aoc202207;
mod aoc202208;
mod aoc202209;
mod aoc202210;
mod aoc202211;
mod aoc202212;
mod aoc202213;
mod aoc202214;
mod aoc202215;
mod aoc202216;
mod aoc202217;
mod aoc202218;
mod aoc202219;
mod aoc202220;
mod aoc202221;
mod aoc202222;
mod aoc202223;
mod aoc202224;
mod aoc202225;

fn main() {
    let mut day01 = aoc202201::AocDay::new("aoc2022/inputs/day01.txt");
    let mut day02 = aoc202202::AocDay::new("aoc2022/inputs/day02.txt");
    let mut day03 = aoc202203::AocDay::new("aoc2022/inputs/day03.txt");
    let mut day04 = aoc202204::AocDay::new("aoc2022/inputs/day04.txt");
    let mut day05 = aoc202205::AocDay::new("aoc2022/inputs/day05.txt");
    let mut day06 = aoc202206::AocDay::new("aoc2022/inputs/day06.txt");
    let mut day07 = aoc202207::AocDay::new("aoc2022/inputs/day07.txt");
    let mut day08 = aoc202208::AocDay::new("aoc2022/inputs/day08.txt");
    let mut day09 = aoc202209::AocDay::new("aoc2022/inputs/day09.txt");
    let mut day10 = aoc202210::AocDay::new("aoc2022/inputs/day10.txt");
    let mut day11 = aoc202211::AocDay::new("aoc2022/inputs/day11.txt");
    let mut day12 = aoc202212::AocDay::new("aoc2022/inputs/day12.txt");
    let mut day13 = aoc202213::AocDay::new("aoc2022/inputs/day13.txt");
    let mut day14 = aoc202214::AocDay::new("aoc2022/inputs/day14.txt");
    let mut day15 = aoc202215::AocDay::new("aoc2022/inputs/day15.txt");
    let mut day16 = aoc202216::AocDay::new("aoc2022/inputs/day16.txt");
    let mut day17 = aoc202217::AocDay::new("aoc2022/inputs/day17.txt");
    let mut day18 = aoc202218::AocDay::new("aoc2022/inputs/day18.txt");
    let mut day19 = aoc202219::AocDay::new("aoc2022/inputs/day19.txt");
    let mut day20 = aoc202220::AocDay::new("aoc2022/inputs/day20.txt");
    let mut day21 = aoc202221::AocDay::new("aoc2022/inputs/day21.txt");
    let mut day22 = aoc202222::AocDay::new("aoc2022/inputs/day22.txt");
    let mut day23 = aoc202223::AocDay::new("aoc2022/inputs/day23.txt");
    let mut day24 = aoc202224::AocDay::new("aoc2022/inputs/day24.txt");
    let mut day25 = aoc202225::AocDay::new("aoc2022/inputs/day25.txt");
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
