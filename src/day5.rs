use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day5")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Line {
    fn points(&self) -> LineIter {
        LineIter {
            last_point: None,
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
        }
    }
}

struct LineIter {
    last_point: Option<Point>,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(lp) = &self.last_point {
            if lp.x == self.x2 && lp.y == self.y2 {
                return None;
            }

            let mut x = lp.x;
            let mut y = lp.y;

            match x.cmp(&self.x2) {
                Ordering::Greater => x -= 1,
                Ordering::Less => x += 1,
                Ordering::Equal => {}
            }

            match y.cmp(&self.y2) {
                Ordering::Greater => y -= 1,
                Ordering::Less => y += 1,
                Ordering::Equal => {}
            }

            self.last_point = Some(Point { x, y });
        } else {
            self.last_point = Some(Point {
                x: self.x1,
                y: self.y1,
            });
        }
        self.last_point.clone()
    }
}

fn line(input: &str) -> IResult<&str, Line> {
    let (rest, x1) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y1) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag(" -> ")(rest)?;
    let (rest, x2) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y2) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, Line { x1, y1, x2, y2 }))
}

fn multi(i: &str) -> IResult<&str, Vec<Line>> {
    many0(line)(i)
}

fn parse_input(input: &str) -> Result<Vec<Line>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[Line]) -> Result<String, Error> {
    let straight: Vec<&Line> = input
        .iter()
        .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
        .collect();

    let mut points: HashMap<Point, u32> = HashMap::new();

    straight.iter().for_each(|l| {
        l.points().for_each(|p| {
            if let Some(v) = points.get_mut(&p) {
                *v += 1;
            } else {
                points.insert(p.clone(), 0);
            }
        })
    });

    Ok(format!("{}", points.values().filter(|v| **v > 0).count()))
}

fn part2(_input: &[Line]) -> Result<String, Error> {
    Ok(format!(""))
}

#[test]
pub fn test_points_horizontal() {
    assert_eq!(
        Line {
            x1: 1,
            y1: 1,
            x2: 4,
            y2: 1
        }
        .points()
        .collect::<Vec<Point>>(),
        vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 4, y: 1 }
        ]
    );
}

#[test]
pub fn test_points_vertical() {
    assert_eq!(
        Line {
            x1: 1,
            y1: 1,
            x2: 1,
            y2: 4
        }
        .points()
        .collect::<Vec<Point>>(),
        vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
            Point { x: 1, y: 4 }
        ]
    );
}

#[test]
pub fn test_points_up_left_down_right() {
    assert_eq!(
        Line {
            x1: 1,
            y1: 1,
            x2: 4,
            y2: 4
        }
        .points()
        .collect::<Vec<Point>>(),
        vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 }
        ]
    );
}

#[test]
pub fn test_points_down_left_up_right() {
    assert_eq!(
        Line {
            x1: 4,
            y1: 1,
            x2: 1,
            y2: 4
        }
        .points()
        .collect::<Vec<Point>>(),
        vec![
            Point { x: 4, y: 1 },
            Point { x: 3, y: 2 },
            Point { x: 2, y: 3 },
            Point { x: 1, y: 4 }
        ]
    );
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
    );

    assert_eq!(
        vec![
            Line {
                x1: 0,
                y1: 9,
                x2: 5,
                y2: 9
            },
            Line {
                x1: 8,
                y1: 0,
                x2: 0,
                y2: 8
            },
            Line {
                x1: 9,
                y1: 4,
                x2: 3,
                y2: 4
            },
            Line {
                x1: 2,
                y1: 2,
                x2: 2,
                y2: 1
            },
            Line {
                x1: 7,
                y1: 0,
                x2: 7,
                y2: 4
            },
            Line {
                x1: 6,
                y1: 4,
                x2: 2,
                y2: 0
            },
            Line {
                x1: 0,
                y1: 9,
                x2: 2,
                y2: 9
            },
            Line {
                x1: 3,
                y1: 4,
                x2: 1,
                y2: 4
            },
            Line {
                x1: 0,
                y1: 0,
                x2: 8,
                y2: 8
            },
            Line {
                x1: 5,
                y1: 5,
                x2: 8,
                y2: 2
            }
        ],
        res.unwrap()
    );
}

#[test]
pub fn test_part1() {
    assert_eq!(
        "5",
        part1(&vec![
            Line {
                x1: 0,
                y1: 9,
                x2: 5,
                y2: 9
            },
            Line {
                x1: 8,
                y1: 0,
                x2: 0,
                y2: 8
            },
            Line {
                x1: 9,
                y1: 4,
                x2: 3,
                y2: 4
            },
            Line {
                x1: 2,
                y1: 2,
                x2: 2,
                y2: 1
            },
            Line {
                x1: 7,
                y1: 0,
                x2: 7,
                y2: 4
            },
            Line {
                x1: 6,
                y1: 4,
                x2: 2,
                y2: 0
            },
            Line {
                x1: 0,
                y1: 9,
                x2: 2,
                y2: 9
            },
            Line {
                x1: 3,
                y1: 4,
                x2: 1,
                y2: 4
            },
            Line {
                x1: 0,
                y1: 0,
                x2: 8,
                y2: 8
            },
            Line {
                x1: 5,
                y1: 5,
                x2: 8,
                y2: 2
            }
        ])
        .unwrap()
    );
}

#[test]
pub fn test_part2() {
    assert_eq!(
        "",
        part2(&vec![
            Line {
                x1: 0,
                y1: 9,
                x2: 5,
                y2: 9
            },
            Line {
                x1: 8,
                y1: 0,
                x2: 0,
                y2: 8
            },
            Line {
                x1: 9,
                y1: 4,
                x2: 3,
                y2: 4
            },
            Line {
                x1: 2,
                y1: 2,
                x2: 2,
                y2: 1
            },
            Line {
                x1: 7,
                y1: 0,
                x2: 7,
                y2: 4
            },
            Line {
                x1: 6,
                y1: 4,
                x2: 2,
                y2: 0
            },
            Line {
                x1: 0,
                y1: 9,
                x2: 2,
                y2: 9
            },
            Line {
                x1: 3,
                y1: 4,
                x2: 1,
                y2: 4
            },
            Line {
                x1: 0,
                y1: 0,
                x2: 8,
                y2: 8
            },
            Line {
                x1: 5,
                y1: 5,
                x2: 8,
                y2: 2
            }
        ])
        .unwrap()
    );
}
