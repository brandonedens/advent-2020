use anyhow::Result;
use regex::Regex;

/// Attempt to validate password policy.
fn validate_sled_policy(min_times: usize, max_times: usize, c: char, line: &str) -> bool {
    let count = line.chars().filter(|a| *a == c).count();
    count >= min_times && count <= max_times
}

pub fn day2() -> Result<()> {
    let password_policy = include_str!("day2_input.txt");

    /*
        let password_policy = r#"1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc"#;
    */

    let re = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)")?;

    let num_valid_passwords = password_policy
        .lines()
        .filter(|line| {
            let caps = re.captures(line).unwrap();
            let min_times = caps[1].parse().unwrap();
            let max_times = caps[2].parse().unwrap();
            let c = caps[3].chars().next().unwrap();
            let line = &caps[4];
            validate_sled_policy(min_times, max_times, c, line)
        })
        .count();

    println!("num valid sled passwords: {}", num_valid_passwords);

    let num_valid_passwords = password_policy
        .lines()
        .filter(|line| {
            let caps = re.captures(line).unwrap();
            let pos_a: usize = caps[1].parse().unwrap();
            let pos_b: usize = caps[2].parse().unwrap();
            let c = caps[3].chars().next().unwrap();
            let line = &caps[4];
            (line.chars().nth(pos_a - 1).unwrap() == c)
                ^ (line.chars().nth(pos_b - 1).unwrap() == c)
        })
        .count();

    println!("num valid toboggan passwords: {}", num_valid_passwords);

    Ok(())
}
