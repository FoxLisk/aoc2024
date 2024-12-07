use std::{collections::HashMap, fs};

use regex::Regex;

const TEST_DATA: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> anyhow::Result<()> {
    part_one()?;
    part_two()
}

fn part_one() -> anyhow::Result<()> {
    let (mut left, mut right) = get_lists()?;
    left.sort();
    right.sort();
    let res = left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (r - l).abs());
    println!("{res}");
    Ok(())
}

fn part_two() -> anyhow::Result<()> {
    let (left, right) = get_lists()?;
    let mut right_counts: HashMap<i32, i32> = Default::default();
    for i in right {
        let e = right_counts.entry(i).or_insert(0);
        *e += 1;
    }
    // println!("{right_counts:?}");

    let sims = left
        .iter()
        .fold(0, |acc, i| acc + i * right_counts.get(&i).unwrap_or(&0));
    println!("{sims}");
    Ok(())
}

macro_rules! err {
    ($t:expr) => {
        return Err(anyhow::anyhow!($t));
    };
}

fn get_raw_data() -> anyhow::Result<String> {
    Ok(fs::read_to_string("inputs/day1_1")?)
    // Ok(TEST_DATA.to_string())
}

fn get_lists() -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let raw = get_raw_data()?;
    let mut left = vec![];
    let mut right = vec![];
    let re = Regex::new(r#"(\d+)\s+(\d+)"#)?;
    let handle_line = |line: &str| -> Option<(i32, i32)> {
        let matches = re.captures(line)?;
        let left_i = matches.get(1)?.as_str().parse().ok()?;
        let right_i = matches.get(2)?.as_str().parse().ok()?;
        Some((left_i, right_i))
    };
    for row in raw.lines() {
        match handle_line(row) {
            Some((l_i, r_i)) => {
                left.push(l_i);
                right.push(r_i);
            }
            None => {
                err!("Error parsing line `{row}`");
            }
        }
    }
    Ok((left, right))
}
