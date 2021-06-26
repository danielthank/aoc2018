use std::io::{self, BufRead, BufReader, Read, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn is_plant(ch: char) -> bool {
    match ch {
        '#' => true,
        '.' => false,
        _ => false,
    }
}

fn main() -> MainResult<()> {
    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input)?;
    let (_, input) = input.split_once(": ").ok_or("Parse error")?;
    let initial: Vec<bool> = input
        .as_bytes()
        .iter()
        .map(|&ch| is_plant(ch as char))
        .collect();

    // new line
    let mut input = String::new();
    reader.read_line(&mut input)?;
    input.clear();

    let mut transform = [false; 32];
    reader.read_to_string(&mut input)?;
    for line in input.lines() {
        let (l, r) = line.split_once(" => ").ok_or("Parse error")?;
        let l = l.as_bytes().iter().fold(0, |acc, &ch| match ch as char {
            '#' => acc * 2 + 1,
            '.' => acc * 2,
            _ => acc * 2,
        });
        let r = is_plant(r.chars().nth(0).ok_or("Parse error")?);
        transform[l] = r;
    }
    part1(&initial, &transform)?;
    part2(&initial, &transform)?;
    Ok(())
}

fn part1(initial: &[bool], transform: &[bool; 32]) -> MainResult<()> {
    const ITER_COUNT: i32 = 50;
    let mut now: Vec<bool> = initial.to_vec();
    for _ in 0..ITER_COUNT {
        let mut next = Vec::<bool>::new();
        let mut hash = 0usize;
        for &val in now.iter() {
            hash = (hash << 1) | (val as usize);
            hash %= 1 << 5;
            next.push(transform[hash]);
        }
        for _ in 0..4 {
            hash <<= 1;
            hash %= 1 << 5;
            next.push(transform[hash]);
        }
        /*
        writeln!(
            io::stdout(),
            "{}",
            next.iter()
                .skip(2 * ITER_COUNT as usize)
                .map(|&val| if val { '#' } else { '.' })
                .collect::<String>()
        )?;
        */
        now = next;
    }
    let ans: i32 = now
        .iter()
        .enumerate()
        .map(|(idx, &val)| if val { idx as i32 - 2 * ITER_COUNT } else { 0 })
        .sum();
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}

fn part2(initial: &[bool], transform: &[bool; 32]) -> MainResult<()> {
    const ITER_COUNT: i32 = 200;
    let mut now: Vec<bool> = initial.to_vec();
    let mut prev = 0;
    for generation in 0..ITER_COUNT {
        let mut next = Vec::<bool>::new();
        let mut hash = 0usize;
        for &val in now.iter() {
            hash = (hash << 1) | (val as usize);
            hash %= 1 << 5;
            next.push(transform[hash]);
        }
        for _ in 0..4 {
            hash <<= 1;
            hash %= 1 << 5;
            next.push(transform[hash]);
        }
        let ans: i32 = now
            .iter()
            .enumerate()
            .map(|(idx, &val)| if val { idx as i32 - 2 * generation } else { 0 })
            .sum();
        writeln!(io::stdout(), "{} {} {}", generation, ans, ans - prev)?;
        prev = ans;
        now = next;
    }
    // after observing the pattern
    let ans: u64 = 6767 + (50000000000 - 101) * 67;
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}
