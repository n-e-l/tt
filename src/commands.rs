use std::{fs};
use std::io::Write;
use std::cmp::{Ordering};
use std::collections::HashMap;
use std::fs::File;
use chrono::{Datelike, Timelike};
use homedir::my_home;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;

#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct WorkEntry {
    hour: u32,
    minute: u32,
    title: String
}
#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct DayEntry {
    day: u32,
    entries: Vec<WorkEntry>,
    logs: HashMap<String, String>
}

#[derive (Serialize, Deserialize, Debug)]
#[serde (tag = "type")]
struct MonthEntry {
    year: u32,
    month: u32,
    days: Vec<DayEntry>
}

fn get_work_dir() -> String {
    my_home().unwrap().unwrap().to_str().unwrap().to_owned() + "/.tt/"
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
    let parsed = serde_json::to_string_pretty(&logs).unwrap();
    fs::write(path, parsed).expect("Failed to write");
}

pub fn log(project: String, time: Option<&String>) {

    let date = chrono::Local::now();
    let mut month_entry = parse_data(date.year() as u32, date.month());

    // Add the current day if its missing
    if !month_entry.days.iter().any(|d| d.day == date.day()) {
        month_entry.days.push(DayEntry{ day: date.day(), entries: Vec::new(), logs: HashMap::new()});
    }

    // Retrieve the day
    let day = month_entry.days
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

    day.entries.push(WorkEntry { hour: hour, minute: minute, title: project });

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

    write_data(month_entry);

    show( None, None );
}

fn get_editor() -> String {
    let editor = std::env::var("EDITOR").expect("No EDITOR provided");
    return editor;
}

fn launch_editor(path: &str) {
    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("{} {}", get_editor(), path).as_str())
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");

}

pub fn write() {

    // Retrieve the current entry
    let date = chrono::Local::now();
    let mut logs = parse_data(date.year() as u32, date.month());
    let day = logs.days
        .iter_mut()
        .find(|d| d.day == date.day())
        .expect("month should contain the proper day");
    let entry : &mut WorkEntry = day.entries.last_mut().expect("day should contain at least one entry");

    let current_message = day.logs
        .iter()
        .find(|(k, _)| k.as_str().eq(&*entry.title))
        .map(|(_, v)| v.clone())
        .unwrap_or("".to_string());

    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("note.md");
    let mut file = File::create(file_path.clone()).unwrap();

    // Write node data
    write!(file, "{}", current_message).expect("Couldn't write to temp file");

    // Do vim thingy
    launch_editor(file_path.as_path().to_str().unwrap());

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    println!("{}", data);

    drop(file);
    dir.close().expect("Failed to close directory");

    day.logs.insert(entry.title.clone(), data.clone());

    write_data(logs);
}

pub fn show(in_month: Option<&String>, in_year: Option<&String>) {

    let date = chrono::Local::now();
    let month = if let Some( m ) = in_month { m.parse::<u32>().unwrap() } else { date.month() };
    let year = if let Some( m ) = in_year { m.parse::<u32>().unwrap() } else { date.year() as u32 };
    let logs = parse_data(year, month);
    println!("Month: {}", logs.month);
    logs.days.iter().for_each(|d| {
        println!("{}/{}/{}", date.year(), month, d.day);
        d.entries.iter().for_each(|l| {
            println!("- {:02}h{:02} - {}", l.hour, l.minute, l.title);
        });
    });
    if logs.days.is_empty() {
        println!("No days");
    }
}

pub fn total(in_month: Option<&String>, in_year: Option<&String>) {

    let date = chrono::Local::now();
    let month = if let Some( m ) = in_month { m.parse::<u32>().unwrap() } else { date.month() };
    let year = if let Some( m ) = in_year { m.parse::<u32>().unwrap() } else { date.year() as u32 };
    let logs = parse_data(year, month );

    let mut minutes : HashMap<&String, u32> = HashMap::new();
    logs.days.iter().for_each(|d| {

        let mut prev_entry :Option<&WorkEntry> = None;

        for e in &d.entries {
            if let Some(previous) = prev_entry {
                let default_val = 0;
                let mut time = *minutes.get( &previous.title ).unwrap_or( &default_val );
                time += ( e.hour - previous.hour ) * 60 + e.minute - previous.minute;
                minutes.insert(&previous.title, time);
            }

            prev_entry = Some(&e);
        }
    });

    for (key, value) in minutes {
        let hour = (value as f32 / 60.0f32).floor();
        let minute = value % 60;
        println!("{:?}: {}h{:02}", key, hour, minute);
    }
}