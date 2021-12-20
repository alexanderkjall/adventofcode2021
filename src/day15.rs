use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day15")?)?;

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

fn neighbours(
    node: &(usize, usize),
    goal: &(usize, usize),
    h: &[Vec<u8>],
) -> Vec<((usize, usize), u32)> {
    let mut ret = vec![];
    if node.0 > 0 {
        ret.push(((node.0 - 1, node.1), h[node.0 - 1][node.1] as u32));
    }
    if node.1 > 0 {
        ret.push(((node.0, node.1 - 1), h[node.0][node.1 - 1] as u32));
    }
    if node.0 < goal.0 {
        ret.push(((node.0 + 1, node.1), h[node.0 + 1][node.1] as u32));
    }
    if node.1 < goal.1 {
        ret.push(((node.0, node.1 + 1), h[node.0][node.1 + 1] as u32));
    }
    ret
}

fn reverse_path<N: Eq + Hash + Clone>(parents: &HashMap<N, N>, start: N) -> Vec<N> {
    let mut path = vec![start];
    while let Some(parent) = parents.get(path.last().unwrap()).cloned() {
        path.push(parent);
    }
    path.into_iter().rev().collect()
}

struct SmallestCostHolder<K, P> {
    estimated_cost: K,
    cost: K,
    payload: P,
}

impl<K: PartialEq, P> PartialEq for SmallestCostHolder<K, P> {
    fn eq(&self, other: &SmallestCostHolder<K, P>) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<K: PartialEq, P> Eq for SmallestCostHolder<K, P> {}

impl<K: Ord, P> PartialOrd for SmallestCostHolder<K, P> {
    fn partial_cmp(&self, other: &SmallestCostHolder<K, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, P> Ord for SmallestCostHolder<K, P> {
    fn cmp(&self, other: &SmallestCostHolder<K, P>) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

fn a_star(start: (usize, usize), goal: (usize, usize), h: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let heuristic = |(_, _)| 0u32;

    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: heuristic(start),
        cost: 0,
        payload: (0, start),
    });
    let mut parents: HashMap<(usize, usize), ((usize, usize), u32)> = HashMap::new();
    while let Some(SmallestCostHolder {
        payload: (cost, node),
        ..
    }) = to_see.pop()
    {
        if node == goal {
            let parents = parents.into_iter().map(|(n, (p, _))| (n, p)).collect();
            return reverse_path(&parents, node);
        }
        // We may have inserted a node several time into the binary heap if we found
        // a better way to access it. Ensure that we are currently dealing with the
        // best path and discard the others.
        if let Some(&(_, c)) = parents.get(&node) {
            if cost > c {
                continue;
            }
        }
        for (neighbour, move_cost) in neighbours(&node, &goal, h) {
            let new_cost = cost + move_cost;
            if neighbour != start {
                let mut inserted = true;
                match parents.entry(neighbour) {
                    Vacant(e) => {
                        e.insert((node, new_cost));
                    }
                    Occupied(mut e) => {
                        if e.get().1 > new_cost {
                            e.insert((node, new_cost));
                        } else {
                            inserted = false;
                        }
                    }
                };
                if inserted {
                    let new_predicted_cost = new_cost + heuristic(neighbour);
                    to_see.push(SmallestCostHolder {
                        estimated_cost: new_predicted_cost,
                        cost,
                        payload: (new_cost, neighbour),
                    });
                }
            }
        }
    }

    // Open set is empty but goal was never reached
    return vec![];
}

fn part1(input: &[Vec<u8>]) -> Result<String, Error> {
    let route = a_star((0, 0), (input.len() - 1, input[0].len() - 1), input);

    Ok(format!(
        "{}",
        route
            .iter()
            .skip(1)
            .map(|(x, y)| input[*x][*y] as u32)
            .sum::<u32>()
    ))
}

fn five_times_map(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let x_max = input.len();
    let y_max = input[0].len();
    let mut new = vec![vec![0u8; y_max * 5]; x_max * 5];

    for x in 0..x_max * 5 {
        for y in 0..y_max * 5 {
            let new_val = input[x % x_max][y % y_max] + (x / x_max + y / y_max) as u8;
            if new_val > 9 {
                new[x][y] = (new_val % 10) + 1;
            } else {
                new[x][y] = new_val;
            }
        }
    }

    new
}

fn part2(input: &[Vec<u8>]) -> Result<String, Error> {
    let input = five_times_map(input);
    let route = a_star((0, 0), (input.len() - 1, input[0].len() - 1), &input);

    Ok(format!(
        "{}",
        route
            .iter()
            .skip(1)
            .map(|(x, y)| input[*x][*y] as u32)
            .sum::<u32>()
    ))
}

#[cfg(test)]
mod tests {
    use crate::day15::five_times_map;
    use crate::day15::parse_input;
    use crate::day15::part1;
    use crate::day15::part2;

