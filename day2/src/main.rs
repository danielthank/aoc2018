use itertools::Itertools;
use std::io::{self, Read, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> MainResult<()> {
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        let mut count = vec![0; 26];
        for ch in line.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        if count.iter().any(|cnt| cnt == &2) {
            x += 1;
        }
        if count.iter().any(|cnt| cnt == &3) {
            y += 1;
        }
    }
    writeln!(io::stdout(), "{}", x * y)?;
    Ok(())
}

fn part2(input: &str) -> MainResult<()> {
    for pair in input.lines().combinations(2) {
        if let Some(pos) = check(pair[0], pair[1]) {
            writeln!(io::stdout(), "{}{}", &pair[0][..pos], &pair[0][pos + 1..])?;
            return Ok(());
        }
    }
    Ok(())
}

fn check(x: &str, y: &str) -> Option<usize> {
    let mut diff_count = 0;
    let mut diff_pos = 0usize;
    for (i, (x, y)) in x.chars().zip(y.chars()).enumerate() {
        if x != y {
            diff_count += 1;
            diff_pos = i;
        }
        if diff_count == 2 {
            return None;
        }
    }
    if diff_count == 0 {
        return None;
    }
    Some(diff_pos)
}
