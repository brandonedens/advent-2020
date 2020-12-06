use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;

fn count_unique_answers(answers: &str) -> Result<usize> {
    let split_answers = Regex::new("\n\n")?;
    Ok(split_answers
        .split(answers)
        .map(|answer| {
            answer
                .replace("\n", "")
                .replace(" ", "")
                .chars()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum())
}

fn count_duplicate_answers(answers: &str) -> Result<usize> {
    let split_answers = Regex::new("\n\n")?;

    Ok(split_answers
        .split(answers)
        .map(|answer_set| {
            let sets = answer_set
                .split_whitespace()
                .map(|answers| answers.chars().collect::<HashSet<_>>())
                .collect::<Vec<HashSet<_>>>();
            let mut final_set: HashSet<_> = sets.first().unwrap().clone();
            sets.iter().cloned().skip(1).for_each(|set| {
                let set = set.iter().cloned().collect();
                final_set = final_set.intersection(&set).cloned().collect();
            });
            final_set.len()
        })
        .sum())
}

pub fn day6() -> Result<()> {
    let answers = include_str!("day6_input.txt");
    let sum = count_unique_answers(answers)?;
    println!("sum of the unique counts: {}", sum);

    let sum = count_duplicate_answers(answers)?;
    println!("sum of the duplicate counts: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_answers() {
        let answers = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        assert_eq!(count_unique_answers(answers).unwrap(), 11);
    }

    #[test]
    fn test_count_duplicate_answers() {
        let answers = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        assert_eq!(count_duplicate_answers(answers).unwrap(), 6);
    }
}
