use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day12")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Debug)]
enum Room {
    Start,
    Large(String),
    Small(String),
    End,
}

fn is_uppercase(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_uppercase() {
            return false;
        }
    }

    true
}

impl From<&str> for Room {
    fn from(s: &str) -> Self {
        match s {
            "start" => Room::Start,
            "end" => Room::End,
            _ => {
                if is_uppercase(s) {
                    Room::Large(s.to_owned())
                } else {
                    Room::Small(s.to_owned())
                }
            }
        }
    }
}

fn get_or_insert(vec: &mut Vec<Room>, room: Room) -> usize {
    for (i, v) in vec.iter().enumerate() {
        if *v == room {
            return i;
        }
    }
    vec.push(room);
    vec.len() - 1
}

#[derive(PartialEq, Debug)]
struct Graph {
    nodes: Vec<Room>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn from_edges(es: Vec<(Room, Room)>) -> Graph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (r1, r2) in es {
            let r1r = get_or_insert(&mut nodes, r1);
            let r2r = get_or_insert(&mut nodes, r2);

            edges.push((r1r, r2r));
        }

        Graph { nodes, edges }
    }

    fn neighbours(&self, pos: usize) -> Vec<usize> {
        let mut ret = vec![];
        for (a, b) in &self.edges {
            if *a == pos {
                ret.push(*b);
            }
            if *b == pos {
                ret.push(*a);
            }
        }
        ret
    }

    fn next_step(&self, pos: usize, mut path: Vec<usize>) -> Vec<Vec<usize>> {
        let room = &self.nodes[pos];

        let mut paths = vec![];
        match room {
            Room::Start => {
                if !path.is_empty() {
                    return vec![];
                }
                path.push(pos);
                for n in self.neighbours(pos) {
                    let p = self.next_step(n, path.clone());
                    if !p.is_empty() {
                        paths.extend_from_slice(&p);
                    }
                }
            }
            Room::End => {
                path.push(pos);
                paths.push(path);
            }
            Room::Large(_) => {
                path.push(pos);
                for n in self.neighbours(pos) {
                    let p = self.next_step(n, path.clone());
                    if !p.is_empty() {
                        paths.extend_from_slice(&p);
                    }
                }
            }
            Room::Small(_) => {
                if path.contains(&pos) {
                    return vec![];
                }
                path.push(pos);
                for n in self.neighbours(pos) {
                    let p = self.next_step(n, path.clone());
                    if !p.is_empty() {
                        paths.extend_from_slice(&p);
                    }
                }
            }
        }
        paths
    }

    fn paths(&self, from: &Room) -> Vec<Vec<usize>> {
        let f_idx = self.nodes.iter().position(|r| r == from).unwrap();

        let mut paths = vec![];

        paths.extend_from_slice(&self.next_step(f_idx, vec![]));

        paths
    }
}

fn room_pair(input: &str) -> IResult<&str, (Room, Room)> {
    let (rest, room1) = alpha1(input)?;
    let (rest, _) = tag("-")(rest)?;
    let (rest, room2) = alpha1(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, (Room::from(room1), Room::from(room2))))
}

fn multi(i: &str) -> IResult<&str, Graph> {
    let (rest, corridors) = many0(room_pair)(i)?;

    Ok((rest, Graph::from_edges(corridors)))
}

fn parse_input(input: &str) -> Result<Graph, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &Graph) -> Result<String, Error> {
    let paths = input.paths(&Room::Start);

    Ok(format!("{}", paths.len()))
}

fn part2(_input: &Graph) -> Result<String, Error> {
    Ok(format!(""))
}

#[cfg(test)]
mod tests {
    use crate::day12::parse_input;
    use crate::day12::part1;
    use crate::day12::part2;
    use crate::day12::room_pair;
    use crate::day12::Graph;
    use crate::day12::Room;

    #[test]
    fn test_room_pair() {
        let (_, corridor) = room_pair("start-end\n").unwrap();

        assert_eq!((Room::Start, Room::End), corridor);
    }

    fn test_input_1() -> Graph {
        Graph::from_edges(vec![
            (Room::Start, Room::Large("A".to_owned())),
            (Room::Start, Room::Small("b".to_owned())),
            (Room::Large("A".to_owned()), Room::Small("c".to_owned())),
            (Room::Large("A".to_owned()), Room::Small("b".to_owned())),
            (Room::Small("b".to_owned()), Room::Small("d".to_owned())),
            (Room::Large("A".to_owned()), Room::End),
            (Room::Small("b".to_owned()), Room::End),
        ])
    }

    fn test_input_2() -> Graph {
        Graph::from_edges(vec![
            (Room::Small("dc".to_owned()), Room::End),
            (Room::Large("HN".to_owned()), Room::Start),
            (Room::Start, Room::Small("kj".to_owned())),
            (Room::Small("dc".to_owned()), Room::Start),
            (Room::Small("dc".to_owned()), Room::Large("HN".to_owned())),
            (Room::Large("LN".to_owned()), Room::Small("dc".to_owned())),
            (Room::Large("HN".to_owned()), Room::End),
            (Room::Small("kj".to_owned()), Room::Small("sa".to_owned())),
            (Room::Small("kj".to_owned()), Room::Large("HN".to_owned())),
            (Room::Small("kj".to_owned()), Room::Small("dc".to_owned())),
        ])
    }

    #[test]
    pub fn test_parse_1() {
        let res = parse_input(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );

        assert_eq!(test_input_1(), res.unwrap());
    }

    #[test]
    pub fn test_parse_2() {
        let res = parse_input(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
",
        );

        assert_eq!(test_input_2(), res.unwrap());
    }

    #[test]
    pub fn test_part1_1() {
        assert_eq!("10", part1(&test_input_1()).unwrap());
    }

    #[test]
    pub fn test_part1_2() {
        assert_eq!("19", part1(&test_input_2()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("", part2(&test_input_1()).unwrap());
    }
}
