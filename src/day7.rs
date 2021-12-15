use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::cmp::{max, min};
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day7")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn my_u32(input: &str) -> IResult<&str, u32> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = alt((tag(","), tag("\n")))(rest)?;

    Ok((rest, data))
}

fn multi(i: &str) -> IResult<&str, Vec<u32>> {
    many0(my_u32)(i)
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[u32]) -> Result<String, Error> {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    let mid = numbers.len() / 2;

    let target = numbers[mid];
    let result = input
        .iter()
        .fold(0u32, |s, v| s + max(target, *v) - min(target, *v));

    Ok(format!("{}", result))
}

fn part2(_input: &[u32]) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
pub fn test_parse() {
    let res = parse_input("16,1,2,0,4,2,7,1,2,14\n");

    assert_eq!(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], res.unwrap());
}

#[test]
pub fn test_part1() {
    assert_eq!("37", part1(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]).unwrap());
}

#[test]
pub fn test_part2() {
    assert_eq!("", part2(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]).unwrap());
}
