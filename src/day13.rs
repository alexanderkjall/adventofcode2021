use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::many0;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day13")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Debug)]
enum Direction {
    X,
    Y,
}

#[derive(PartialEq, Debug)]
struct Instructions {
    points: Vec<(u32, u32)>,
    folds: Vec<(Direction, u32)>,
}

fn point(input: &str) -> IResult<&str, (u32, u32)> {
    let (rest, x) = map_res(recognize(digit1), str::parse)(input)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, (x, y)))
}

fn multi_points(i: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many0(point)(i)
}

fn fold(input: &str) -> IResult<&str, (Direction, u32)> {
    let (rest, _) = tag("fold along ")(input)?;
    let (rest, direction) = alt((tag("x"), tag("y")))(rest)?;
    let (rest, _) = tag("=")(rest)?;
    let (rest, amount) = map_res(recognize(digit1), str::parse)(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    let direction = if direction == "x" {
        Direction::X
    } else {
        Direction::Y
    };

    Ok((rest, (direction, amount)))
}

fn multi_folds(i: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    many0(fold)(i)
}

fn parse_input(input: &str) -> Result<Instructions, Error> {
    let (rest, points) = multi_points(input)?;
    let (rest, _) = tag("\n")(rest)?;
    let (_, folds) = multi_folds(rest)?;

    Ok(Instructions { points, folds })
}

fn fold_points(points: &[(u32, u32)], dir: &Direction, amount: &u32) -> Vec<(u32, u32)> {
    let mut new_points = HashSet::new();
    for (x, y) in points {
        if *dir == Direction::X {
            match x.cmp(amount) {
                Ordering::Greater => {
                    new_points.insert((amount * 2 - *x, *y));
                }
                Ordering::Less => {
                    new_points.insert((*x, *y));
                }
                Ordering::Equal => {}
            }
        } else {
            match y.cmp(amount) {
                Ordering::Greater => {
                    new_points.insert((*x, amount * 2 - *y));
                }
                Ordering::Less => {
                    new_points.insert((*x, *y));
                }
                Ordering::Equal => {}
            }
        }
    }
    new_points.into_iter().collect()
}

fn plot_grid(input: &[(u32, u32)]) -> String {
    let max_x = input.iter().map(|(x, _)| x).max().unwrap() + 2;
    let max_y = input.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut chars = vec!['.'; (max_x * max_y) as usize];

    for (x, y) in input {
        chars[(x + max_x * (y)) as usize] = '#';
    }

    for i in 0..max_y {
        chars[((i + 1) * max_x - 1) as usize] = '\n';
    }

    chars.into_iter().collect()
}

fn part1(input: &Instructions) -> Result<String, Error> {
    if let Some((dir, amount)) = input.folds.get(0) {
        let new_points = fold_points(&input.points, dir, amount);
        Ok(format!("{}", new_points.len()))
    } else {
        Err(Error::Generic("no folds in input"))
    }
}

fn part2(input: &Instructions) -> Result<String, Error> {
    let mut points = input.points.clone();
    for (dir, amount) in &input.folds {
        let new_points = fold_points(&points, dir, amount);
        points.resize(0, (0, 0));
        points.extend_from_slice(&new_points);
    }
    let grid = plot_grid(&points);
    //println!("{}", grid);
    Ok(format!("\n{}", grid))
}

#[cfg(test)]
mod tests {
    use crate::day13::parse_input;
    use crate::day13::part1;
    use crate::day13::part2;
    use crate::day13::Direction;
    use crate::day13::Instructions;

    fn test_input() -> Instructions {
        Instructions {
            points: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            folds: vec![(Direction::Y, 7), (Direction::X, 5)],
        }
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
",
        );

        assert_eq!(test_input(), res.unwrap());
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("17", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(
            "#####\n#...#\n#...#\n#...#\n#####\n",
            part2(&test_input()).unwrap()
        );
    }
}
