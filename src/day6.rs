use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day6")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn my_u64(input: &str) -> IResult<&str, u64> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = alt((tag(","), tag("\n")))(rest)?;

    Ok((rest, data))
}

fn multi(i: &str) -> IResult<&str, Vec<u64>> {
    many0(my_u64)(i)
}

fn parse_input(input: &str) -> Result<Vec<u64>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn calc_rec(init: u64, day: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if day == 0 {
        return 1;
    }

    if init == 0 {
        if !cache.contains_key(&(6, day - 1)) {
            let val = calc_rec(6, day - 1, cache);
            cache.insert((6, day - 1), val);
        }
        if !cache.contains_key(&(8, day - 1)) {
            let val = calc_rec(8, day - 1, cache);
            cache.insert((8, day - 1), val);
        }
        return *cache.get(&(6, day - 1)).unwrap() + *cache.get(&(8, day - 1)).unwrap();
    } else {
        if !cache.contains_key(&(init - 1, day - 1)) {
            let val = calc_rec(init - 1, day - 1, cache);
            cache.insert((init - 1, day - 1), val);
        }
        return *cache.get(&(init - 1, day - 1)).unwrap();
    }
}

fn part1(input: &[u64]) -> Result<String, Error> {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let result = input
        .iter()
        .fold(0u64, |s, n| s + calc_rec(*n, 80, &mut cache));

    Ok(format!("{}", result))
}

fn part2(input: &[u64]) -> Result<String, Error> {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let result = input
        .iter()
        .fold(0u64, |s, n| s + calc_rec(*n, 256, &mut cache));

    Ok(format!("{}", result))
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
    assert_eq!("26984457539", part2(&vec![3, 4, 3, 1, 2]).unwrap());
}
