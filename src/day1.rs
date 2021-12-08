use std::fs::read_to_string;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::multi::many0;
use crate::Error;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day1")?)?;

    Ok((part1(&input)?, part2()?))
}

fn my_u64(input : &str) -> IResult<&str, u32> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data))
}


fn multi(i: &str) -> IResult<&str, Vec<u32>> {
    many0(my_u64)(i)
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[u32]) -> Result<String, Error> {
    let mut max = None;
    let mut num_increases = 0;

    for i in input {
        match max {
            None => { },
            Some(old) => {
                if i > old {
                    num_increases += 1;
                }
            }
        }
        max = Some(i)
    }

    Ok(format!("{}", num_increases))
}

fn part2() -> Result<String, Error> {
    Ok("".to_owned())
}

#[test]
pub fn test_parse() {
    let res = parse_input("199
200
208
210
200
207
240
269
260
263
");

    assert_eq!(vec![199,200,208,210,200,207,240,269,260,263], res.unwrap());
}

#[test]
pub fn test_part1() {
    assert_eq!("7", part1(&vec![199,200,208,210,200,207,240,269,260,263]).unwrap());
}