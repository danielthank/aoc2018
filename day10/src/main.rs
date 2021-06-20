use std::{
    io::{self, Read, Write},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // position=< 50746, -40351> velocity=<-5,  4>
            static ref POINT_RE: Regex =
                Regex::new(r"position=<\s*(\-*\d+),\s*(\-*\d+)> velocity=<\s*(\-*\d+),\s*(\-*\d+)>").unwrap();
        }
        let cap = POINT_RE.captures(s).ok_or("Capture failed")?;
        Ok(Point {
            pos: (cap[1].parse()?, cap[2].parse()?),
            vel: (cap[3].parse()?, cap[4].parse()?),
        })
    }
}

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut data: Vec<Point> = Vec::new();
    for line in input.lines() {
        data.push(line.parse()?);
    }
    part1(&data)?;
    Ok(())
}

fn get_boundary(data: &[Point]) -> (i32, i32, i32, i32) {
    let xmin = data.iter().map(|data| data.pos.0).min().unwrap();
    let xmax = data.iter().map(|data| data.pos.0).max().unwrap();
    let ymin = data.iter().map(|data| data.pos.1).min().unwrap();
    let ymax = data.iter().map(|data| data.pos.1).max().unwrap();
    (xmin, xmax, ymin, ymax)
}

fn get_points_after_n_second(data: &[Point], n: i32) -> Vec<Point> {
    data.iter()
        .map(|data| Point {
            pos: (data.pos.0 + n * data.vel.0, data.pos.1 + n * data.vel.1),
            vel: data.vel,
        })
        .collect()
}

fn print_after_n_second(data: &[Point], n: i32) -> MainResult<()> {
    let data = get_points_after_n_second(data, n);
    let (xmin, xmax, ymin, ymax) = get_boundary(&data);
    writeln!(io::stdout(), "{} {} {} {}", xmin, xmax, ymin, ymax)?;
    let mut bitmap = vec![vec!['.'; (xmax - xmin + 1) as usize]; (ymax - ymin + 1) as usize];
    for point in data {
        bitmap[(point.pos.1 - ymin) as usize][(point.pos.0 - xmin) as usize] = '#';
    }
    for i in 0..bitmap.len() {
        for j in 0..bitmap[i].len() {
            write!(io::stdout(), "{}", bitmap[i][j])?;
        }
        writeln!(io::stdout(), "")?;
    }
    Ok(())
}

// define score as length + width
fn score_after_n_second(data: &[Point], n: i32) -> i32 {
    let (xmin, xmax, ymin, ymax) = get_boundary(&get_points_after_n_second(data, n));
    xmax - xmin + ymax - ymin
}

fn part1(data: &[Point]) -> MainResult<()> {
    let mut l = 0;
    let mut r = 20000;
    // binary search to find the minimum score
    while l < r {
        let mid = (l + r) / 2;
        let score1 = score_after_n_second(data, mid);
        let score2 = score_after_n_second(data, mid + 1);
        if score1 < score2 {
            r = mid;
        } else if score1 > score2 {
            l = mid + 1;
        } else {
            break;
        }
    }
    writeln!(io::stdout(), "{}", l)?;
    print_after_n_second(data, l)?;
    Ok(())
}
