use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day4")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(Clone)]
struct Board {
    numbers: [u8; 25],
    marks: [bool; 25],
}

impl Board {
    fn mark_number(&mut self, num: u8) -> Option<u32> {
        for (i, v) in self.numbers.iter().enumerate() {
            if num == *v {
                self.marks[i] = true;

                let mut vertical = true;
                let mut horizontal = true;
                let x = i % 5;
                let y = i / 5;
                for index in 0..=4 {
                    if !self.marks[x + index * 5] {
                        vertical = false;
                    }
                    if !self.marks[index + y * 5] {
                        horizontal = false;
                    }
                }

                if vertical || horizontal {
                    let score: u32 = self
                        .numbers
                        .iter()
                        .enumerate()
                        .map(|(i, v)| if self.marks[i] { 0 } else { *v as u32 })
                        .sum();
                    return Some(score * (*v as u32));
                } else {
                    return None;
                }
            }
        }

        None
    }
}

fn u8_comma(input: &str) -> IResult<&str, u8> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = alt((tag(","), tag("")))(rest)?;

    Ok((rest, data))
}

fn read_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    many0(u8_comma)(input)
}

fn u8_space(input: &str) -> IResult<&str, u8> {
    let (rest, data) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = alt((tag("  "), tag(" "), tag("\n "), tag("\n"), tag("")))(rest)?;

    Ok((rest, data))
}

fn read_board_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    many0(u8_space)(input)
}

fn board(input: &str) -> IResult<&str, Board> {
    let (rest, _) = alt((tag("\n "), tag("\n\n "), tag("\n\n"), tag("\n")))(input)?;
    let (rest, numbers) = read_board_numbers(rest)?;

    Ok((
        rest,
        Board {
            numbers: numbers.try_into().unwrap(),
            marks: [false; 25],
        },
    ))
}

fn read_boards(input: &str) -> IResult<&str, Vec<Board>> {
    many0(board)(input)
}

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<Board>), Error> {
    let (rest, numbers) = read_numbers(input)?;
    let (rest, _) = tag("\n")(rest)?;
    let (_, boards) = read_boards(rest)?;

    Ok((numbers, boards))
}

fn part1(input: &(Vec<u8>, Vec<Board>)) -> Result<String, Error> {
    let mut boards = input.1.clone();
    for num in &input.0 {
        for board in &mut boards {
            if let Some(score) = board.mark_number(*num) {
                return Ok(format!("{}", score));
            }
        }
    }

    Err(Error::Generic("no winner"))
}

fn part2(_input: &(Vec<u8>, Vec<Board>)) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
fn test_u8() {
    let (_, numbers) = u8_comma("32").unwrap();

    assert_eq!(32, numbers);
}

#[test]
fn test_u8_comma() {
    let (_, numbers) = u8_comma("32,").unwrap();

    assert_eq!(32, numbers);
}

#[test]
fn test_board() {
    let (_, board) = board(
        "
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19",
    )
    .unwrap();

    assert_eq!(
        vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19
        ],
        board.numbers
    );
}

#[test]
fn test_parse_input() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    let result = parse_input(input).unwrap();

    assert_eq!(
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1
        ],
        result.0
    );

    assert_eq!(
        vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19
        ],
        result.1[0].numbers
    );

    assert_eq!(
        vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6
        ],
        result.1[1].numbers
    );

    assert_eq!(
        vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7
        ],
        result.1[2].numbers
    );
}

#[test]
fn test_part1() {
    let input = (
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ],
        vec![
            Board {
                numbers: [
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                marks: [false; 25],
            },
            Board {
                numbers: [
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6,
                ],
                marks: [false; 25],
            },
            Board {
                numbers: [
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
                marks: [false; 25],
            },
        ],
    );

    let res = part1(&input).unwrap();

    assert_eq!("4512", res);
}
