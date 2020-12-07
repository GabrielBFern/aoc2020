use std::{convert::TryFrom, error::Error};
#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}

impl TryFrom<&str> for Seat {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.len() {
            10 => {
                let bin = value
                    .chars()
                    .map(|c| match c {
                        'F' | 'L' => "0",
                        'B' | 'R' => "1",
                        _ => panic!("Invalid caracter"),
                    })
                    .collect::<String>();
                let n = bin.split_at(7);
                Ok(Seat {
                    row: usize::from_str_radix(n.0, 2)?,
                    column: usize::from_str_radix(n.1, 2)?,
                })
            }
            _ => Err("Invalid length".into()),
        }
    }
}

impl From<usize> for Seat {
    fn from(v: usize) -> Self {
        Seat {
            row: v >> 3,
            column: v & 0b111,
        }
    }
}

impl Seat {
    fn get_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn part1(data: &str) {
    let max_id = data
        .lines()
        .map(|l| Seat::try_from(l).unwrap().get_id())
        .max()
        .unwrap();
    println!("Part 1: {}", max_id);
}

fn part2(data: &str) {
    let mut seat_exist = data
        .lines()
        .map(|l| Seat::try_from(l).unwrap())
        .filter(|s| s.row != 0 && s.row != 127)
        .map(|s| s.get_id())
        .collect::<Vec<_>>();
    seat_exist.sort_unstable();
    let seat = seat_exist
        .windows(2)
        .filter(|&e| e[1] - e[0] == 2)
        .collect::<Vec<_>>();
    if let Some([ant, _]) = seat.get(0) {
        println!("Part 2: {}", ant + 1);
    } else {
        println!("Part 2: not found");
    }
}
fn main() {
    let input = include_str!("../../input/d5");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(
            Seat::try_from("BFFFBBFRRR").unwrap(),
            Seat { row: 70, column: 7 }
        );
        assert_eq!(
            Seat::try_from("FFFBBBFRRR").unwrap(),
            Seat { row: 14, column: 7 }
        );
        assert_eq!(
            Seat::try_from("BBFFBBFRLL").unwrap(),
            Seat {
                row: 102,
                column: 4
            }
        );
    }
    #[test]
    fn test_from() {
        assert_eq!(Seat::from(567), Seat { row: 70, column: 7 });
        assert_eq!(Seat::from(119), Seat { row: 14, column: 7 });
        assert_eq!(
            Seat::from(820),
            Seat {
                row: 102,
                column: 4
            }
        );
    }
}
