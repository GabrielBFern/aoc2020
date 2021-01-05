use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::{alpha0, char, digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{Finish, IResult};

#[derive(Debug, Clone)]
struct Program {
    accumulator: isize,
    program_counter: usize,
    instructions: Vec<OpCode>,
}
#[derive(Debug, PartialEq, Clone)]
enum OpCode {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn main() {
    let input = include_str!("../../input/d8");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let program = parse_program(input).expect("invalid input");
    let (_, acu) = program.run_program();
    println!("Part1: {:}", acu);
}

// TODO: Remake this, brute force is not acceptable
fn part2(input: &str) {
    let program = parse_program(input).expect("invalid input");
    let change = program
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, m)| !matches!(m, OpCode::Acc(_)))
        .find(|(f, _)| {
            let mut p = program.clone();
            p.change_op(*f);
            p.run_program().0.is_none()
        });
    let mut p = program.clone();
    p.change_op(change.unwrap().0);
    let (_, acu) = p.run_program();
    println!("Part2: {:}", acu);
}

impl Program {
    fn tick(&mut self) -> Option<&OpCode> {
        let op = self.instructions.get(self.program_counter)?;
        match op {
            OpCode::Acc(i) => {
                self.accumulator += i;
                self.program_counter += 1;
            }
            OpCode::Jmp(i) => {
                self.program_counter = if i.is_negative() {
                    self.program_counter.checked_sub(i.wrapping_abs() as usize)
                } else {
                    self.program_counter.checked_add(*i as usize)
                }?;
            }
            OpCode::Nop(_) => {
                self.program_counter += 1;
            }
        }
        Some(op)
    }

    fn change_op(&mut self, index: usize) {
        self.instructions[index] = match self.instructions.get(index).unwrap() {
            OpCode::Nop(i) => OpCode::Jmp(*i),
            OpCode::Jmp(i) => OpCode::Nop(*i),
            _ => unimplemented!(),
        };
    }

    // Run the program and return if found loop and the last accumulator
    fn run_program(mut self) -> (Option<()>, isize) {
        let mut instruction_viewed: HashSet<usize> = HashSet::new();
        loop {
            if !instruction_viewed.insert(self.program_counter) {
                break (Some(()), self.accumulator);
            }
            if self.tick().is_none() {
                break (None, self.accumulator);
            }
        }
    }
}

impl TryFrom<(&str, isize)> for OpCode {
    type Error = ();

    fn try_from(value: (&str, isize)) -> Result<Self, Self::Error> {
        match value.0 {
            "acc" => Ok(Self::Acc(value.1)),
            "jmp" => Ok(Self::Jmp(value.1)),
            "nop" => Ok(Self::Nop(value.1)),
            _ => Err(()),
        }
    }
}

fn parse_program(input: &str) -> Result<Program, String> {
    match all_consuming(separated_list1(line_ending, parse_op))(input).finish() {
        Ok((_remaining, op_codes)) => Ok(Program {
            accumulator: 0,
            program_counter: 0,
            instructions: op_codes,
        }),
        Err(_) => Err("Parse error".into()),
    }
}

fn parse_op(input: &str) -> IResult<&str, OpCode> {
    map_res(tuple((alpha0, space1, parse_isize)), |(f, _, i)| {
        OpCode::try_from((f, i))
    })(input)
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(tuple((alt((char('-'), char('+'))), digit1))),
        FromStr::from_str,
    )(input)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_opcode_parse() {
        let (_, d) = parse_op("acc -9").unwrap();
        assert_eq!(d, OpCode::Acc(-9));
        let (_, d) = parse_op("nop -9").unwrap();
        assert_eq!(d, OpCode::Nop(-9));
        let (_, d) = parse_op("jmp -9").unwrap();
        assert_eq!(d, OpCode::Jmp(-9));
        let (_, d) = parse_op("jmp +9").unwrap();
        assert_eq!(d, OpCode::Jmp(9));
    }

    #[test]
    fn test_example_1() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let program = parse_program(input).unwrap();
        let result = program.run_program();
        assert_eq!(result, (Some(()), 5));
    }

    #[test]
    fn test_example_2() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\nnop -4\nacc +6";
        let program = parse_program(input).unwrap();
        let result = program.run_program();
        assert_eq!(result, (None, 8));
    }
}
