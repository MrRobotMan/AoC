use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn parse(&mut self);
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
}

pub fn output<T: Display>(output: T) -> String {
    format!("{}", output)
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T) {
    let (year, day) = solution.name();
    println!("---- {year}: {day:02} ----");

    let start = Instant::now();
    solution.parse();
    let end = start.elapsed().as_millis();
    println!("{:3}.{:03} Parsing", end / 1000, end % 1000);

    let start = Instant::now();
    let part1 = solution.part1();
    let end = start.elapsed();
    print_solution(1, &part1, end);

    let start = Instant::now();
    let part2 = solution.part2();
    let end = start.elapsed();
    print_solution(2, &part2, end);
}

fn print_solution(which: usize, output: &str, duration: Duration) {
    let ms = duration.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;

    println!("{sec_part:3}.{ms_part:03} Part {which}: {output}");
    // let mut i = output.iter();
    // println!(
    //     "{sec_part:3}.{ms_part:03} Part {which}: {}",
    //     i.next().unwrap()
    // );
    // for line in i {
    //     println!("{:16}{line}", "");
    // }
}
