mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use anyhow::Result;
use day1::*;
use day2::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("specify a day");
        return Ok(());
    }

    if let Some(day) = args.get(1) {
        match day.as_str() {
            "day1" | "1" => {
                day1()?;
            }
            "day2" | "2" => {
                day2()?;
            }
            "day3" | "3" => {
                day3()?;
            }
            "day4" | "4" => {
                day4()?;
            }
            "day5" | "5" => {
                day5()?;
            }
            "day6" | "6" => {
                day6()?;
            }
            _ => {
                eprintln!("Invalid day specification.");
            }
        }
    } else {
        eprintln!("Must specify a day.");
    }

    Ok(())
}
