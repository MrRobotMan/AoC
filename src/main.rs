mod aoc2023;
use aoc2023::*;

use aoc::Selector;

fn main() {
    let runners: Vec<fn(Selector)> = vec![run_aoc_2023];
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 && args[1] == "all" {
        for year in runners {
            year(Selector::All);
        }
    } else if args.len() > 2 {
        let year = if let Ok(year) = args[1].parse::<usize>() {
            year
        } else {
            eprintln!("Invalid year {}", args[1]);
            std::process::exit(1);
        };

        let day = if let Ok(day) = args[2].parse::<usize>() {
            day
        } else {
            eprintln!("Invalid year {}", args[1]);
            std::process::exit(1);
        };

        if !(2023..=2023).contains(&year) {
            // if year < 2015 || year > 2022 {
            eprintln!("Year must be in range 2015..2022");
            std::process::exit(1);
        }

        runners[year - 2023](Selector::One(day));
    } else {
        runners.last().unwrap()(Selector::Last);
    }
}
