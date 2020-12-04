use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

pub fn day4() -> Result<()> {
    let passports = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
    let passports = include_str!("day4_input.txt");

    let split_passports = Regex::new("\n\n")?;

    let re = Regex::new("(byr|iyr|eyr|hgt|hcl|ecl|pid):([#0-9a-zA-Z]+)")?;

    let num_valid_passports = split_passports
        .split(passports)
        .filter(|passport| {
            let fields: HashMap<&str, &str> = passport
                .split_whitespace()
                .filter_map(|kv| {
                    re.captures(kv).map(|caps| {
                        (
                            caps.get(1).map(|m| m.as_str()).unwrap(),
                            caps.get(2).map(|m| m.as_str()).unwrap(),
                        )
                    })
                })
                .collect();

            if fields.contains_key(&"byr")
                && fields.contains_key(&"iyr")
                && fields.contains_key(&"eyr")
                && fields.contains_key(&"hgt")
                && fields.contains_key(&"hcl")
                && fields.contains_key(&"ecl")
                && fields.contains_key(&"pid")
            {
                let byr_val = fields.get("byr").unwrap().parse::<u32>().unwrap();
                let iyr_val = fields.get("iyr").unwrap().parse::<u32>().unwrap();
                let eyr_val = fields.get("eyr").unwrap().parse::<u32>().unwrap();

                enum Height {
                    Cm(usize),
                    In(usize),
                }
                let hgt_val = fields.get("hgt").unwrap();
                let hcl_val = fields.get("hcl").unwrap();
                let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                let ecl_val = fields.get("ecl").unwrap();
                let pid_val = fields.get("pid").unwrap();

                byr_val >= 1920
                    && byr_val <= 2002
                    && iyr_val >= 2010
                    && iyr_val <= 2020
                    && eyr_val >= 2020
                    && eyr_val <= 2030
                    && if hgt_val.ends_with("cm") {
                        let z = hgt_val.replace("cm", "").parse::<u32>().unwrap();
                        z >= 150 && z <= 193
                    } else if hgt_val.ends_with("in") {
                        let z = hgt_val.replace("in", "").parse::<u32>().unwrap();
                        z >= 59 && z <= 76
                    } else {
                        false
                    }
                    && if hcl_val.starts_with("#") {
                        let hcl_val = hcl_val.replace("#", "");
                        hcl_val.trim().chars().count() == 6
                            && hcl_val.trim().chars().all(|c| {
                                let c = c as u8;
                                (c >= '0' as u8 && c <= '9' as u8)
                                    || (c >= 'a' as u8 && c <= 'f' as u8)
                            })
                    } else {
                        false
                    }
                    && eye_colors.contains(ecl_val)
                    && pid_val.len() == 9
                    && pid_val.chars().all(|c| {
                        let c = c as u8;
                        c >= '0' as u8 && c <= '9' as u8
                    })
            } else {
                false
            }
        })
        .count();

    println!("num valid passports: {}", num_valid_passports);

    Ok(())
}
