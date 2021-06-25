use std::io::{self, Write};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const PUZZLE: i32 = 3613;
const W: usize = 300;
const H: usize = 300;

fn main() -> MainResult<()> {
    let mut map = vec![vec![0i32; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            let rack_id: i32 = j as i32 + 10;
            let power_level = rack_id * i as i32 + PUZZLE;
            let power_level = power_level * rack_id;
            map[i][j] = power_level / 100 % 10 - 5;
        }
    }
    let mut cum = [[0i32; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            cum[i][j] = cum[i - 1][j] + cum[i][j - 1] - cum[i - 1][j - 1] + map[i][j];
        }
    }
    part1(&cum)?;
    part2(&cum)?;
    Ok(())
}

fn part1(cum: &[[i32; H + 1]; W + 1]) -> MainResult<()> {
    let mut max_val = 0;
    let mut max_point = (0, 0);
    for i in 3..=H {
        for j in 3..=W {
            let tmp = cum[i][j] - cum[i - 3][j] - cum[i][j - 3] + cum[i - 3][j - 3];
            if tmp > max_val {
                max_val = tmp;
                max_point = (i - 2, j - 2);
            }
        }
    }
    let (x, y) = max_point;
    writeln!(io::stdout(), "{},{}", y, x)?;
    Ok(())
}

fn part2(cum: &[[i32; H + 1]; W + 1]) -> MainResult<()> {
    let mut max_val = 0;
    let mut max_point = (0, 0, 0);
    for k in 1..=H {
        for i in k..=H {
            for j in k..=W {
                let tmp = cum[i][j] - cum[i - k][j] - cum[i][j - k] + cum[i - k][j - k];
                if tmp > max_val {
                    max_val = tmp;
                    max_point = (i - k + 1, j - k + 1, k);
                }
            }
        }
    }
    let (x, y, z) = max_point;
    writeln!(io::stdout(), "{},{},{}", y, x, z)?;
    Ok(())
}
