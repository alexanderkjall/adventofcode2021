use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day1")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn my_u32(input: &str) -> IResult<&str, u32> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = tag("\n")(rest)?;

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
    let mut previous = None;
    let mut num_increases = 0;

    for val in input {
        match previous {
            None => {}
            Some(old) => {
                if val > old {
                    num_increases += 1;
                }
            }
        }
        previous = Some(val)
    }

    Ok(format!("{}", num_increases))
}

fn part2(input: &[u32]) -> Result<String, Error> {
    let mut previous = None;
    let mut num_increases = 0;

    for (i, _) in input.iter().enumerate() {
        if i < 3 {
            continue;
        }

        match previous {
            None => {}
            Some(old) => {
                if input[i - 2] + input[i - 1] + input[i] > old {
                    num_increases += 1;
                }
            }
        }
        previous = Some(input[i - 2] + input[i - 1] + input[i])
    }

    Ok(format!("{}", num_increases))
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "199
200
208
210
200
207
240
269
260
263
",
    );

    assert_eq!(
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
        res.unwrap()
    );
}

#[test]
pub fn test_part1() {
    assert_eq!(
        "7",
        part1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]).unwrap()
    );
}

#[test]
pub fn test_part2() {
    assert_eq!(
        "4",
        part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]).unwrap()
    );
}
