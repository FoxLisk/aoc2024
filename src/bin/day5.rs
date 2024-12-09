use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

const TEST_DATA: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() -> anyhow::Result<()> {
    part_one()?;
    part_two()
}

fn part_one() -> anyhow::Result<()> {
    let text = get_raw_data()?;
    // println!("{text}");
    let (rules, updates) = parse_raw_data(text)?;
    // the values of must_precede are all the pages that must be before the key
    let mut must_precede: HashMap<u32, HashSet<u32>> = Default::default();
    // the values of must_succeed are all the page stha tmust be after the key
    let mut must_succeed: HashMap<u32, HashSet<u32>> = Default::default();
    for rule in &rules {
        must_precede
            .entry(rule.after)
            .or_default()
            .insert(rule.before);
        must_succeed
            .entry(rule.before)
            .or_default()
            .insert(rule.after);
    }
    // println!("{rules:?}");
    // println!("{updates:?}");
    let mut tot = 0;
    for u in updates {
        println!("Checking {u:?}");
        let valid = update_valid(&rules, &u);
        println!("  valid? {valid:?}");
        if valid {
            let middle = u.pages[u.pages.len() / 2];
            tot += middle;
        }
    }
    println!("Total of middle thingies: {tot}");
    Ok(())
}

fn part_two() -> anyhow::Result<()> {
    let text = get_raw_data()?;
    // println!("{text}");
    let (rules, updates) = parse_raw_data(text)?;
    // the values of must_precede are all the pages that must be before the key
    let mut must_precede: HashMap<u32, HashSet<u32>> = Default::default();
    // the values of must_succeed are all the page stha tmust be after the key
    let mut must_succeed: HashMap<u32, HashSet<u32>> = Default::default();
    for rule in &rules {
        must_precede
            .entry(rule.after)
            .or_default()
            .insert(rule.before);
        must_succeed
            .entry(rule.before)
            .or_default()
            .insert(rule.after);
    }
    for u in &updates {
        if u.pages.len() != u.pages.iter().collect::<HashSet<_>>().len() {
            println!("update with repeated element: {u:?}");
        }
    }
    // println!("{rules:?}");
    // println!("{updates:?}");
    let mut tot = 0;
    for u in updates {
        let valid = update_valid(&rules, &u);
        if !valid {
            let sorted = sort_invalid(&rules, &u)?;
            println!(
                "\
Sorted: before {:?}
        after  {sorted:?}",
                u.pages
            );
            let middle = sorted[sorted.len() / 2];
            tot += middle;
        }
    }
    println!("Total of middle thingies: {tot}");
    Ok(())
}

fn sort_invalid(rules: &Vec<Rule>, update: &Update) -> anyhow::Result<Vec<u32>> {
    let mut deps: HashMap<u32, Vec<u32>> = Default::default();
    let relevant_pages = update.pages.iter().collect::<HashSet<_>>();
    for rule in rules {
        if relevant_pages.contains(&rule.before) && relevant_pages.contains(&rule.after) {
            deps.entry(rule.after).or_default().push(rule.before);
        }
    }
    let mut topo = topo_sort::TopoSort::with_capacity(update.pages.len());
    for p in &update.pages {
        topo.insert(*p, deps.remove(p).unwrap_or_default());
    }
    topo.try_into_vec_nodes().map_err(From::from)
}

fn update_valid(rules: &Vec<Rule>, update: &Update) -> bool {
    let mut active_rules: Vec<&Rule> = vec![];
    let all_pages = update.pages.iter().cloned().collect::<HashSet<_>>();
    for rule in rules {
        if all_pages.contains(&rule.before) && all_pages.contains(&rule.after) {
            active_rules.push(rule);
        }
    }
    struct Appearances {
        first: usize,
        last: usize,
    }
    let mut appearances: HashMap<u32, Appearances> = Default::default();
    for (index, page) in update.pages.iter().enumerate() {
        appearances
            .entry(*page)
            .or_insert(Appearances {
                first: index,
                last: index,
            })
            .last = index;
    }
    for rule in active_rules {
        let bef_app = appearances.get(&rule.before).expect(&format!(
            "Missing appearances for expected page {}",
            rule.before
        ));
        let aft_app = appearances.get(&rule.after).expect(&format!(
            "Missing appearances for expected page {}",
            rule.before
        ));
        if bef_app.first > aft_app.first {
            println!("Violation of rule {}|{}", rule.before, rule.after);
            return false;
        }
    }

    true
}

fn get_raw_data() -> anyhow::Result<String> {
    let mut res = fs::read_to_string("inputs/day5")?;
    // res = TEST_DATA.to_string();
    // Ok(TEST_DATA_2.to_string())

    Ok(res)
}

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

fn parse_raw_data(raw: String) -> anyhow::Result<(Vec<Rule>, Vec<Update>)> {
    let mut rules = vec![];
    let mut lines = raw.lines();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let mut parts = line.split('|');
        rules.push(Rule {
            before: parts.next().unwrap().parse().unwrap(),
            after: parts.next().unwrap().parse().unwrap(),
        });
    }

    let mut updates = vec![];

    while let Some(line) = lines.next() {
        let pages = line
            .split(',')
            .map(|c| c.parse().expect(&format!("can't parse {line}")))
            .collect::<Vec<_>>();
        updates.push(Update { pages });
    }
    Ok((rules, updates))
}
