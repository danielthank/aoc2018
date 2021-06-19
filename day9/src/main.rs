// cursor API is still unstable
#![feature(linked_list_cursors)]

use std::{
    collections::{linked_list::CursorMut, LinkedList},
    io::{self, Read, Write},
};

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut iter = input.split_whitespace();
    let token = iter.next();
    let m = token.ok_or("No input")?.parse::<u32>()?;
    let token = iter.skip(5).next();
    let n = token.ok_or("No input")?.parse::<u32>()?;
    part1(n, m)?;
    // part2 is the same as part1 but with larger input
    Ok(())
}

// cursor API inserts None between the last item and the first item of the linked list
// so define new apis
trait NoGhost<T> {
    fn my_move_next(&mut self);
    fn my_move_prev(&mut self);
    fn my_remove_current(&mut self) -> T;
}

impl<T> NoGhost<T> for CursorMut<'_, T> {
    fn my_move_next(&mut self) {
        self.move_next();
        if self.current().is_none() {
            self.move_next();
        }
    }

    fn my_move_prev(&mut self) {
        self.move_prev();
        if self.current().is_none() {
            self.move_prev();
        }
    }

    fn my_remove_current(&mut self) -> T {
        let t = self.remove_current().unwrap();
        if self.current().is_none() {
            self.move_next();
        }
        t
    }
}

fn part1(n: u32, m: u32) -> MainResult<()> {
    let mut l: LinkedList<u32> = LinkedList::new();
    let mut current = l.cursor_front_mut();
    let mut scores = vec![0u32; m as usize];
    let mut player_now = 0;
    current.insert_after(0);
    for i in 1..n {
        if i % 23 != 0 {
            current.my_move_next();
            current.insert_after(i);
            current.my_move_next();
        } else {
            scores[((i - 1) % m) as usize] += i as u32;
            for _ in 0..7 {
                current.my_move_prev();
            }
            scores[((i - 1) % m) as usize] += current.my_remove_current();
        }
        player_now = (player_now + 1) % m;
    }
    writeln!(io::stdout(), "{}", scores.iter().max().unwrap())?;
    Ok(())
}
