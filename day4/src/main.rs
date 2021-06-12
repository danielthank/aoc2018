use std::{
    collections::HashMap,
    io::{self, Read, Write},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RecordType {
    Begin(i32),
    Sleep,
    Awake,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    record_type: RecordType,
}

#[derive(Debug)]
struct Stat {
    id: i32,
    sleep: [bool; 60],
}

impl FromStr for Record {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
        [1518-03-15 23:57] Guard #2909 begins shift
        [1518-04-07 00:17] falls asleep
        [1518-07-01 00:44] wakes up
        */
        lazy_static! {
            static ref RECORD_RE: Regex =
                Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)").unwrap();
            static ref BEGIN_RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
            static ref SLEEP_RE: Regex = Regex::new(r"falls asleep").unwrap();
            static ref AWAKE_RE: Regex = Regex::new(r"wakes up").unwrap();
        }
        let cap = RECORD_RE.captures(s).ok_or("Capture failed")?;
        let record_type;
        if let Some(type_cap) = BEGIN_RE.captures(&cap[6]) {
            record_type = RecordType::Begin(type_cap[1].parse()?);
        } else if let Some(_) = SLEEP_RE.captures(&cap[6]) {
            record_type = RecordType::Sleep;
        } else if let Some(_) = AWAKE_RE.captures(&cap[6]) {
            record_type = RecordType::Awake;
        } else {
            return Err(Box::<dyn std::error::Error>::from("Cannot identify record"));
        }
        Ok(Self {
            year: cap[1].parse()?,
            month: cap[2].parse()?,
            day: cap[3].parse()?,
            hour: cap[4].parse()?,
            minute: cap[5].parse()?,
            record_type,
        })
    }
}

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut records: Vec<Record> = Vec::new();
    for line in input.lines() {
        records.push(line.parse()?);
    }
    records.sort_unstable();
    let mut stats: Vec<Stat> = Vec::new();
    let mut from: usize = 0;
    for record in records.iter() {
        match record.record_type {
            RecordType::Begin(id) => {
                let stat = Stat {
                    id: id,
                    sleep: [false; 60],
                };
                stats.push(stat);
            }
            RecordType::Sleep => {
                from = record.minute as usize;
            }
            RecordType::Awake => {
                stats
                    .last_mut()
                    .map(|stat| {
                        for i in from..record.minute as usize {
                            stat.sleep[i] = true;
                        }
                    })
                    .ok_or("No last item")?;
            }
        };
    }
    part1(&stats)?;
    part2(&stats)?;
    Ok(())
}

fn part1(stats: &[Stat]) -> MainResult<()> {
    let mut h: HashMap<i32, usize> = HashMap::new();
    for stat in stats {
        *h.entry(stat.id).or_default() += stat.sleep.iter().filter(|b| **b).count();
    }
    let (&id, _) = h.iter().max_by_key(|(_, v)| *v).ok_or("No data")?;
    let mut sum = [0; 60];
    for stat in stats {
        if stat.id != id {
            continue;
        }
        for i in 0..60 {
            sum[i] += stat.sleep[i] as i32;
        }
    }
    let (minute, _) = sum
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .ok_or("No data")?;
    writeln!(io::stdout(), "{} {} {}", id, minute, id * minute as i32)?;
    Ok(())
}

fn part2(stats: &[Stat]) -> MainResult<()> {
    let mut h: HashMap<i32, usize> = HashMap::new();
    for stat in stats {
        for i in 0..60 {
            if stat.sleep[i] {
                *h.entry(stat.id * 60 + i as i32).or_default() += 1;
            }
        }
    }
    let tmp = h
        .iter()
        .max_by_key(|&(_, v)| v)
        .map(|(k, _)| k)
        .ok_or("No data")?;
    writeln!(
        io::stdout(),
        "{} {} {}",
        tmp / 60,
        tmp % 60,
        (tmp / 60) * (tmp % 60)
    )?;
    Ok(())
}
