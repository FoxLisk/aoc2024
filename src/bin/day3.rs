use std::{collections::HashMap, fs};

use regex::Regex;

const TEST_DATA: &'static str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const TEST_DATA_2: &'static str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> anyhow::Result<()> {
    part_one()?;
    part_two()
}

fn part_one() -> anyhow::Result<()> {
    let source = get_raw_data()?;
    let reg = regex::Regex::new(r#"mul\((\d+),(\d+)\)"#)?;

    let caps = reg.captures_iter(&source);
    let mut sum = 0;
    for c in caps {
        let a = c.get(1).unwrap().as_str().parse::<i32>()?;
        let b = c.get(2).unwrap().as_str().parse::<i32>()?;
        sum += a * b;
    }
    println!("Sum: {sum}");
    Ok(())
}

fn part_two() -> anyhow::Result<()> {
    let source = get_raw_data()?;
    let re_str = r#"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"#;
    println!("{re_str}");
    let reg = regex::Regex::new(re_str)?;
    let mut open = true;
    let mut sum = 0;
    for cap in reg.captures_iter(&source) {
        let c = cap.get(0).unwrap();
        match c.as_str() {
            "do()" => {
                open = true;
            }
            "don't()" => {
                open = false;
            }
            _ => {
                if !open {
                    continue;
                }
                let a = cap.get(1).unwrap().as_str().parse::<i32>()?;
                let b = cap.get(2).unwrap().as_str().parse::<i32>()?;
                sum += a * b;
            }
        }
    }
    println!("Sum with do/don'ts: {sum}");
    Ok(())
}

fn get_raw_data() -> anyhow::Result<String> {
    Ok(fs::read_to_string("inputs/day3")?)
    // Ok(TEST_DATA.to_string())
    // Ok(TEST_DATA_2.to_string())
}
