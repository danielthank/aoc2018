use std::io::{self, Read, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const NODE_SIZE: usize = 26;
const WORKER_SIZE: usize = 5;
const TIME_NEED: usize = 61;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    const VAL: Vec<usize> = Vec::new();
    let mut map = [VAL; NODE_SIZE];
    for line in input.lines() {
        // 5, 36`
        let line = line.as_bytes();
        let x = line[5] as usize - 'A' as usize;
        let y = line[36] as usize - 'A' as usize;
        map[x].push(y);
    }
    part1(&map)?;
    part2(&map)?;
    Ok(())
}

fn part1(map: &[Vec<usize>; NODE_SIZE]) -> MainResult<()> {
    let mut in_count = [0i32; NODE_SIZE];
    for from in 0..NODE_SIZE {
        for &to in &map[from] {
            in_count[to] += 1;
        }
    }
    let mut ans = String::new();
    loop {
        match in_count.iter().position(|&x| x == 0) {
            Some(idx) => {
                in_count[idx] = -1;
                ans.push(('A' as u8 + idx as u8) as char);
                for &to in &map[idx] {
                    in_count[to] -= 1;
                }
            }
            None => break,
        };
    }
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}

fn part2(map: &[Vec<usize>; NODE_SIZE]) -> MainResult<()> {
    let mut in_count = [0i32; NODE_SIZE];
    for from in 0..NODE_SIZE {
        for &to in &map[from] {
            in_count[to] += 1;
        }
    }
    let mut workers: [(usize, Option<usize>); WORKER_SIZE] = [(0, None); WORKER_SIZE];
    let mut ans = 0;
    loop {
        for worker in 0..WORKER_SIZE {
            if workers[worker].0 != 0 {
                // Still working
                continue;
            }
            match in_count.iter().position(|&x| x == 0) {
                Some(idx) => {
                    workers[worker] = (idx + TIME_NEED, Some(idx));
                    in_count[idx] = -1;
                }
                None => (),
            };
        }
        if workers.iter().all(|&(v, _)| v == 0) {
            break;
        }
        let advance = workers.iter().filter(|&&(v, _)| v != 0).min().unwrap().0;
        ans += advance;
        for (time, worker) in workers.iter_mut().filter(|(v, _)| *v != 0) {
            *time -= advance;
            if *time == 0 {
                // After finishing the task, decrese in_count
                for &to in &map[worker.unwrap()] {
                    in_count[to] -= 1;
                }
                *worker = None;
            }
        }
    }
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}
