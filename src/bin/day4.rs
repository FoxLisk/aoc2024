use std::{collections::HashMap, fs};

use regex::Regex;

const TEST_DATA: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> anyhow::Result<()> {
    part_one()?;
    part_two()
}

fn part_one() -> anyhow::Result<()> {
    let text = get_raw_data()?;
    let lines = text
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total_xmases = 0;
    for y in (0..lines.len()) {
        let line = lines.get(y).unwrap();
        for x in (0..line.len()) {
            let c = line.get(x).unwrap();
            if *c == 'X' {
                let xmases = find_xmases_from(&lines, x, y);
                // println!("{xmases} XMASes found starting from ({x}, {y})");
                total_xmases += xmases;
            }
        }
    }
    println!("Total xmases found: {total_xmases}");
    Ok(())
}

fn part_two() -> anyhow::Result<()> {
    let text = get_raw_data()?;
    let lines = text
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total_xmases = 0;
    for y in (0..lines.len()) {
        let line = lines.get(y).unwrap();
        for x in (0..line.len()) {
            let c = line.get(x).unwrap();
            if *c == 'A' {
                let xmases = find_xmas_crosses_from(&lines, x, y);
                // println!("{xmases} X-MASes found starting from ({x}, {y})");
                total_xmases += xmases;
            }
        }
    }
    println!("Total xmases found: {total_xmases}");
    Ok(())
}

fn find_xmases_from(lines: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let x_i = x as i32;
    let y_i = y as i32;
    let offset_steps = [
        (1, 0),
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut tot = 0;
    // println!("Looking from ({x}, {y})");
    for (offset_x, offset_y) in offset_steps {
        // println!("Looking at offset ({offset_x}, {offset_y})");
        let mut next_x = x_i + offset_x;
        let mut next_y = y_i + offset_y;
        if get_char_at(lines, next_x, next_y) != 'M' {
            // println!("No M at ({next_x}, {next_y})");
            continue;
        }
        next_x += offset_x;
        next_y += offset_y;
        if get_char_at(lines, next_x, next_y) != 'A' {
            // println!("No M at ({next_x}, {next_y})");
            continue;
        }
        next_x += offset_x;
        next_y += offset_y;
        if get_char_at(lines, next_x, next_y) != 'S' {
            // println!("No S at ({next_x}, {next_y})");
            continue;
        }
        tot += 1;
    }
    tot
}

fn find_xmas_crosses_from(wordsearch: &Vec<Vec<char>>, x_u: usize, y_u: usize) -> u32 {
    let x = x_u as i32;
    let y = y_u as i32;

    let cardinal = [((-1, 0), (1, 0)), ((0, 1), (0, -1))];
    let diagonal = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
    // its embarrassing how long it took me to figure out the correct grouping of pairs
    let crosses = [diagonal];

    let mut tot = 0;
    for cross in crosses {
        if is_mas_cross(wordsearch, x, y, cross) {
            // println!("Found X-MAS at ({x}, {y})");
            // let mut local_area = String::new();
            // local_area.push(get_char_at(wordsearch, x - 1, y - 1));
            // local_area.push(get_char_at(wordsearch, x - 0, y - 1));
            // local_area.push(get_char_at(wordsearch, x + 1, y - 1));
            // local_area.push('\n');
            // local_area.push(get_char_at(wordsearch, x - 1, y - 0));
            // local_area.push(get_char_at(wordsearch, x - 0, y - 0));
            // local_area.push(get_char_at(wordsearch, x + 1, y - 0));
            // local_area.push('\n');
            // local_area.push(get_char_at(wordsearch, x - 1, y + 1));
            // local_area.push(get_char_at(wordsearch, x - 0, y + 1));
            // local_area.push(get_char_at(wordsearch, x + 1, y + 1));
            // println!("Local area:\n{local_area}");
            tot += 1;
        }
    }

    tot
}

fn is_mas_cross(
    wordsearch: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    cross: [((i32, i32), (i32, i32)); 2],
) -> bool {
    for ((dx_a, dy_a), (dx_b, dy_b)) in cross {
        let a = get_char_at(wordsearch, x + dx_a, y + dy_a);
        let b = get_char_at(wordsearch, x + dx_b, y + dy_b);
        let spells_mas = (a == 'M' && b == 'S') || (a == 'S' && b == 'M');
        if !spells_mas {
            return false;
        }
    }
    true
}

fn get_char_at(wordsearch: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if y < 0 || x < 0 {
        return '.';
    }
    let x_i = x as usize;
    let y_i = y as usize;
    if y_i < wordsearch.len() {
        let row = wordsearch.get(y_i).unwrap();
        if x_i < row.len() {
            return row.get(x_i).unwrap().clone();
        }
    }
    '.'
}

fn get_raw_data() -> anyhow::Result<String> {
    let mut res = fs::read_to_string("inputs/day4")?;
    // res = TEST_DATA.to_string();
    // Ok(TEST_DATA_2.to_string())

    Ok(res)
}
