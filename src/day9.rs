use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day9")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn my_u32(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, data) = digit1(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((
        rest,
        data.chars()
            .map(|d| match d {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => panic!("wrong input"),
            })
            .collect::<Vec<u8>>(),
    ))
}

fn multi(i: &str) -> IResult<&str, Vec<Vec<u8>>> {
    many0(my_u32)(i)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[Vec<u8>]) -> Result<String, Error> {
    let x_max = input.len();
    let y_max = input[0].len();

    let mut results = vec![];
    for x in 0..x_max {
        for y in 0..y_max {
            let mut smallest = true;
            if x > 0 && input[x - 1][y] <= input[x][y] {
                smallest = false;
            }
            if y > 0 && input[x][y - 1] <= input[x][y] {
                smallest = false;
            }
            if x < x_max - 1 && y < y_max && input[x + 1][y] <= input[x][y] {
                smallest = false;
            }
            if x < x_max && y < y_max - 1 && input[x][y + 1] <= input[x][y] {
                smallest = false;
            }

            if smallest {
                results.push(input[x][y]);
            }
        }
    }

    Ok(format!(
        "{}",
        results.iter().fold(0u32, |s, v| s + (*v as u32) + 1)
    ))
}

fn part2(_input: &[Vec<u8>]) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "2199943210
3987894921
9856789892
8767896789
9899965678
",
    );

    assert_eq!(
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ],
        res.unwrap()
    );
}

#[test]
pub fn test_part1() {
    assert_eq!(
        "15",
        part1(&vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ])
        .unwrap()
    );
}

#[test]
pub fn test_part2() {
    assert_eq!(
        "",
        part2(&vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ])
        .unwrap()
    );
}
