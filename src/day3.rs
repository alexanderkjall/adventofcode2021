use crate::Error;
use gmp::mpz::Mpz;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input_t = parse_input_transposed(&read_to_string("input/day3")?)?;
    let input = parse_input(&read_to_string("input/day3")?)?;

    Ok((part1(&(&input_t.0, input_t.1))?, part2(&input, input_t.1)?))
}

fn from_str_to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn chars(input: &str) -> IResult<&str, Vec<char>> {
    let (rest, data) = map(recognize(digit1), from_str_to_chars)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data))
}

fn char_grid(i: &str) -> IResult<&str, Vec<Vec<char>>> {
    many0(chars)(i)
}

fn parse_input_transposed(input: &str) -> Result<(Vec<(Mpz, usize)>, usize), Error> {
    let (_, data) = char_grid(input)?;

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

fn from_str_to_u32(input: &str) -> Result<u32, Error> {
    Ok(u32::from_str_radix(input, 2)?)
}

fn row(input: &str) -> IResult<&str, u32> {
    let (rest, data) = map_res(recognize(digit1), from_str_to_u32)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data))
}

fn multi(i: &str) -> IResult<&str, Vec<u32>> {
    many0(row)(i)
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
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

fn part2(input: &[u32], width: usize) -> Result<String, Error> {
    let mut rows: Vec<u32> = input.to_vec();
    let mut o2_generator = 0;
    for i in (0..width).rev() {
        let popcount = rows.iter().filter(|num| *num & 2u32.pow(i.try_into().unwrap()) != 0).count();
        let popcount_inv = rows.len() - popcount;
        rows = rows.iter()
            .filter(|num| {
                if popcount > popcount_inv {
                    *num & 2u32.pow(i.try_into().unwrap()) != 0
                } else if popcount == popcount_inv {
                    *num & 2u32.pow(i.try_into().unwrap()) != 0
                } else {
                    *num & 2u32.pow(i.try_into().unwrap()) == 0
                }
            })
            .map(|n|*n)
            .collect();

        if rows.len() == 1 {
            o2_generator = rows[0];
            break;
        }
    }

    let mut rows: Vec<u32> = input.to_vec();
    let mut co2_scrubber = 0;
    for i in (0..width).rev() {
        let popcount = rows.iter().filter(|num| *num & 2u32.pow(i.try_into().unwrap()) != 0).count();
        let popcount_inv = rows.len() - popcount;
        rows = rows.iter()
            .filter(|num| {
                if popcount > popcount_inv {
                    *num & 2u32.pow(i.try_into().unwrap()) == 0
                } else if popcount == popcount_inv {
                    *num & 2u32.pow(i.try_into().unwrap()) == 0
                } else {
                    *num & 2u32.pow(i.try_into().unwrap()) != 0
                }
            })
            .map(|n|*n)
            .collect();

        if rows.len() == 1 {
            co2_scrubber = rows[0];
            break;
        }
    }

    Ok(format!("{}", o2_generator * co2_scrubber))
}

#[test]
pub fn test_parse_input_transposed() {
    let res = parse_input_transposed(
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
pub fn test_parse_input() {
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

    assert_eq!(vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10], res.unwrap());
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


#[test]
pub fn test_part2() {
    assert_eq!(
        "230",
        part2(&vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10], 5)
        .unwrap()
    );
}
