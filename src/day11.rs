use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day11")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn my_u32(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, data) = digit1(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((
        rest,
        data.chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .collect(),
    ))
}

fn multi(i: &str) -> IResult<&str, Vec<Vec<u8>>> {
    many0(my_u32)(i)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn step(state: &mut [Vec<u8>]) -> u32 {
    let mut flashes = [[false; 10]; 10];

    for r in state.iter_mut() {
        for c in r.iter_mut() {
            *c += 1;
        }
    }

    let mut flashed = true;

    while flashed {
        flashed = false;
        for x in 0..10 {
            for y in 0..10 {
                if state[x][y] > 9 && !flashes[x][y] {
                    flashed = true;
                    flashes[x][y] = true;

                    if x > 0 && y > 0 {
                        state[x - 1][y - 1] += 1;
                    }
                    if x > 0 {
                        state[x - 1][y] += 1;
                    }
                    if x > 0 && y < 9 {
                        state[x - 1][y + 1] += 1;
                    }
                    if y > 0 {
                        state[x][y - 1] += 1;
                    }
                    state[x][y] += 1;
                    if y < 9 {
                        state[x][y + 1] += 1;
                    }
                    if x < 9 && y > 0 {
                        state[x + 1][y - 1] += 1;
                    }
                    if x < 9 {
                        state[x + 1][y] += 1;
                    }
                    if x < 9 && y < 9 {
                        state[x + 1][y + 1] += 1;
                    }
                }
            }
        }
    }

    let mut num_flashes = 0;
    for r in state.iter_mut() {
        for c in r.iter_mut() {
            if *c > 9 {
                num_flashes += 1;
                *c = 0;
            }
        }
    }
    num_flashes
}

fn part1(input: &[Vec<u8>]) -> Result<String, Error> {
    let mut state: Vec<Vec<u8>> = input.to_vec();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut state);
    }

    Ok(format!("{}", flashes))
}

fn part2(_input: &[Vec<u8>]) -> Result<String, Error> {
    Ok(format!(""))
}

#[cfg(test)]
mod tests {
    use crate::day11::parse_input;
    use crate::day11::part1;
    use crate::day11::part2;
    use crate::day11::step;

    #[test]
    fn test_step() {
        let mut state = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 9, 9, 9, 1, 0, 0],
            vec![0, 0, 0, 1, 9, 1, 9, 1, 0, 0],
            vec![0, 0, 0, 1, 9, 9, 9, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let num_flashes = step(&mut state);

        assert_eq!(9, num_flashes);

        assert_eq!(
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 3, 4, 5, 4, 3, 1, 1],
                vec![1, 1, 1, 4, 0, 0, 0, 4, 1, 1],
                vec![1, 1, 1, 5, 0, 0, 0, 5, 1, 1],
                vec![1, 1, 1, 4, 0, 0, 0, 4, 1, 1],
                vec![1, 1, 1, 3, 4, 5, 4, 3, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            state
        );
    }

    fn test_input() -> Vec<Vec<u8>> {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
",
        );

        assert_eq!(test_input(), res.unwrap());
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("1656", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("", part2(&test_input()).unwrap());
    }
}
