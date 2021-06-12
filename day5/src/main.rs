use std::io::{self, Read, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.lines().next().ok_or("No data")?;
    part1(input)?;
    part2(input)?;
    Ok(())
}

fn check(a: u8, b: u8) -> bool {
    let tmp = ((a as i8) - (b as i8)).abs();
    tmp == ('a' as i8) - ('A' as i8)
}

fn reduce(input: &[u8]) -> Option<Vec<u8>> {
    let mut v = vec![false; input.len()];
    let mut i: i32 = 0;
    let mut ans = input.len();
    while (i as usize) < input.len() - 1 {
        let mut l = i;
        let mut r = i + 1;
        while l >= 0
            && (r as usize) < input.len()
            && !v[l as usize]
            && !v[r as usize]
            && check(input[l as usize], input[r as usize])
        {
            v[l as usize] = true;
            v[r as usize] = true;
            l -= 1;
            r += 1;
            ans -= 2
        }
        i += 1;
    }
    if ans == input.len() {
        return None;
    }
    Some(
        input
            .iter()
            .zip(v)
            .filter(|&(_, deleted)| !deleted)
            .map(|(ch, _)| *ch)
            .collect(),
    )
}

fn remove(input: &[u8], delete: u8) -> Vec<u8> {
    input
        .iter()
        .filter(|&&ch| ch != delete && ch != delete - 'A' as u8 + 'a' as u8)
        .cloned()
        .collect()
}

fn part1(input: &str) -> MainResult<()> {
    let mut now = input.as_bytes().to_owned();
    while let Some(next) = reduce(&now) {
        writeln!(io::stdout(), "{}", now.len())?;
        now = next;
    }
    writeln!(io::stdout(), "part1: {}", now.len())?;
    Ok(())
}

fn part2(input: &str) -> MainResult<()> {
    let input = input.as_bytes().to_owned();
    let mut ans = input.len();
    for delete in 'A'..'Z' {
        let mut now = remove(&input, delete as u8);
        while let Some(next) = reduce(&now) {
            now = next;
        }
        writeln!(io::stdout(), "{}", now.len())?;
        ans = ans.min(now.len());
    }
    writeln!(io::stdout(), "part2: {}", ans)?;
    Ok(())
}
