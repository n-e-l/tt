use std::{fs};
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct WorkEntry {
    hour: u32,
    minute: u32,
    message: String,
}
#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct DayEntry {
    day: u32,
    entries: Vec<WorkEntry>
}

#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct MonthEntry {
    year: u32,
    month: u32,
    days: Vec<DayEntry>
}

fn get_work_dir() -> String {
    std::env::home_dir().unwrap().to_str().unwrap().to_owned() + "/.tt/"
}

pub fn get_current_datafile(year: u32, month: u32) -> String {
    get_work_dir() + format!("tt_{}_{}.json", year, month).as_str()
}

fn parse_data(year: u32, month: u32) -> MonthEntry {
    let logs: MonthEntry;

    // Load data
    let path = get_current_datafile(year, month);
    if fs::metadata(path.clone()).is_ok()
    {
        let data = fs::read_to_string(path.clone()).expect("Unable to read file");
        logs = serde_json::from_str(&data).expect("Unable to parse");
    }
    else {
        logs = MonthEntry { year: year, month: month, days: Vec::new() };
    }

    return logs;
}

fn write_data(logs: MonthEntry) {
    let path = get_current_datafile(logs.year, logs.month);
    println!("{:?}", logs);
    let parsed = serde_json::to_string(&logs).unwrap();
    fs::write(path, parsed).expect("Failed to write");
}

pub fn log(project: String) {

    let date = chrono::Local::now();
    let mut logs = parse_data(date.year() as u32, date.month());

    // Add the current day if its missing
    if !logs.days.iter().any(|d| d.day == date.day()) {
        logs.days.push(DayEntry{ day: date.day(), entries: Vec::new()});
    }

    // Retrieve the day
    let day = logs.days
        .iter_mut()
        .find(|d| d.day == date.day())
        .expect("month should contain the proper day");

    day.entries.push(WorkEntry {hour: date.hour(), minute: date.minute(), message: project });

    write_data(logs);
}

pub fn show() {

    let date = chrono::Local::now();
    let logs = parse_data(date.year() as u32, date.month());
    println!("Month: {}", logs.month);
    logs.days.iter().for_each(|d| {
        println!("Day: {}", d.day);
        d.entries.iter().for_each(|l| {
            println!("- Log: {:02}h{:02} - {}", l.hour, l.minute, l.message);
        });
    });
    if logs.days.is_empty() {
        println!("No days");
    }
}