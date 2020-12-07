use std::{collections::HashMap, convert::TryFrom};

#[derive(PartialEq, Debug)]
struct Document {
    fields: HashMap<&'static str, &'static str>,
}

impl TryFrom<&'static str> for Document {
    type Error = ();

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let fields: HashMap<&str, &str> = value
            .split_whitespace()
            .map(|s| {
                let mut tuple = s.splitn(2, ':');
                (tuple.next().unwrap(), tuple.next().unwrap())
            })
            .collect();
        Ok(Document { fields })
    }
}

fn part1(data: &'static str) {
    let count = data
        .split_terminator("\n\n")
        .map(|s| Document::try_from(s).unwrap())
        .filter(validate_fields)
        .count();
    println!("Part 1 {}", count);
}

fn part2(data: &'static str) {
    let count = data
        .split_terminator("\n\n")
        .map(|s| Document::try_from(s).unwrap())
        .filter(validate_fields)
        .filter(validate)
        .count();
    println!("Part 2 {}", count);
}

fn validate_fields(document: &Document) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|s| document.fields.contains_key(*s))
}

fn validate(d: &Document) -> bool {
    d.fields.iter().all(|(&key, value)| match key {
        "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),
        "hgt" => {
            let h = value[0..(value.len() - 2)].parse().unwrap_or(0);
            match &value[(value.len() - 2)..] {
                "in" => (59..=76).contains(&h),
                "cm" => (150..=193).contains(&h),
                _ => false,
            }
        }
        "hcl" => {
            value.starts_with('#')
                && value.len() == 7
                && value.chars().skip(1).all(|h| h.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
        "pid" => value.len() == 9 && value.chars().all(|d| d.is_ascii_digit()),
        "cid" => true,
        _ => panic!("Invalid field"),
    })
}

fn main() {
    let input = include_str!("../../input/d4");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod test {
    use super::*;
}
