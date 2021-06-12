use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    str::FromStr,
};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Node {
    id: i32,
    posx: i32,
    posy: i32,
    lenx: i32,
    leny: i32,
}

impl FromStr for Node {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // #1 @ 167,777: 23x12
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)\#
                (?P<id>\d+)\s+@\s+
                (?P<posx>\d+),
                (?P<posy>\d+):\s+
                (?P<lenx>\d+)x
                (?P<leny>\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(s).ok_or("Capture failed")?;
        Ok(Node {
            id: caps["id"].parse()?,
            posx: caps["posx"].parse()?,
            posy: caps["posy"].parse()?,
            lenx: caps["lenx"].parse()?,
            leny: caps["leny"].parse()?,
        })
    }
}

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut data: Vec<Node> = Vec::new();
    for line in input.lines() {
        let node = line.parse::<Node>()?;
        data.push(node);
    }
    part1(&data)?;
    part2(&data)?;
    Ok(())
}

fn part1(data: &[Node]) -> MainResult<()> {
    let mut h: HashMap<(i32, i32), bool> = HashMap::new();
    let mut ans = 0;
    for item in data {
        for i in item.posx..item.posx + item.lenx {
            for j in item.posy..item.posy + item.leny {
                h.entry((i, j))
                    .and_modify(|v| {
                        if !*v {
                            ans += 1;
                        }
                        *v = true;
                    })
                    .or_default();
            }
        }
    }
    writeln!(io::stdout(), "{}", ans)?;
    Ok(())
}

fn part2(data: &[Node]) -> MainResult<()> {
    for i in 0..data.len() {
        let mut ok = true;
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            if check_overlap(&data[i], &data[j]) {
                ok = false;
                break;
            }
        }
        if ok {
            writeln!(io::stdout(), "{}", data[i].id)?;
            return Ok(());
        }
    }
    Ok(())
}

fn check_overlap(a: &Node, b: &Node) -> bool {
    let (al, ar) = (a.posx, a.posx + a.lenx);
    let (bl, br) = (b.posx, b.posx + b.lenx);
    if ar <= bl || br <= al {
        return false;
    }
    let (al, ar) = (a.posy, a.posy + a.leny);
    let (bl, br) = (b.posy, b.posy + b.leny);
    if ar <= bl || br <= al {
        return false;
    }
    return true;
}
