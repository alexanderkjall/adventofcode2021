use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day6")?)?;

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

fn calc(init: u32) -> u32 {
    let mut state = vec![init];

    for _ in 1..=80 {
        let mut to_expand = vec![];
        for fish in &mut state {
            if *fish == 0 {
                *fish = 6;
                to_expand.push(8);
            } else {
                *fish -= 1;
            }
        }
        state.extend(to_expand);
    }

    state.len() as u32
}

fn precompute() -> [u32; 6] {
    [calc(0), calc(1), calc(2), calc(3), calc(4), calc(5)]
}

fn part1(input: &[u32]) -> Result<String, Error> {
    let lookup = precompute();

    let result = input.iter().fold(0u32, |s, n| s + lookup[*n as usize]);

    Ok(format!("{}", result))
}

fn part2(_input: &[u32]) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
pub fn test_parse() {
    let res = parse_input("3,4,3,1,2\n");

    assert_eq!(vec![3, 4, 3, 1, 2], res.unwrap());
}

#[test]
pub fn test_part1() {
    assert_eq!("5934", part1(&vec![3, 4, 3, 1, 2]).unwrap());
}

#[test]
pub fn test_part2() {
    assert_eq!("", part2(&vec![3, 4, 3, 1, 2]).unwrap());
}
