use std::collections::HashSet;

fn part1(data: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut complement: HashSet<u32> = HashSet::new();
    for e in data {
        if complement.contains(e) {
            return Some((target - e, *e));
        }
        if target > *e {
            complement.insert(target - e);
        }
    }
    None
}

fn part2(data: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    for e in data {
        let complement = target - e;
        if let Some((a, b)) = part1(data, complement) {
            return Some((*e, a, b));
        }
    }
    None
}

fn main() {
    let input = include_str!("../../input/d1");
    let parsed_input: Vec<u32> = input.lines().map(|e| e.parse::<u32>().unwrap()).collect();
    println!("Part1: {:?}", part1(parsed_input.as_slice(), 2020));
    println!("Part2: {:?}", part2(parsed_input.as_slice(), 2020));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Some((1, 2)), part1(&[1, 2, 3], 3));
        assert_eq!(Some((1, 3)), part1(&[1, 2, 3], 4));
        assert_eq!(None, part1(&[1, 2, 3], 6));
    }

    #[test]
    fn test_part2() {
        assert_eq!(Some((3, 6, 5)), part2(&[2, 3, 6, 5], 14));
        assert_eq!(None, part2(&[1, 2, 3], 3));
    }
}
