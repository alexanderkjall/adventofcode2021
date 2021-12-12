use std::fs::read_to_string;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::multi::many0;
use crate::Error;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day2")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

enum Direction {
    UP,
    DOWN,
    FORWARD
}

impl TryFrom<&str> for Direction {
    type Error = crate::Error;

    fn try_from(i: &str) -> Result<Self, Self::Error> {
        Ok(match i.to_lowercase().as_str() {
            "up" => Direction::UP,
            "down" => Direction::DOWN,
            "forward" => Direction::FORWARD,
            _ => return Err(Error::GenericDyn(format!("unknown word {}", i))),
        })
    }
}

fn my_i32_pair(input : &str) -> IResult<&str, (i32, i32)> {
    let (rest, direction) = map_res(recognize(alpha1), Direction::try_from)(input)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, amount) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    match direction {
        Direction::UP => Ok((rest, (0, amount))),
        Direction::DOWN => Ok((rest, (0, -amount))),
        Direction::FORWARD => Ok((rest, (amount, 0))),
    }
}

fn multi(i: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(my_i32_pair)(i)
}

fn parse_input(input: &str) -> Result<Vec<(i32, i32)>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[(i32, i32)]) -> Result<String, Error> {
    let sum = input.iter().fold((0i32, 0i32), |sum, val| (sum.0 + val.0, sum.1 + val.1));

    Ok(format!("{}", sum.0.abs() * sum.1.abs()))
}

fn part2(input: &[(i32, i32)]) -> Result<String, Error> {
    let sum = input.iter().fold((0i32, 0i32, 0i32), |sum, val| {
        let aim = sum.2 + val.1;
        (sum.0 + val.0, sum.1 + (aim * val.0), aim)
    });

    Ok(format!("{}", sum.0.abs() * sum.1.abs()))
}

#[test]
pub fn test_parse() {
    let res = parse_input("forward 5
down 5
forward 8
up 3
down 8
forward 2
");

    assert_eq!(vec![(5, 0), (0, -5), (8, 0), (0, 3), (0, -8), (2, 0)], res.unwrap());
}

#[test]
pub fn test_part1() {
    assert_eq!("150", part1(&vec![(5, 0), (0, -5), (8, 0), (0, 3), (0, -8), (2, 0)]).unwrap());
}

#[test]
pub fn test_part2() {
    assert_eq!("900", part2(&vec![(5, 0), (0, -5), (8, 0), (0, 3), (0, -8), (2, 0)]).unwrap());
}
