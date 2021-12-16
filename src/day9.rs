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

fn find_minimum(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
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
            if x < x_max - 1 && input[x + 1][y] <= input[x][y] {
                smallest = false;
            }
            if y < y_max - 1 && input[x][y + 1] <= input[x][y] {
                smallest = false;
            }

            if smallest {
                results.push((x, y));
            }
        }
    }

    results
}

fn is_ridge(x: usize, y: usize, input: &[Vec<u8>]) -> bool {
    input[x][y] == 9
}

fn find_basin_size(x: usize, y: usize, input: &[Vec<u8>]) -> usize {
    let x_max = input.len();
    let y_max = input[0].len();

    let mut included = vec![];
    let mut to_check = vec![];
    let mut blockers = vec![];

    to_check.push((x, y));

    while !to_check.is_empty() {
        let (x, y) = to_check.pop().unwrap();
        if !is_ridge(x, y, input) {
            included.push((x, y));

            if x > 0
                && !blockers.contains(&(x - 1, y))
                && !included.contains(&(x - 1, y))
                && !to_check.contains(&(x - 1, y))
            {
                to_check.push((x - 1, y));
            }
            if y > 0
                && !blockers.contains(&(x, y - 1))
                && !included.contains(&(x, y - 1))
                && !to_check.contains(&(x, y - 1))
            {
                to_check.push((x, y - 1));
            }
            if x < x_max - 1
                && !blockers.contains(&(x + 1, y))
                && !included.contains(&(x + 1, y))
                && !to_check.contains(&(x + 1, y))
            {
                to_check.push((x + 1, y));
            }
            if y < y_max - 1
                && !blockers.contains(&(x, y + 1))
                && !included.contains(&(x, y + 1))
                && !to_check.contains(&(x, y + 1))
            {
                to_check.push((x, y + 1));
            }
        } else {
            blockers.push((x, y));
        }
    }

    included.len()
}

fn part1(input: &[Vec<u8>]) -> Result<String, Error> {
    Ok(format!(
        "{}",
        find_minimum(input)
            .iter()
            .map(|(x, y)| input[*x][*y])
            .fold(0u32, |s, v| s + (v as u32) + 1)
    ))
}

fn part2(input: &[Vec<u8>]) -> Result<String, Error> {
    let mut res: Vec<usize> = find_minimum(input)
        .iter()
        .map(|(x, y)| find_basin_size(*x, *y, input))
        .collect();
    res.sort_by(|a, b| b.cmp(a));
    let res = res.iter().take(3).fold(1u32, |s, v| s * *v as u32);
    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use crate::day9::is_ridge;
    use crate::day9::parse_input;
    use crate::day9::part1;
    use crate::day9::part2;

    fn input() -> Vec<Vec<u8>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
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

        assert_eq!(input(), res.unwrap());
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("15", part1(&input()).unwrap());
    }

    #[test]
    pub fn test_is_ridge() {
        assert_eq!(true, is_ridge(0, 2, &input()));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("1134", part2(&input()).unwrap());
    }
}
