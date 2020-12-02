use anyhow::Result;
use std::collections::HashSet;

pub fn day1() -> Result<()> {
    let txt = include_str!("day1_input.txt");
    let expenses: Vec<u32> = txt
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    const DESIRED_SUM: u32 = 2020;

    let mut seen_values: HashSet<u32> = HashSet::new();
    for v in expenses.iter() {
        let target = DESIRED_SUM - v;
        if seen_values.contains(&target) {
            println!("{} * {} = {}", target, v, target * v);
            break;
        }
        seen_values.insert(*v);
    }

    for (e, v1) in expenses.iter().enumerate() {
        for (e, v2) in expenses[..e + 1].iter().enumerate() {
            for (e, v3) in expenses[..e + 1].iter().enumerate() {
                if v1 + v2 + v3 == DESIRED_SUM {
                    println!("{} * {} * {} = {}", v1, v2, v3, v1 * v2 * v3);
                    break;
                }
            }
        }
    }

    Ok(())
}
