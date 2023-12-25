fn main() {
    for day in 1..=25 {
        let res = std::process::Command::new("pwsh")
            .args([
                "-command",
                "cargo",
                "run",
                "--release",
                "--bin",
                &format!("aoc2023{day:02}"),
            ])
            .output();
        println!(
            "{}",
            match res {
                Ok(o) => o.stdout.iter().map(|c| char::from(*c)).collect::<String>(),
                Err(e) => e.to_string(),
            }
        );
    }
}
