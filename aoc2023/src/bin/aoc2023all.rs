use std::{io, process::Output, time::Instant};

fn main() {
    build_days();
    let start = Instant::now();
    run_days();
    let duration = start.elapsed().as_millis();
    println!("\nTotal: {:3}.{:03}", duration / 1000, duration % 1000);
}

fn build_days() {
    for day in 1..=25 {
        print!("Building day {day:02}..");
        println!(
            "{}",
            match cmd("build", day) {
                Ok(_) => "Success",
                Err(_) => "Fail",
            }
        );
    }
}

fn run_days() {
    for day in 1..=25 {
        println!(
            "{}",
            match cmd("run", day) {
                Ok(o) => o.stdout.iter().map(|c| char::from(*c)).collect::<String>(),
                Err(e) => e.to_string(),
            }
        )
    }
}

fn cmd(cmd: &str, day: usize) -> io::Result<Output> {
    std::process::Command::new("pwsh")
        .args([
            "-command",
            "cargo",
            cmd,
            "--release",
            "--bin",
            &format!("aoc2023{day:02}"),
        ])
        .output()
}
