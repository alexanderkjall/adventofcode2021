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

fn part2(_input: &[Vec<u8>]) -> Result<String, Error> {
    Ok(format!(""))
}

#[cfg(test)]
mod tests {
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
        assert_eq!("", part2(&test_input()).unwrap());
    }
}
