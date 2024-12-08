use std::fs;

const TEST_DATA: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> anyhow::Result<()> {
    part_one()?;
    part_two()
}

fn part_one() -> anyhow::Result<()> {
    let reports = get_reports()?;
    let mut tot = 0;
    for r in reports {
        println!(
            "{r:?} - {}safe",
            if report_is_valid(&r) { "" } else { "un" }
        );
        if report_is_valid(&r) {
            tot += 1;
        }
    }
    println!("Total safe reports: {tot}");
    Ok(())
}

fn part_two() -> anyhow::Result<()> {
    let reports = get_reports()?;
    let mut tot = 0;
    for r in reports {
        println!(
            "{r:?} - {}safe",
            if report_is_valid_with_dampener(&r) {
                ""
            } else {
                "un"
            }
        );
        if report_is_valid_with_dampener(&r) {
            tot += 1;
        }
    }
    println!("Total safe reports: {tot}");
    Ok(())
}

fn report_is_valid_with_dampener(report: &Vec<i32>) -> bool {
    if report_is_valid(&report) {
        return true;
    }
    // gross
    let mut rep_copy = report.clone();
    for i in (0..report.len()) {
        let stash = rep_copy.remove(i);
        if report_is_valid(&rep_copy) {
            return true;
        }
        rep_copy.insert(i, stash);
    }
    false
}

fn report_is_valid<'a>(report: &Vec<i32>) -> bool {
    let mut rep_iter = report.iter();
    let mut prev = match rep_iter.next() {
        Some(v) => v,
        None => {
            return true;
        }
    };
    let mut last_sign = None;
    while let Some(v) = rep_iter.next() {
        // println!("looking at {v}");
        let sign = prev > v;
        // println!("  sign: {sign}");
        if let Some(ls) = last_sign {
            if ls != sign {
                return false;
            }
        }
        last_sign = Some(sign);

        let d = (v - prev).abs();
        if d > 3 || d < 1 {
            return false;
        }
        // d < 1 should be covered by the sign handling if im not stupid
        prev = v;
    }
    true
}

fn get_raw_data() -> anyhow::Result<String> {
    Ok(fs::read_to_string("inputs/day2_1")?)
    // Ok(TEST_DATA.to_string())
}

fn get_reports() -> anyhow::Result<Vec<Vec<i32>>> {
    let raw = get_raw_data()?;
    let mut reports = vec![];
    for line in raw.lines() {
        // let levels = line.split_whitespace();
        let levels = line
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<_>>();
        reports.push(levels);
    }
    Ok(reports)
}
