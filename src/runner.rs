use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn parse(&self);
    fn part1(&self) -> Vec<String>;
    fn part2(&self) -> Vec<String>;
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
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

fn print_solution(which: usize, output: &[String], duration: Duration) {
    let ms = duration.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;

    let mut i = output.iter();
    println!(
        "{sec_part:3}.{ms_part:03} Part {which}: {}",
        i.next().unwrap()
    );
    for line in i {
        println!("{:16}{line}", "");
    }
}
