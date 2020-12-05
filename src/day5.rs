use anyhow::Result;
use std::collections::HashSet;

fn parse_boarding_pass(boarding_pass: &str) -> Result<u32> {
    let bin_val = boarding_pass
        .replace("F", "0")
        .replace("B", "1")
        .replace("R", "1")
        .replace("L", "0");
    Ok(u32::from_str_radix(bin_val.as_str(), 2)?)
}

pub fn day5() -> Result<()> {
    let boarding_passes = include_str!("day5_input.txt");

    let highest_seat_id = boarding_passes
        .lines()
        .map(|pass| parse_boarding_pass(pass).unwrap())
        .max()
        .unwrap();
    println!("highest seat id: {}", highest_seat_id);

    let seats: HashSet<_> = boarding_passes
        .lines()
        .map(|pass| parse_boarding_pass(pass).unwrap())
        .collect();
    let last_row = highest_seat_id / 8;

    let all_seats: HashSet<_> = (8..(last_row * 8)).collect();

    let empty_seat: HashSet<_> = all_seats.difference(&seats).collect();
    assert_eq!(empty_seat.len(), 1);
    println!("empty seat: {}", empty_seat.iter().next().unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_passes() {
        assert_eq!(parse_boarding_pass("FBFBBFFRLR").unwrap(), 357);
        assert_eq!(parse_boarding_pass("BFFFBBFRRR").unwrap(), 567);
        assert_eq!(parse_boarding_pass("FFFBBBFRRR").unwrap(), 119);
        assert_eq!(parse_boarding_pass("BBFFBBFRLL").unwrap(), 820);
    }
}
