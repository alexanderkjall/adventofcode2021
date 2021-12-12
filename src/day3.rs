use crate::Error;
use gmp::mpz::Mpz;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day3")?)?;

    Ok((part1(&(&input.0, input.1))?, part2(&(&input.0, input.1))?))
}

fn from_binary(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn my_u64(input: &str) -> IResult<&str, Vec<char>> {
    let (rest, data) = map(recognize(digit1), from_binary)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data))
}

fn multi(i: &str) -> IResult<&str, Vec<Vec<char>>> {
    many0(my_u64)(i)
}

fn parse_input(input: &str) -> Result<(Vec<(Mpz, usize)>, usize), Error> {
    let (_, data) = multi(input)?;

    let mut transposed: Vec<String> = vec!["".to_owned(); data[0].len()];
    for r in &data {
        for (i, c) in r.iter().enumerate() {
            transposed[i].push(*c);
        }
    }

    Ok((
        transposed
            .iter()
            .map(|s| (gmp::mpz::Mpz::from_str_radix(s, 2).unwrap(), s.len()))
            .collect(),
        transposed.len(),
    ))
}

fn part1(input: &(&[(Mpz, usize)], usize)) -> Result<String, Error> {
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;

    for (i, (m, s)) in input.0.iter().enumerate() {
        if m.popcount() > (s / 2) {
            gamma += 2u64.pow((input.1 - i - 1).try_into().unwrap());
        } else {
            epsilon += 2u64.pow((input.1 - i - 1).try_into().unwrap());
        }
    }
    Ok(format!("{}", gamma * epsilon))
}

fn part2(_input: &(&[(Mpz, usize)], usize)) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
",
    );

    assert_eq!(
        (
            vec![
                (Mpz::from(1948), 12),
                (Mpz::from(1109), 12),
                (Mpz::from(4080), 12),
                (Mpz::from(1891), 12),
                (Mpz::from(484), 12)
            ],
            5
        ),
        res.unwrap()
    );
}

#[test]
pub fn test_part1() {
    assert_eq!(
        "198",
        part1(&(
            &vec![
                (Mpz::from(1948), 12),
                (Mpz::from(1109), 12),
                (Mpz::from(4080), 12),
                (Mpz::from(1891), 12),
                (Mpz::from(484), 12)
            ],
            5
        ))
        .unwrap()
    );
}
