use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use reqwest::{
    Url,
    blocking::{Client, ClientBuilder},
};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex, RawCookie};
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{fs::create_dir, io::Read};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

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
    println!("Created input.");
    match create_day(year, day) {
        Err(e) => {
            println!("{e}");
            return;
        }
        Ok(t) => println!("{t}"),
    }
    if let Err(e) = update_cargo(year, day) {
        println!("{e}");
        return;
    };
    println!("Updated aoc{year} Cargo.toml");
    if let Err(e) = update_bacon(year, day) {
        println!("{e}");
    };
    println!("Updated bacon.")
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
    let cookie_value = env::var("ADVENT_OF_CODE_SESSION").unwrap();
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

fn create_day(year: i32, day: u32) -> io::Result<String> {
    let filename = format!("aoc{year}/src/bin/aoc{year}{day:02}.rs");
    let file = Path::new(&filename);
    if file.exists() {
        return Ok(format!("{year} {day} already exists."));
    }
    if let Some(dir) = file.parent()
        && !dir.exists()
    {
        let _ = create_dir(dir);
    }
    let template = format!(
        r#" use aoc;

fn main() {{
    println!("---- {year}: {day:02} ----");
    let input = "";
    println!("Parsing");
    let model = parse(input);
    println!("Part 1: {{}}", part1(""));
    println!("Part 1: {{}}", part1(""));
}}

fn parse(input: &str) -> String {{
    input.into()
}}

fn part1<T>(_input: T) -> String {{
    "Unsolved".into()
}}

fn part2<T>(_input: T) -> String {{
    "Unsolved".into()
}}


#[cfg(test)]
mod test {{
    use super::*;

    #[test]
    fn test_example1() {{
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }}
}}
        "#
    );
    fs::write(file, template)?;
    Ok(format!("Created {year} {day}."))
}

/// Update bacon.toml to replace the aoc bin target with the year requested.
fn update_bacon(year: i32, day: u32) -> io::Result<()> {
    let mut bacon = get_existing_file("bacon.toml")?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    for line in bacon.iter_mut() {
        if line.contains("--package") {
            *line = format!(r#"    "--package", "aoc{year}","#);
        }
        if line.contains("--bin") {
            *line = format!(r#"    "--bin", "aoc{year}{day:02}","#);
        }
    }
    fs::write("bacon.toml", bacon.join("\n"))
}

/// Update the year's Cargo.toml file for the new binary.
fn update_cargo(year: i32, day: u32) -> Result<(), Box<dyn Error>> {
    let file = format!("aoc{year}/Cargo.toml");
    let mut cargo = get_existing_file(&file)?.parse::<DocumentMut>()?;
    let mut new_table = Table::new();
    new_table["name"] = value(format!("aoc{year}{day:02}"));
    let bin = cargo
        .entry("bin")
        .or_insert(Item::ArrayOfTables(ArrayOfTables::new()))
        .as_array_of_tables_mut()
        .unwrap();
    if !bin.iter().any(|t| *t.to_string() == new_table.to_string()) {
        bin.push(new_table);
    };
    fs::write(file, cargo.to_string())?;
    Ok(())
}

/// Read the existing file.
fn get_existing_file(file: &str) -> io::Result<String> {
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cookies() {
        let _ = dotenv::dotenv();
        let expected = env::var("ADVENT_OF_CODE_SESSION").unwrap();

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
        let expected = Ok(String::from("bgvyzdsv\n"));
        let _ = dotenv::dotenv();
        let cookie_store = Arc::new(CookieStoreMutex::new(load_cookies()));
        let client = ClientBuilder::new()
            .cookie_provider(cookie_store)
            .build()
            .unwrap();
        let actual = get_input(&client, 2015, 4);
        assert_eq!(expected, actual);
    }
}
