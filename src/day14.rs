use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::anychar;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day14")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Debug)]
struct State {
    start: Vec<char>,
    templates: HashMap<(char, char), char>,
}

#[derive(PartialEq, Debug)]
struct Template {
    source: (char, char),
    result: char,
}

fn template(input: &str) -> IResult<&str, Template> {
    let (rest, s1) = anychar(input)?;
    let (rest, s2) = anychar(rest)?;
    let (rest, _) = tag(" -> ")(rest)?;
    let (rest, r) = anychar(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((
        rest,
        Template {
            source: (s1, s2),
            result: r,
        },
    ))
}

fn multi_template(i: &str) -> IResult<&str, Vec<Template>> {
    many0(template)(i)
}

fn parse_input(input: &str) -> Result<State, Error> {
    let (rest, start_state) = alpha1(input)?;
    let (rest, _) = tag("\n\n")(rest)?;
    let (_, templates) = multi_template(rest)?;

    let mut map = HashMap::new();
    for t in templates {
        map.insert(t.source, t.result);
    }
    Ok(State {
        start: start_state.chars().collect(),
        templates: map,
    })
}

fn expand_state(
    state: &HashMap<(char, char), u64>,
    templates: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_state = HashMap::new();

    for (k, v) in state {
        let to_insert = templates.get(k).unwrap();

        let counter = new_state.entry((k.0, *to_insert)).or_insert(0);
        *counter += v;
        let counter = new_state.entry((*to_insert, k.1)).or_insert(0);
        *counter += v;
    }

    new_state
}

fn calc(state: &HashMap<(char, char), u64>, start: (char, char)) -> u64 {
    let mut map = HashMap::new();

    for (k, v) in state {
        let counter = map.entry(k.0).or_insert(0);
        *counter += v;
        let counter = map.entry(k.1).or_insert(0);
        *counter += v;
    }
    let counter = map.entry(start.0).or_insert(0);
    *counter += 1;
    let counter = map.entry(start.1).or_insert(0);
    *counter += 1;

    let mut max = 0;
    let mut min = u64::MAX;

    for v in map.values() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }

    (max - min) / 2
}

fn precalc(input: &State) -> HashMap<(char, char), u64> {
    let mut state = HashMap::new();
    for i in 0..(input.start.len() - 1) {
        let counter = state
            .entry((input.start[i], input.start[i + 1]))
            .or_insert(0);
        *counter += 1;
    }
    state
}

fn part1(input: &State) -> Result<String, Error> {
    let mut state = precalc(input);
    for _ in 0..10 {
        state = expand_state(&state, &input.templates);
    }

    Ok(format!(
        "{}",
        calc(
            &state,
            (*input.start.get(0).unwrap(), *input.start.last().unwrap())
        )
    ))
}

fn part2(input: &State) -> Result<String, Error> {
    let mut state = precalc(input);
    for _ in 0..40 {
        state = expand_state(&state, &input.templates);
    }

    Ok(format!(
        "{}",
        calc(
            &state,
            (*input.start.get(0).unwrap(), *input.start.last().unwrap())
        )
    ))
}

#[cfg(test)]
mod tests {
    use crate::day14::parse_input;
    use crate::day14::part1;
    use crate::day14::part2;
    use crate::day14::State;
    use crate::day14::Template;
    use std::collections::HashMap;

    fn test_input() -> State {
        let templates = vec![
            Template {
                source: ('C', 'H'),
                result: 'B',
            },
            Template {
                source: ('H', 'H'),
                result: 'N',
            },
            Template {
                source: ('C', 'B'),
                result: 'H',
            },
            Template {
                source: ('N', 'H'),
                result: 'C',
            },
            Template {
                source: ('H', 'B'),
                result: 'C',
            },
            Template {
                source: ('H', 'C'),
                result: 'B',
            },
            Template {
                source: ('H', 'N'),
                result: 'C',
            },
            Template {
                source: ('N', 'N'),
                result: 'C',
            },
            Template {
                source: ('B', 'H'),
                result: 'H',
            },
            Template {
                source: ('N', 'C'),
                result: 'B',
            },
            Template {
                source: ('N', 'B'),
                result: 'B',
            },
            Template {
                source: ('B', 'N'),
                result: 'B',
            },
            Template {
                source: ('B', 'B'),
                result: 'N',
            },
            Template {
                source: ('B', 'C'),
                result: 'B',
            },
            Template {
                source: ('C', 'C'),
                result: 'N',
            },
            Template {
                source: ('C', 'N'),
                result: 'C',
            },
        ];

        let mut map = HashMap::new();
        for t in templates {
            map.insert(t.source, t.result);
        }
        State {
            start: vec!['N', 'N', 'C', 'B'],
            templates: map,
        }
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
",
        );

        assert_eq!(test_input(), res.unwrap());
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("1588", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("2188189693529", part2(&test_input()).unwrap());
    }
}
