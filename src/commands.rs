use std::{fs};
use std::cmp::{Ordering};
use std::collections::HashMap;
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
    let parsed = serde_json::to_string(&logs).unwrap();
    fs::write(path, parsed).expect("Failed to write");
}

pub fn log(project: String, time: Option<&String>) {

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

    // Parse the time
    let mut hour = date.hour();
    let mut minute = date.minute();
    if let Some(t) = time {
        hour = t.split('h').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        minute = t.split('h').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
    }

    day.entries.push(WorkEntry {hour: hour, minute: minute, message: project });

    day.entries.sort_by(|a, b| {
        if a.hour == b.hour {
            if a.minute < b.minute {
                Ordering::Less
            } else if a.minute > b.minute {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else if a.hour < b.hour {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    write_data(logs);

    show( None );
}

pub fn show(in_month: Option<&String>) {

    let date = chrono::Local::now();
    let month = if let Some( m ) = in_month { m.parse::<u32>().unwrap() } else { date.month() };
    let logs = parse_data(date.year() as u32, month );
    println!("Month: {}", logs.month);
    logs.days.iter().for_each(|d| {
        println!("{}/{}/{}", date.year(), month, d.day);
        d.entries.iter().for_each(|l| {
            println!("- {:02}h{:02} - {}", l.hour, l.minute, l.message);
        });
    });
    if logs.days.is_empty() {
        println!("No days");
    }
}

pub fn total(in_month: Option<&String>) {

    let date = chrono::Local::now();
    let month = if let Some( m ) = in_month { m.parse::<u32>().unwrap() } else { date.month() };
    let logs = parse_data(date.year() as u32, month );

    let mut minutes : HashMap<&String, u32> = HashMap::new();
    logs.days.iter().for_each(|d| {

        let mut prev_entry :Option<&WorkEntry> = None;

        for e in &d.entries {
            if let Some(previous) = prev_entry {
                let default_val = 0;
                let mut time = *minutes.get( &previous.message ).unwrap_or( &default_val );
                time += ( e.hour - previous.hour ) * 60 + e.minute - previous.minute;
                minutes.insert(&previous.message, time);
            }

            prev_entry = Some(&e);
        }
    });

    for (key, value) in minutes {
        let hour = (value as f32 / 60.0f32).floor();
        let minute = value % 60;
        println!("{:?}: {}h{}", key, hour, minute);
    }
}