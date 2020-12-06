use std::convert::TryFrom;

enum PasswordPolice {
    Quantity,
    Position,
}

struct PasswordConstrains {
    letter: char,
    range: (usize, usize),
}

impl TryFrom<&str> for PasswordConstrains {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splited_value: Vec<&str> = value
            .split(|c: char| c == '-' || c.is_whitespace())
            .collect();
        if splited_value.len() != 3 {
            return Err(());
        };
        let letter = splited_value[2].chars().next().ok_or(())?;
        let range = match (
            splited_value[0].parse::<usize>(),
            splited_value[1].parse::<usize>(),
        ) {
            (Ok(x), Ok(y)) => (x, y),
            _ => return Err(()),
        };
        Ok(PasswordConstrains { letter, range })
    }
}

impl PasswordConstrains {
    fn validate(&self, police: PasswordPolice, password: &str) -> bool {
        match police {
            PasswordPolice::Quantity => {
                let quantity = password.chars().filter(|e| e.eq(&self.letter)).count();
                match quantity {
                    x if x < self.range.0 => false,
                    x if x > self.range.1 => false,
                    _ => true,
                }
            }
            PasswordPolice::Position => matches!((
                password.chars().nth(self.range.0 - 1),
                password.chars().nth(self.range.1 - 1),
            ), (Some(x), Some(y)) if (x == self.letter) ^ (y == self.letter)),
        }
    }
}

fn part1(data: &str) {
    let count = data
        .lines()
        .map(|f| -> bool {
            let args: Vec<&str> = f.split(": ").collect();
            PasswordConstrains::try_from(args[0])
                .unwrap()
                .validate(PasswordPolice::Quantity, args[1])
        })
        .filter(|r| *r)
        .count();
    dbg!(count);
}

fn part2(data: &str) {
    let count = data
        .lines()
        .map(|f| -> bool {
            let args: Vec<&str> = f.split(": ").collect();
            PasswordConstrains::try_from(args[0])
                .unwrap()
                .validate(PasswordPolice::Position, args[1])
        })
        .filter(|r| *r)
        .count();
    dbg!(count);
}

fn main() {
    let input = include_str!("../../input/d2");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tryfrom() {
        assert!(PasswordConstrains::try_from("8-11 l").is_ok());
    }

    #[test]
    fn test_validate_quantity() {
        assert!(PasswordConstrains::try_from("1-3 a")
            .unwrap()
            .validate(PasswordPolice::Quantity, "abcde"));
        assert!(!PasswordConstrains::try_from("1-3 b")
            .unwrap()
            .validate(PasswordPolice::Quantity, "cdefg"));
        assert!(PasswordConstrains::try_from("2-9 c")
            .unwrap()
            .validate(PasswordPolice::Quantity, "ccccccccc"));
    }

    #[test]
    fn test_validate_position() {
        assert!(PasswordConstrains::try_from("1-3 a")
            .unwrap()
            .validate(PasswordPolice::Position, "abcde"));
        assert!(!PasswordConstrains::try_from("1-3 b")
            .unwrap()
            .validate(PasswordPolice::Position, "cdefg"));
        assert!(!PasswordConstrains::try_from("2-9 c")
            .unwrap()
            .validate(PasswordPolice::Position, "ccccccccc"));
    }
}
