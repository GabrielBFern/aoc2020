use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::{map, recognize},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Bag {
    name: String,
    insides: Vec<(u32, String)>,
}

type BagsRules = HashMap<String, Bag>;

fn parse_bags(input: &str) -> BagsRules {
    input
        .lines()
        .filter_map(|f| {
            let b = parse_bag(f);
            match b {
                Ok(b) => Some((b.name.clone(), b)),
                Err(_) => None,
            }
        })
        .collect()
}

fn parse_bag(input: &str) -> Result<Bag, String> {
    let result = tuple((
        bag_name,
        tag(" contain "),
        alt((
            map(tag("no other bags"), |_| Vec::new()),
            separated_list1(
                tag(", "),
                map(tuple((digit1, space1, bag_name)), |tuple| {
                    (tuple.0.parse::<u32>().unwrap(), tuple.2.to_string())
                }),
            ),
        )),
    ))(input);
    if let Ok((_, (name, _, insides))) = result {
        Ok(Bag {
            name: name.into(),
            insides,
        })
    } else {
        Err("Parse error".into())
    }
}

fn bag_name(input: &str) -> IResult<&str, &str> {
    terminated(
        recognize(tuple((alpha1, space1, alpha1))),
        alt((tag(" bags"), tag(" bag"))),
    )(input)
}

fn part1(rules: &BagsRules) {
    let count = rules
        .iter()
        .filter(|(_, b)| contains_gold(&rules, b))
        .count();
    println!("Part 1: {}", count);
}

fn contains_gold(rules: &BagsRules, bag: &Bag) -> bool {
    bag.insides
        .iter()
        .any(|(_, k)| k == "shiny gold" || contains_gold(rules, rules.get(k).unwrap()))
}

fn part2(rules: &BagsRules) {
    let count = contains_bags(&rules, rules.get("shiny gold").unwrap());
    println!("Part 2: {}", count);
}

fn contains_bags(rules: &BagsRules, bag: &Bag) -> u32 {
    bag.insides
        .iter()
        .map(|(c, k)| c + (c * contains_bags(rules, rules.get(k).unwrap())))
        .sum()
}

fn main() {
    let input = include_str!("../../input/d7");
    let rules = parse_bags(input);
    part1(&rules);
    part2(&rules);
}
