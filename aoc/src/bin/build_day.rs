use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use reqwest::{
    blocking::{Client, ClientBuilder},
    Url,
};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex, RawCookie};
use std::{
    env,
    fmt::Display,
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

const URL: &str = "https://adventofcode.com";
const DOMAIN: &str = "adventofcode.com";
const PREVIEW: usize = 50;
const LINES: usize = 5;

fn main() {
    if dotenv::dotenv().is_err() {
        println!("Failed to load .env");
        return;
    }

    let cookie_store = Arc::new(CookieStoreMutex::new(load_cookies()));
    let client = ClientBuilder::new()
        .cookie_provider(cookie_store)
        .build()
        .unwrap();

    let (year, day) = match get_args() {
        None => {
            println!("Invalid arguments. Please supply year and day");
            return;
        }
        Some((year, day)) => (year, day),
    };
    if day == 0 {
        for day in 1..=25 {
            build_day(year, day, &client);
        }
    } else {
        build_day(year, day, &client);
    }
}

fn build_day(year: i32, day: u32, client: &Client) {
    let file = PathBuf::from(&format!("aoc{year}/inputs/day{day:02}.txt"));
    let data = match get_input(client, year, day) {
        Ok(text) => text,
        Err(InputResult::NotLoggedIn) => {
            println!("Session cookie missing.");
            return;
        }
        Err(InputResult::TooEarly) => {
            println!("That day is not active yet.");
            return;
        }
        Err(InputResult::BadText) => {
            println!("Return text error.");
            return;
        }
        Err(InputResult::BadUrl) | Err(InputResult::NotFound) => {
            println!("Can't find URL. Check dates input.");
            return;
        }
        Err(InputResult::RequestError) => {
            println!("Error sending GET request.");
            return;
        }
        Err(InputResult::UnknownResponse) => {
            println!("Unknown reqwest error. Please try again.");
            return;
        }
    };
    show_preview(&data);
    write_file(file, data);
    create_day(year, day);
    update_tests(year, day);
    update_main(year, day);
}

#[derive(Debug, PartialEq)]
enum InputResult {
    RequestError,
    NotLoggedIn,
    BadText,
    TooEarly,
    NotFound,
    UnknownResponse,
    BadUrl,
}

fn get_input<T: Display, U: Display>(
    client: &Client,
    year: T,
    day: U,
) -> Result<String, InputResult> {
    let url = match format!("{URL}/{year}/day/{day}/input").parse::<Url>() {
        Err(_) => return Err(InputResult::BadUrl),
        Ok(url) => url,
    };

    let response = match client.get(url).send() {
        Err(_) => return Err(InputResult::RequestError),
        Ok(response) => response,
    };
    match (response.status().as_u16(), response.text()) {
        (_, Err(_)) => Err(InputResult::BadText),
        (400, Ok(text)) | (404, Ok(text)) => {
            if text.contains("log in") {
                Err(InputResult::NotLoggedIn)
            } else if text.contains("before it unlocks") {
                Err(InputResult::TooEarly)
            } else {
                Err(InputResult::NotFound)
            }
        }
        (200, Ok(text)) => Ok(text),
        _ => Err(InputResult::UnknownResponse),
    }
}

fn load_cookies() -> CookieStore {
    let cookie_value = env::var("AOC_COOKIE").unwrap();
    let cookie = RawCookie::build("session", cookie_value)
        .domain(DOMAIN)
        .path("/")
        .secure(true)
        .finish();
    let mut store = CookieStore::new(None);
    store.insert_raw(&cookie, &URL.parse().unwrap()).unwrap();
    store
}

fn show_preview(data: &str) {
    let count = data
        .chars()
        .filter(|chr| *chr == '\n')
        .collect::<Vec<_>>()
        .len();
    println!("{count} Lines Downloaded");
    let lines = data
        .lines()
        .map(|l| l.chars().take(PREVIEW).collect::<String>())
        .collect::<Vec<String>>();
    let max = LINES.min(lines.len());
    for line in lines.iter().take(max) {
        println!("{line}");
    }
    println!("{} Lines", lines.len())
}

fn write_file(file: PathBuf, data: String) {
    let parent = file.parent().unwrap();
    if file.exists() {
        return;
    }
    let _ = fs::create_dir_all(parent);
    let _ = fs::write(file, data);
}

fn get_args() -> Option<(i32, u32)> {
    let mut args = env::args();
    match args.len() {
        1 => {
            let east_coast = FixedOffset::west_opt(5 * 60 * 60).unwrap();
            let today = Utc::now().with_timezone(&east_coast);
            if today
                < east_coast
                    .with_ymd_and_hms(today.year(), 12, 1, 0, 0, 0)
                    .unwrap()
            {
                Some((today.year(), 1))
            } else {
                Some((today.year(), today.day()))
            }
        }
        2 => {
            args.next();
            let year = args.next();
            let day = 0;
            Some((year.unwrap().parse().unwrap(), day))
        }
        3 => {
            args.next();
            let year = args.next();
            let day = args.next();
            Some((
                year.unwrap().parse().unwrap(),
                day.unwrap().parse().unwrap(),
            ))
        }
        _ => None,
    }
}

fn create_day(year: i32, day: u32) {
    let filename = format!("aoc{year}/src/aoc{year}{day:02}.rs");
    let file = Path::new(&filename);
    if file.exists() {
        return;
    }
    let template = format!(
        r#"use aoc::runner::{{output, Runner}};

#[derive(Default)]
pub struct AocDay {{
    input: String,
}}

impl for AocDay {{
    pub fn new<S: Into<String>>(input: S) -> Self {{
        Self {{
            input: input.into(),
            ..Default::default()
        }}
    }}
}}

impl Runner for AocDay {{
    fn name(&self) -> (usize, usize) {{
        ({year}, {day})
    }}

    fn parse(&mut self) {{
        // Parse the input
    }}

    fn part1(&mut self) -> Vec<String> {{
        output("Unsolved")
    }}

    fn part2(&mut self) -> Vec<String> {{
        output("Unsolved")
    }}
}}
        "#
    );
    let _ = fs::write(file, template);
}

fn update_tests(year: i32, day: u32) {
    let mut template = format!(
        r#"
mod aoc_{year}{day}_tests {{
    use super::*;
    use crate::aoc{year}{day:02}::*;

    static INPUT: &str = "";

    #[test]
    fn test_part1() {{
        let mut day = AocDay::new(INPUT);
        day.parse();
        let expected = 0;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }}

    #[test]
    fn test_part2() {{
        let mut day = AocDay::new(INPUT);
        day.parse();
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }}
}}
"#
    );
    match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(format!("aoc{year}/src/tests.rs"))
    {
        Ok(mut file) => {
            if file.seek(SeekFrom::End(0)).unwrap() == 0 {
                template.insert_str(0, "use aoc::runner::Runner;\n\n");
            }
            let _ = file.write(template.as_bytes());
        }
        Err(_) => println!("Could not update tests."),
    };
}

fn update_main(year: i32, day: u32) {
    let module = format!("aoc{year}{day:02}");
    let file = format!("aoc{year}/main.rs");
    if let Ok(mut contents) = fs::read_to_string(&file) {
        if let Some(mod_line) = contents.find(&format!("mod aoc{year}{:02}", day - 1)) {
            let new_mod = format!("mod {module};\n");
            contents.insert_str(mod_line + new_mod.len(), &new_mod);
        };

        if let Some(struct_line) = contents.find(&format!("    let day{:02}", day - 1)) {
            let new_struct =
                format!(r#"    let day{day:02} = {module}::AocDay::new("inputs/day{day:02}.txt")"#);
            contents.insert_str(struct_line + new_struct.len(), &new_struct);
        };
        if let Some(collection) = contents.find("let mut days") {
            if let Some(measure) = contents.find("    let len = ") {
                let mut new_arr = "let mut days = [".to_string();
                for d in 1..day {
                    new_arr.push_str(&format!("day{d:02},"));
                }
                new_arr.push_str("];\n");
                contents.replace_range(collection..measure, &new_arr);
            }
        };
        let _ = fs::write(file, contents);
    } else {
        let _ = fs::write(
            file,
            r#"use std::env;

use aoc::runner::run_solution;

mod aoc{year}01;

#[cfg(test)]
mod tests;

fn main() {{
    let day01 = aoc{year}01::AocDay::new("inputs/day01.txt");
    let mut days = [day01];
    let len = days.len() - 1;
    match get_args() {{
        Some(0) => {{
            // Run all days
            for selected in days.iter_mut() {{
                run_solution(selected);
            }}
        }}
        Some(d) => {{
            // Run selected day
            let selected = &mut days[(d - 1).min(len)];
            run_solution(selected)
        }}
        None => {{
            // Run last day
            let selected = &mut days[len];
            run_solution(selected);
        }}
    }};
}}

fn get_args() -> Option<usize> {{
    let mut args = env::args();
    match args.len() {{
        2 => {{
            args.next();
            Some(args.next().unwrap().parse().unwrap())
        }}
        _ => None,
    }}
}}
        "#,
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cookies() {
        let _ = dotenv::dotenv();
        let expected = env::var("AOC_COOKIE").unwrap();

        let cookies = load_cookies();
        let actual = cookies.get_any(DOMAIN, "/", "session").unwrap();
        assert_eq!(expected, actual.value());
    }

    #[test]
    fn test_no_cookie() {
        let expected = Err(InputResult::NotLoggedIn);
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
        let client = ClientBuilder::new()
            .cookie_provider(cookie_store)
            .build()
            .unwrap();
        let actual = get_input(&client, 2015, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_successful_data() {
        let expected = Ok(aoc::read_line("inputs/2015/day01.txt")
            .iter()
            .collect::<String>()
            .trim()
            .into());
        let _ = dotenv::dotenv();
        let cookie_store = Arc::new(CookieStoreMutex::new(load_cookies()));
        let client = ClientBuilder::new()
            .cookie_provider(cookie_store)
            .build()
            .unwrap();
        let actual = get_input(&client, 2015, 1);
        assert_eq!(expected, actual);
    }
}
