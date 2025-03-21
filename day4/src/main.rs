#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{cmp::Ordering, num::ParseIntError, ops::Sub, str::FromStr};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let sorted_entries = get_sorted_entries(input);
    let sleepiest_guard = find_sleepiest_guard(sorted_entries);
    let (time_most_asleep, _) = sleepiest_guard.time_most_asleep();
    println!(
        "Sleepiest guard was guard id: {}, minute most often asleep was: {}, multiplied: {}",
        sleepiest_guard.id,
        time_most_asleep,
        sleepiest_guard.id * time_most_asleep
    );
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let sorted_entries = get_sorted_entries(input);
    let guards = get_guards(sorted_entries);
    let (id, (min, num_times)) = guards
        .iter()
        .map(|(id, g)| (id, g.time_most_asleep()))
        .max_by(|(_, (_, n1)), (_, (_, n2))| n1.cmp(n2))
        .unwrap();
    println!("Guard id: {id} was asleep most often at same time ({num_times} times), at minute {min}. 
        Multiplied: {id} * {min} = {}", 
        id * min);
}

fn get_guards(entries: Vec<Entry>) -> HashMap<usize, Guard> {
    let mut map = HashMap::new();
    let mut cur_guard_id = 0;
    for entry in entries {
        match entry.entry_type {
            EntryType::FallsAsleep => {
                let cur_guard = map.entry(cur_guard_id).or_insert(Guard::new(0));
                cur_guard.entries.push(entry);
            }
            EntryType::WakesUp => {
                let cur_guard = map.entry(cur_guard_id).or_insert(Guard::new(0));
                let last_entry = cur_guard.entries.last().expect("Guard never fell asleep!");
                assert_eq!(
                    last_entry.entry_type,
                    EntryType::FallsAsleep,
                    "Guard didn't fall asleep"
                );
                cur_guard.time_sleeping += entry.datetime.diff(&last_entry.datetime);
                cur_guard.entries.push(entry);
            }
            EntryType::BeginsShift(id) => {
                map.entry(id).or_insert(Guard::new(id));
                cur_guard_id = id;
            }
        }
    }
    map
}

fn find_sleepiest_guard(entries: Vec<Entry>) -> Guard {
    let map = get_guards(entries);
    map.iter()
        .max_by(|(_, g1), (_, g2)| g1.time_sleeping.cmp(&g2.time_sleeping))
        .expect("Failed to find sleepiest guard")
        .1
        .clone()
}

fn get_sorted_entries(input: &str) -> Vec<Entry> {
    let mut entries = input
        .lines()
        .map(|l| l.parse::<Entry>().expect("Failed to parse into datetime"))
        .collect::<Vec<_>>();

    entries.sort();
    entries
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    id: usize,
    entries: Vec<Entry>,
    time_sleeping: usize,
}

impl Guard {
    fn new(id: usize) -> Self {
        Guard {
            id,
            entries: Vec::new(),
            time_sleeping: 0,
        }
    }

    fn time_most_asleep(&self) -> (usize, i32) {
        let mut times = [0; 60];
        let mut prev_time = None;
        for entry in &self.entries {
            match entry.entry_type {
                EntryType::FallsAsleep => prev_time = Some(entry.datetime.minute as usize),
                EntryType::WakesUp => {
                    if let Some(prev_time) = prev_time {
                        for min in prev_time..entry.datetime.minute as usize {
                            times[min] += 1;
                        }
                    } else {
                        panic!("Woke up but never fell asleep in first place")
                    }
                }
                EntryType::BeginsShift(_) => unreachable!("Because we don't store these"),
            }
        }

        let (min, num_times) = times
            .iter()
            .enumerate()
            .max_by(|(_, i1), (_, i2)| i1.cmp(i2))
            .unwrap();
        (min, *num_times)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum EntryType {
    FallsAsleep,
    WakesUp,
    BeginsShift(usize),
}

impl FromStr for EntryType {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "falls asleep" => Ok(EntryType::FallsAsleep),
            "wakes up" => Ok(EntryType::WakesUp),
            other => {
                let id = other
                    .split_whitespace()
                    .skip(1)
                    .next()
                    .expect("Missing guard id");
                let id = id[1..].parse::<usize>()?;
                Ok(EntryType::BeginsShift(id))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Entry {
    datetime: DateTime,
    entry_type: EntryType,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime_end = s.find(']').expect("Failed to find ']' in entry");
        let datetime = s[0..=datetime_end].parse::<DateTime>()?;
        let entry_type = s[datetime_end + 1..].parse::<EntryType>()?;
        Ok(Entry {
            datetime,
            entry_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl DateTime {
    fn diff(&self, rhs: &DateTime) -> usize {
        assert_eq!(
            self.year, rhs.year,
            "Can only compare datetimes if year is same"
        );
        assert_eq!(
            self.month, rhs.month,
            "Can only compare datetimes if month is same"
        );
        assert_eq!(
            self.day, rhs.day,
            "Can only compare datetimes if day is same"
        );
        assert_eq!(
            self.hour, rhs.hour,
            "Can only compare datetimes if hour is same"
        );
        assert_eq!(
            self.year, rhs.year,
            "Can only compare datetimes if year is same"
        );
        (self.minute - rhs.minute) as usize
    }
}

impl Sub for DateTime {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.year, rhs.year,
            "Can only compare datetimes if year is same"
        );
        assert_eq!(
            self.month, rhs.month,
            "Can only compare datetimes if month is same"
        );
        assert_eq!(
            self.day, rhs.day,
            "Can only compare datetimes if day is same"
        );
        assert_eq!(
            self.hour, rhs.hour,
            "Can only compare datetimes if hour is same"
        );
        assert_eq!(
            self.year, rhs.year,
            "Can only compare datetimes if year is same"
        );
        (self.minute - rhs.minute) as usize
    }
}

impl FromStr for DateTime {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(
            s.chars().next().expect("empty string, cannot parse"),
            '[',
            "First character should be '['"
        );
        let end_bracket = s
            .find(']')
            .expect("Failed to find end bracket ']' of date time");
        let substr = &s[1..end_bracket];
        let mut split = substr.split_whitespace();

        // parse date
        let date = split.next().expect("Empty datetime string");
        let mut date_parts = date.split('-');
        let (year, month, day) = (
            date_parts.next().expect("Missing year").parse::<u16>()?,
            date_parts.next().expect("Missing month").parse::<u8>()?,
            date_parts.next().expect("Missing day").parse::<u8>()?,
        );

        //parse time
        let time = split.next().expect("missing time specifier");
        let hour = time[0..2].parse::<u8>()?;
        let minute = time[3..].parse::<u8>()?;

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}
