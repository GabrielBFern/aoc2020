use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::{alpha0, char, digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{Finish, IResult};

#[derive(Default)]
struct Cpu {
    accumulator: isize,
    program_counter: usize,
}
#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<OpCode>,
}
#[derive(Debug, PartialEq, Clone)]
enum OpCode {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn main() {
    let input = include_str!("../../input/d8large");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let program = parse_program(input).expect("invalid input");
    let (_, acu) = Cpu::default().run_program(&program);
    println!("Part1: {:}", acu);
}

fn part2(input: &str) {
    let mut program = parse_program(input).expect("invalid input");
    let end_point = generate_endpoints(&program);
    let mut cpu = Cpu::default();
    loop {
        let op = cpu.get_instruction(&program).unwrap();
        if let Some(changed_op) = op.change() {
            let destiny = cpu.calculate_destination(&changed_op);

            if end_point.contains(&destiny) {
                program.instructions[cpu.program_counter] = changed_op;
                break;
            }
        }
        cpu.tick(&program);
    }
    let (_, acu) = Cpu::default().run_program(&program);
    println!("Part2: {:}", acu);
}

fn generate_endpoints(program: &Program) -> HashSet<usize> {
    let mut destinations: HashMap<usize, HashSet<usize>> = HashMap::new();
    let length = program.instructions.len() as isize;
    program
        .instructions
        .iter()
        .enumerate()
        .map(|(ori, op)| {
            let des: isize = match op {
                OpCode::Jmp(x) => ori as isize + x,
                _ => (ori + 1) as isize,
            };
            let des: usize = match des {
                x if x < 0 => 0,
                x if x >= length => program.instructions.len() - 1,
                x => x as usize,
            };
            (ori, des)
        })
        .for_each(|(origem, destino)| {
            let des = destinations.get_mut(&destino);
            if let Some(set) = des {
                set.insert(origem);
            } else {
                let mut set = HashSet::new();
                set.insert(origem);
                destinations.insert(destino, set);
            }
        });
    let mut end_points = HashSet::new();
    let mut nodes_left = vec![];
    nodes_left.push(program.instructions.len() - 1);
    while let Some(x) = nodes_left.pop() {
        end_points.insert(x);
        destinations.get(&x).iter().for_each(|e| {
            e.iter().for_each(|d| {
                if !end_points.contains(d) {
                    nodes_left.push(*d);
                }
            })
        });
    }
    end_points
}

impl Cpu {
    fn tick<'a>(&mut self, program: &'a Program) -> Option<&'a OpCode> {
        let op = self.get_instruction(program)?;
        self.accumulator = self.calcualte_accu(op);
        self.program_counter = self.calculate_destination(op);
        Some(op)
    }

    fn get_instruction<'a>(&self, program: &'a Program) -> Option<&'a OpCode> {
        program.instructions.get(self.program_counter)
    }

    fn calcualte_accu(&self, op: &OpCode) -> isize {
        if let OpCode::Acc(i) = op {
            self.accumulator + i
        } else {
            self.accumulator
        }
    }

    fn calculate_destination(&self, op: &OpCode) -> usize {
        match op {
            OpCode::Acc(_) | OpCode::Nop(_) => self.program_counter + 1,
            OpCode::Jmp(i) => if i.is_negative() {
                self.program_counter
                    .checked_sub(i.wrapping_abs() as usize)
                    .or(Some(0))
            } else {
                self.program_counter.checked_add(*i as usize)
            }
            .unwrap(),
        }
    }

    // Run the program and return if found loop and the last accumulator
    fn run_program(mut self, program: &Program) -> (Option<()>, isize) {
        let mut instruction_viewed: HashSet<usize> = HashSet::new();
        loop {
            if !instruction_viewed.insert(self.program_counter) {
                break (Some(()), self.accumulator);
            }
            if self.tick(program).is_none() {
                break (None, self.accumulator);
            }
        }
    }
}

impl OpCode {
    const fn change(&self) -> Option<Self> {
        match self {
            Self::Jmp(x) => Some(Self::Nop(*x)),
            Self::Nop(x) => Some(Self::Jmp(*x)),
            Self::Acc(_) => None,
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
        let result = Cpu::default().run_program(&program);
        assert_eq!(result, (Some(()), 5));
    }

    #[test]
    fn test_example_2() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\nnop -4\nacc +6";
        let program = parse_program(input).unwrap();
        let result = Cpu::default().run_program(&program);
        assert_eq!(result, (None, 8));
    }
}
