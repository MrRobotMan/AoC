use aoc::{run_solution, Runner, Selector};

mod day01;
use day01::*;
pub fn run_aoc_2023(day: Selector) {
    let mut day01 = Aoc202301::new();
    let mut days: Vec<&mut dyn Runner> = vec![&mut day01];

    match day {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            run_solution(*d);
        }
        Selector::All => {
            for d in days {
                run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            run_solution(*d);
        }
    }
}