    fn test_input() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    }

    #[test]
    pub fn test_five_times_map() {
        let map = five_times_map(&test_input());

        let expected = vec![
            vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9,
                6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6,
            ],
            vec![
                1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5, 9, 5, 8,
                9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6,
            ],
            vec![
                2, 1, 3, 6, 5, 1, 1, 3, 2, 8, 3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7, 3, 3, 5,
                4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3,
            ],
            vec![
                3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2, 5, 3, 7,
                8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4,
            ],
            vec![
                7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6, 3, 9, 3,
                3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5,
            ],
            vec![
                1, 3, 1, 9, 1, 2, 8, 1, 3, 7, 2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3, 4, 1, 3,
                5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2,
            ],
            vec![
                1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2, 3, 4, 6,
                4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5,
            ],
            vec![
                3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6, 4, 3, 8,
                5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4,
            ],
            vec![
                1, 2, 9, 3, 1, 3, 8, 5, 2, 1, 2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3, 5, 1, 7,
                4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5,
            ],
            vec![
                2, 3, 1, 1, 9, 4, 4, 5, 8, 1, 3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2, 6, 6, 7,
                1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5,
            ],
            vec![
                2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1,
                7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7,
            ],
            vec![
                2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9,
                1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7,
            ],
            vec![
                3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6,
                5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4,
            ],
            vec![
                4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8,
                9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5,
            ],
            vec![
                8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4,
                4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6,
            ],
            vec![
                2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4,
                6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3,
            ],
            vec![
                2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7,
                5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6,
            ],
            vec![
                4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9,
                6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5,
            ],
            vec![
                2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8,
                5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6,
            ],
            vec![
                3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8,
                2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6,
            ],
            vec![
                3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2,
                8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8,
            ],
            vec![
                3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1,
                2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8,
            ],
            vec![
                4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7,
                6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5,
            ],
            vec![
                5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9,
                1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6,
            ],
            vec![
                9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5,
                5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7,
            ],
            vec![
                3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5,
                7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4,
            ],
            vec![
                3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8,
                6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7,
            ],
            vec![
                5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1,
                7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6,
            ],
            vec![
                3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9,
                6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7,
            ],
            vec![
                4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9,
                3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7,
            ],
            vec![
                4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3,
                9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9,
            ],
            vec![
                4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2,
                3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9,
            ],
            vec![
                5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8,
                7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6,
            ],
            vec![
                6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1,
                2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7,
            ],
            vec![
                1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6,
                6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8,
            ],
            vec![
                4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6,
                8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5,
            ],
            vec![
                4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9,
                7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8,
            ],
            vec![
                6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2,
                8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7,
            ],
            vec![
                4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1,
                7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8,
            ],
            vec![
                5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1,
                4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8,
            ],
            vec![
                5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4,
                1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9, 9, 9, 5, 2, 6, 4, 9, 6, 3, 1,
            ],
            vec![
                5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3,
                4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9, 9, 2, 7, 9, 2, 6, 2, 5, 6, 1,
            ],
            vec![
                6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9,
                8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6, 1, 9, 2, 5, 4, 9, 9, 2, 1, 7,
            ],
            vec![
                7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2,
                3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7, 2, 5, 8, 3, 8, 2, 9, 4, 5, 8,
            ],
            vec![
                2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7,
                7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8, 6, 3, 5, 2, 3, 9, 6, 9, 9, 9,
            ],
            vec![
                5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7,
                9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5, 9, 2, 9, 8, 9, 1, 7, 9, 2, 6,
            ],
            vec![
                5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1,
                8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8, 9, 2, 4, 8, 8, 9, 1, 3, 1, 9,
            ],
            vec![
                7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3,
                9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7, 2, 9, 1, 4, 3, 1, 9, 5, 2, 8,
            ],
            vec![
                5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2,
                8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8, 9, 1, 8, 2, 9, 2, 7, 4, 1, 9,
            ],
            vec![
                6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2,
                5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8, 1, 2, 9, 9, 8, 3, 3, 4, 7, 9,
            ],
        ];

        assert_eq!(expected, map);
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
",
        );

        assert_eq!(test_input(), res.unwrap());
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("40", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("315", part2(&test_input()).unwrap());
    }
}
