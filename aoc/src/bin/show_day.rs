use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

use aoc_client::{AocClient, AocError, SubmissionOutcome};
use chrono::{Datelike, FixedOffset, TimeZone, Utc};

fn main() {
    if dotenv::dotenv().is_err() {
        println!("Failed to load .env");
        return;
    }

    let (year, day, part) = get_args();
    let client = match get_client(year, day) {
        Ok(c) => c,
        Err(e) => {
            match e {
                AocError::InvalidEventYear(y) => println!("Bad year {y}"),
                AocError::InvalidPuzzleDay(d) => println!("Bad day {d}"),
                AocError::LockedPuzzle(d, y) => println!("{y}-{d} is still locked"),
                _ => println!("Client error {e}"),
            };
            return;
        }
    };

    if (!Path::new(&format!(r#"aoc{year}\inputs\day{day:02}.txt"#)).exists()
        || !Path::new(&format!(r#"aoc{year}\src\aoc{year}{day:02}.rs"#)).exists())
        && Command::new("build_day")
            .args([year.to_string(), day.to_string()])
            .spawn()
            .is_err()
    {
        println!("Something went wrong building the day. Exiting...");
        return;
    };

    let _ = client.show_calendar();

    if part == 1 {
        let _ = client.show_puzzle();
        if submit(&client, 1).is_none() {
            return;
        };
    }
    let _ = client.show_puzzle();
    submit(&client, 2);

    let _ = client.show_calendar();
}

fn get_client(year: i32, day: u32) -> Result<AocClient, AocError> {
    AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .build()
}

fn submit(client: &AocClient, part: u8) -> Option<()> {
    let mut answer = match get_input(&format!("Enter your answer for part {part}")) {
        Message::Value(s) => s,
        Message::Exit => return None,
    };
    loop {
        match client.submit_answer(part as i64, answer) {
            Ok(SubmissionOutcome::Correct) => return Some(()),
            Ok(SubmissionOutcome::Incorrect) => print!("That's incorrect. "),
            Ok(SubmissionOutcome::Wait) => print!("Resubmit too soon. Wait a bit and try again. "),
            Ok(SubmissionOutcome::WrongLevel) => print!("You're answering for the wrong day. "),
            Err(e) => println!("Error Submitting answer. {e}"),
        };
        answer = match get_input("Try Again") {
            Message::Value(s) => s,
            Message::Exit => return None,
        };
    }
}

enum Message {
    Value(String),
    Exit,
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        match value {
            "q" | "quit" | "exit" => Self::Exit,
            s => Self::Value(s.into()),
        }
    }
}

fn get_input(msg: &str) -> Message {
    print!("{msg}: ");
    let _ = stdout().flush();
    let mut answer = String::new();
    let stdin = stdin();
    let _ = stdin.read_line(&mut answer).unwrap_or_default();
    answer.trim().into()
}

fn get_args() -> (i32, u32, u8) {
    let mut args = env::args();
    args.next(); // Skip the program name.
    let (mut year, mut day) = default_date();
    let mut part = 1;
    for (idx, arg) in args.enumerate() {
        match idx {
            0 => year = arg.parse().unwrap(),
            1 => day = arg.parse().unwrap(),
            2 => part = arg.parse().unwrap(),
            _ => (),
        }
    }
    (year, day, part)
}

fn default_date() -> (i32, u32) {
    let east_coast = FixedOffset::west_opt(5 * 60 * 60).unwrap();
    let today = Utc::now().with_timezone(&east_coast);
    if today
        < east_coast
            .with_ymd_and_hms(today.year(), 12, 1, 0, 0, 0)
            .unwrap()
    {
        (today.year(), 1)
    } else {
        (today.year(), today.day())
    }
}
