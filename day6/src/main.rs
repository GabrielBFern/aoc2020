use std::collections::HashSet;

fn part1(data: &'static str) {
    let count: usize = data
        .split("\n\n")
        .map(|s| {
            s.lines()
                .flat_map(|l| l.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("Part 1: {}", count);
}

fn part2(data: &'static str) {
    let count: usize = data
        .split("\n\n")
        .map(|s| {
            let mut all: HashSet<_> = ('a'..='z').collect();
            s.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .for_each(|e| {
                    all = &all & &e;
                });
            all.len()
        })
        .sum();
    println!("Part 2: {}", count);
}

fn main() {
    let input = include_str!("../../input/d6");
    part1(input);
    part2(input);
}
