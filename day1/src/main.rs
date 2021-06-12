use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&mut input)?;
    part2(&mut input)?;
    Ok(())
}

fn part1(input: &mut String) -> MainResult<()> {
    let nums = input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    writeln!(io::stdout(), "{}", nums.iter().sum::<i32>())?;
    Ok(())
}

fn part2(input: &mut String) -> MainResult<()> {
    let mut h: HashSet<i32> = HashSet::new();
    let nums = input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut sum = 0;
    loop {
        for num in nums.iter() {
            h.insert(sum);
            sum += num;
            if h.contains(&sum) {
                writeln!(io::stdout(), "{}", sum)?;
                return Ok(());
            }
        }
    }
}
