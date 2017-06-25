#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod workout;

extern crate serde;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;
use chrono::Duration;
use chrono::prelude::*;
use workout::Workout;

static FILE_PATH: &'static str = "./pushups_data.json";
const HOURS_IN_DAY: f64 = 24.0;
const PUSHUPS_TARGET: u32 = 10_000;

fn get_file_contents(mut file: File) -> String {
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => s,
    }
}

fn parse_json(file_contents: &str) -> Result<Vec<Workout>, serde_json::Error> {
    if file_contents.is_empty() {
        Ok(vec![])
    } else {
        serde_json::from_str(file_contents)
    }
}

fn get_workouts_days_span(first: &Workout, last: &Workout) -> u32 {
    let hours = last.difference(first).num_hours();
    let days = (hours as f64 / HOURS_IN_DAY).ceil() as u32;
    if days == 0 { 1 } else { days }
}

fn get_total_pushups(workouts: &[Workout]) -> u32 {
    workouts.iter().map(|workout| workout.get_repeats()).sum()
}

fn get_progress_speed_per_day(total_pushups: u32, num_days: u32) -> u32 {
    total_pushups / num_days
}

fn get_progress_percentage(total_pushups: u32) -> f64 {
    (total_pushups as f64 / PUSHUPS_TARGET as f64) * 100.0
}

fn estimate_target_reached_date(last_day: &Workout, total_pushups: u32, total_days: u32) -> DateTime<FixedOffset> {
    let pushups_left_todo = PUSHUPS_TARGET - total_pushups;
    let progress_speed = get_progress_speed_per_day(total_pushups, total_days);
    let days_ahead_until_target = pushups_left_todo / progress_speed;
    DateTime::parse_from_rfc2822(last_day.get_date()).unwrap() + Duration::days(days_ahead_until_target as i64)
}

fn main() {
    // 0 read cmd line args, create a workout object out of it
    // TODO - for now its mock
    let now = Utc::now();
    let mock = Workout::new(now.to_rfc2822(), 45);

    // 1 read file contents to string - it might be empty
    let path = Path::new(FILE_PATH);
    let mut workouts: Vec<Workout> = File::open(&path)
        .map(get_file_contents)
        .or_else(|_| Ok("".to_string()))
        .and_then(|s| parse_json(&s))
        .expect("invalid workouts JSON!");

    // 2 push the workout object into array, sort it
    workouts.push(mock);
    workouts.sort();

    // 3 get days difference (last - first), get total pushups - calculate the "speed" of pushups
    let total_pushups: u32 = get_total_pushups(&workouts);
    let first = workouts.first().unwrap();
    let last = workouts.last().unwrap();
    let total_days = get_workouts_days_span(first, last);

    if total_pushups >= PUSHUPS_TARGET {
        println!("Conratulations! You have reached the goal. You have completed {} pushups in just {} days!", total_pushups, total_days);
    } else {
        let estimated_date_of_target_reached = estimate_target_reached_date(last, total_pushups, total_days);
        println!("Nice! Your current progress is {}%. Keep going!", format!("{:.*}", 2, get_progress_percentage(total_pushups)));
        println!("If you keep the pace, you will reach your goal on {}-{}-{}.", 
            estimated_date_of_target_reached.day(), estimated_date_of_target_reached.month(), estimated_date_of_target_reached.year());
    }

    // 4 serialize to string and save to file
    let serialized = serde_json::to_string(&workouts).expect("could not serialize!");
    let mut file = File::create(&path).expect("could not open file for write operation");
    if let Err(why) = file.write_all(serialized.as_bytes()) {
        println!("{}", why.description());
    }
}
