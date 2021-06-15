use std::io::{self, Read, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Result<Vec<i32>, _> = input.split(' ').map(|str| str.parse::<i32>()).collect();
    let input = input?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn traverse1<'a>(mut input: &'a [i32], metadata_sum: &mut i32) -> &'a [i32] {
    let node_count = input[0] as usize;
    let metadata_count = input[1] as usize;
    input = &input[2..];
    for _ in 0..node_count {
        input = traverse1(input, metadata_sum);
    }
    *metadata_sum += input.iter().take(metadata_count).sum::<i32>();
    &input[metadata_count..]
}

fn traverse2<'a>(mut input: &'a [i32]) -> (&'a [i32], i32) {
    let node_count = input[0] as usize;
    let metadata_count = input[1] as usize;
    input = &input[2..];
    let mut vals: Vec<i32> = Vec::new();
    for _ in 0..node_count {
        let ret = traverse2(input);
        input = ret.0;
        vals.push(ret.1);
    }
    if node_count == 0 {
        return (
            &input[metadata_count..],
            input.iter().take(metadata_count).sum(),
        );
    } else {
        return (
            &input[metadata_count..],
            input
                .iter()
                .take(metadata_count)
                .map(|&idx| {
                    if 1 <= idx && idx as usize <= vals.len() {
                        vals[(idx - 1) as usize]
                    } else {
                        0
                    }
                })
                .sum(),
        );
    }
}

fn part1(input: &[i32]) -> MainResult<()> {
    let mut metadata_sum = 0;
    traverse1(input, &mut metadata_sum);
    writeln!(io::stdout(), "{}", metadata_sum)?;
    Ok(())
}

fn part2(input: &[i32]) -> MainResult<()> {
    let (_, ans) = traverse2(input);
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}
