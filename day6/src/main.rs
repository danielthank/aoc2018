use std::{
    collections::HashSet,
    io::{self, Read, Write},
    str::FromStr,
};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").ok_or("Parse error")?;
        Ok(Self(x.parse()?, y.parse()?))
    }
}

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        points.push(line.parse()?);
    }
    part1(&points)?;
    part2(&points)?;
    Ok(())
}

fn dis(x: &Point, y: &Point) -> i32 {
    (x.0 as i32 - y.0 as i32).abs() + (x.1 as i32 - y.1 as i32).abs()
}

fn find_nearest(now: &Point, points: &[Point]) -> Option<usize> {
    let mut nearest = 0;
    let mut only = true;
    for i in 1..points.len() {
        let tmp = dis(now, &points[i]) - dis(now, &points[nearest]);
        if tmp < 0 {
            nearest = i;
            only = true;
        } else if tmp == 0 {
            only = false;
        }
    }
    if !only {
        return None;
    }
    Some(nearest)
}

fn part1(points: &[Point]) -> MainResult<()> {
    let mut map = vec![vec![None as Option<usize>; 1000]; 1000];
    let mut counts = vec![Some(0); points.len()];
    for x in 0..1000 {
        for y in 0..1000 {
            map[x][y] = find_nearest(&Point(x as i32, y as i32), points);
            if x == 0 || x == 999 || y == 0 || y == 999 {
                if let Some(idx) = map[x][y] {
                    counts[idx] = None;
                }
            }
        }
    }
    for x in 0..1000 {
        for y in 0..1000 {
            if let Some(idx) = map[x][y] {
                if let Some(ref mut count) = counts[idx] {
                    *count += 1;
                }
            }
        }
    }
    writeln!(
        io::stdout(),
        "{:?}",
        counts
            .iter()
            .max()
            .ok_or("No data")?
            .ok_or("No definite point")?
    )?;
    Ok(())
}

fn part2(points: &[Point]) -> MainResult<()> {
    let mut xs = points.iter().map(|point| point.0).collect::<Vec<_>>();
    xs.sort();
    let mut ys = points.iter().map(|point| point.1).collect::<Vec<_>>();
    ys.sort();
    let xmid = xs[xs.len() / 2];
    let ymid = xs[ys.len() / 2];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = Vec::new();
    stack.push(Point(xmid, ymid));
    visited.insert(Point(xmid, ymid));
    let mut ans = 0;
    while !stack.is_empty() {
        let now = stack.pop().unwrap();
        let total_dis: i32 = points.iter().map(|point| dis(point, &now)).sum();
        if total_dis >= 10000 {
            continue;
        }
        ans += 1;
        for (i, j) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] as Vec<(i32, i32)> {
            let next = Point(now.0 + i, now.1 + j);
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next.clone());
            stack.push(next);
        }
    }
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}
