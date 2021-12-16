use crate::Error;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day8")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Debug)]
struct SignalPattern {
    pattern: Vec<u8>,
}

impl SignalPattern {
    fn missing(&self) -> Vec<u8> {
        let mut ret = vec![0, 1, 2, 3, 4, 5, 6];

        for p in &self.pattern {
            if let Some(i) = ret.iter().position(|v| *v == *p) {
                ret.swap_remove(i);
            }
        }

        ret
    }
}

impl From<&str> for SignalPattern {
    fn from(input: &str) -> Self {
        let pattern = input
            .chars()
            .map(|c| match c {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                _ => panic!("unreachable"),
            })
            .collect();

        SignalPattern { pattern }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Digit {
    sections: [u8; 7],
}

impl From<&str> for Digit {
    fn from(input: &str) -> Self {
        let mut digit = Digit { sections: [0; 7] };
        input.chars().for_each(|c| match c {
            'a' => digit.sections[0] = 1,
            'b' => digit.sections[1] = 1,
            'c' => digit.sections[2] = 1,
            'd' => digit.sections[3] = 1,
            'e' => digit.sections[4] = 1,
            'f' => digit.sections[5] = 1,
            'g' => digit.sections[6] = 1,
            _ => panic!("unreachable"),
        });

        digit
    }
}

impl From<&Vec<u8>> for Digit {
    fn from(input: &Vec<u8>) -> Self {
        let mut digit = Digit { sections: [0; 7] };
        input.iter().for_each(|c| match c {
            0 => digit.sections[0] = 1,
            1 => digit.sections[1] = 1,
            2 => digit.sections[2] = 1,
            3 => digit.sections[3] = 1,
            4 => digit.sections[4] = 1,
            5 => digit.sections[5] = 1,
            6 => digit.sections[6] = 1,
            _ => panic!("unreachable"),
        });

        digit
    }
}

impl Copy for Digit {}

impl Digit {
    fn popcount(&self) -> u32 {
        self.sections[0].count_ones()
            + self.sections[1].count_ones()
            + self.sections[2].count_ones()
            + self.sections[3].count_ones()
            + self.sections[4].count_ones()
            + self.sections[5].count_ones()
            + self.sections[6].count_ones()
    }

    fn missing(&self) -> Vec<u8> {
        let mut ret = vec![];
        for (i, v) in self.sections.iter().enumerate() {
            if *v == 0 {
                ret.push(i as u8);
            }
        }
        ret
    }
}

#[derive(PartialEq, Debug)]
struct Row {
    patterns: [SignalPattern; 10],
    digits: [Digit; 4],
}

impl Row {
    fn num_unique_digits(&self) -> u32 {
        let mut sum = 0;
        for d in &self.digits {
            match d.popcount() {
                2 | 3 | 4 | 7 => sum += 1,
                _ => {}
            }
        }
        sum
    }

    fn map(&self) -> [Digit; 10] {
        let mut solutions = [Digit {
            sections: [0, 0, 0, 0, 0, 0, 0],
        }; 10];

        for p in &self.patterns {
            if p.pattern.len() == 2 {
                solutions[1] = Digit::from(&p.pattern);
            }
            if p.pattern.len() == 3 {
                solutions[7] = Digit::from(&p.pattern);
            }
            if p.pattern.len() == 4 {
                solutions[4] = Digit::from(&p.pattern);
            }
            if p.pattern.len() == 7 {
                solutions[8] = Digit::from(&p.pattern);
            }
        }

        solutions[6] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    p.pattern.len() == 6 && solutions[1].sections[p.missing()[0] as usize] == 1
                })
                .unwrap()
                .pattern,
        );
        solutions[9] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    p.pattern.len() == 6 && solutions[4].sections[p.missing()[0] as usize] == 0
                })
                .unwrap()
                .pattern,
        );
        solutions[0] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    if p.pattern.len() != 6 {
                        return false;
                    }
                    let d = Digit::from(&p.pattern);
                    d != solutions[6] && d != solutions[9]
                })
                .unwrap()
                .pattern,
        );
        solutions[3] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    if p.pattern.len() != 5 {
                        return false;
                    }
                    let missing = p.missing();
                    solutions[1].sections[missing[0] as usize] == 0
                        && solutions[1].sections[missing[1] as usize] == 0
                })
                .unwrap()
                .pattern,
        );
        solutions[5] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    if p.pattern.len() != 5 {
                        return false;
                    }
                    let missing = solutions[6].missing();
                    !p.pattern.contains(&missing[0])
                })
                .unwrap()
                .pattern,
        );
        solutions[2] = Digit::from(
            &self
                .patterns
                .iter()
                .find(|p| {
                    if p.pattern.len() != 5 {
                        return false;
                    }
                    let d = Digit::from(&p.pattern);
                    d != solutions[3] && d != solutions[5]
                })
                .unwrap()
                .pattern,
        );

        solutions
    }

    fn digits_to_num(&self, map: [Digit; 10]) -> u32 {
        let mut sum: u32 = 0;

        if map.iter().filter(|v| **v == self.digits[0]).count() == 0 {
            panic!("no match for {:?}", self.digits[0]);
        }
        for (i, v) in map.iter().enumerate() {
            if *v == self.digits[0] {
                sum += 1000 * i as u32;
            }
        }

        for (i, v) in map.iter().enumerate() {
            if *v == self.digits[1] {
                sum += 100 * i as u32;
            }
        }

        for (i, v) in map.iter().enumerate() {
            if *v == self.digits[2] {
                sum += 10 * i as u32;
            }
        }

        for (i, v) in map.iter().enumerate() {
            if *v == self.digits[3] {
                sum += i as u32;
            }
        }

        sum
    }
}

fn my_u32(input: &str) -> IResult<&str, Row> {
    let (rest, signal1) = alpha1(input)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal2) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal3) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal4) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal5) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal6) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal7) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal8) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal9) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, signal10) = alpha1(rest)?;
    let (rest, _) = tag(" | ")(rest)?;
    let (rest, digit1) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, digit2) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, digit3) = alpha1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, digit4) = alpha1(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((
        rest,
        Row {
            patterns: [
                SignalPattern::from(signal1),
                SignalPattern::from(signal2),
                SignalPattern::from(signal3),
                SignalPattern::from(signal4),
                SignalPattern::from(signal5),
                SignalPattern::from(signal6),
                SignalPattern::from(signal7),
                SignalPattern::from(signal8),
                SignalPattern::from(signal9),
                SignalPattern::from(signal10),
            ],
            digits: [
                Digit::from(digit1),
                Digit::from(digit2),
                Digit::from(digit3),
                Digit::from(digit4),
            ],
        },
    ))
}

fn multi(i: &str) -> IResult<&str, Vec<Row>> {
    many0(my_u32)(i)
}

fn parse_input(input: &str) -> Result<Vec<Row>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn part1(input: &[Row]) -> Result<String, Error> {
    Ok(format!(
        "{}",
        input.iter().map(|r| r.num_unique_digits()).sum::<u32>()
    ))
}

fn part2(input: &[Row]) -> Result<String, Error> {
    let sum: u32 = input.iter().map(|r| r.digits_to_num(r.map())).sum();

    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use crate::day8::parse_input;
    use crate::day8::part1;
    use crate::day8::part2;
    use crate::day8::Digit;
    use crate::day8::Row;
    use crate::day8::SignalPattern;

    #[test]
    pub fn test_signal_pattern_missing() {
        let input = SignalPattern {
            pattern: vec![0, 1, 2, 3, 4],
        };
        let expected: Vec<u8> = vec![6, 5];

        assert_eq!(expected, input.missing());
    }

    #[test]
    pub fn test_digit_popcount() {
        assert_eq!(
            5,
            Digit {
                sections: [1, 1, 1, 0, 1, 0, 1]
            }
            .popcount()
        );
    }

    #[test]
    pub fn test_num_unique_digits() {
        let row = Row {
            patterns: [
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
                SignalPattern { pattern: vec![] },
            ],
            digits: [
                Digit {
                    sections: [1, 1, 1, 0, 1, 0, 1],
                },
                Digit {
                    sections: [1, 1, 0, 0, 0, 0, 0],
                },
                Digit {
                    sections: [1, 1, 1, 0, 1, 0, 0],
                },
                Digit {
                    sections: [1, 1, 1, 1, 1, 1, 1],
                },
            ],
        };

        assert_eq!(3, row.num_unique_digits());
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
",
        )
        .unwrap();

        assert_eq!(
            SignalPattern {
                pattern: vec![1, 4]
            },
            res[0].patterns[0]
        );
        assert_eq!(
            Digit {
                sections: [1, 1, 1, 0, 1, 0, 1]
            },
            res[9].digits[3]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("26", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("61229", part2(&test_input()).unwrap());
    }

    fn test_input() -> Vec<Row> {
        vec![
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![1, 4],
                    },
                    SignalPattern {
                        pattern: vec![2, 5, 1, 4, 6, 0, 3],
                    },
                    SignalPattern {
                        pattern: vec![2, 1, 3, 6, 4, 5],
                    },
                    SignalPattern {
                        pattern: vec![5, 6, 0, 4, 2, 3],
                    },
                    SignalPattern {
                        pattern: vec![2, 6, 4, 1],
                    },
                    SignalPattern {
                        pattern: vec![5, 3, 2, 6, 4],
                    },
                    SignalPattern {
                        pattern: vec![0, 6, 4, 1, 5, 3],
                    },
                    SignalPattern {
                        pattern: vec![5, 4, 2, 3, 1],
                    },
                    SignalPattern {
                        pattern: vec![5, 0, 1, 2, 3],
                    },
                    SignalPattern {
                        pattern: vec![4, 3, 1],
                    },
                ],
                digits: [
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 1, 1, 1, 0],
                    },
                    Digit {
                        sections: [0, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 1, 0, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![4, 3, 1, 5, 6, 0],
                    },
                    SignalPattern {
                        pattern: vec![1, 4, 6, 2, 3],
                    },
                    SignalPattern {
                        pattern: vec![2, 1, 6],
                    },
                    SignalPattern {
                        pattern: vec![6, 2],
                    },
                    SignalPattern {
                        pattern: vec![6, 2, 0, 3, 4, 1, 5],
                    },
                    SignalPattern {
                        pattern: vec![5, 1, 6, 3, 4],
                    },
                    SignalPattern {
                        pattern: vec![0, 2, 1, 6, 5, 3],
                    },
                    SignalPattern {
                        pattern: vec![0, 1, 2, 3, 4],
                    },
                    SignalPattern {
                        pattern: vec![6, 5, 2, 1, 4, 3],
                    },
                    SignalPattern {
                        pattern: vec![6, 5, 4, 2],
                    },
                ],
                digits: [
                    Digit {
                        sections: [0, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 0, 0, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 0, 1, 0, 0, 0, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![5, 6, 0, 4, 1, 3],
                    },
                    SignalPattern {
                        pattern: vec![2, 6],
                    },
                    SignalPattern {
                        pattern: vec![1, 3, 0, 4, 2],
                    },
                    SignalPattern {
                        pattern: vec![6, 3, 0, 5, 1],
                    },
                    SignalPattern {
                        pattern: vec![0, 6, 1, 2, 5, 3],
                    },
                    SignalPattern {
                        pattern: vec![6, 3, 2, 1, 4, 5],
                    },
                    SignalPattern {
                        pattern: vec![1, 6, 2, 0, 3],
                    },
                    SignalPattern {
                        pattern: vec![6, 5, 0, 2],
                    },
                    SignalPattern {
                        pattern: vec![6, 2, 1],
                    },
                    SignalPattern {
                        pattern: vec![2, 3, 6, 0, 1, 4, 5],
                    },
                ],
                digits: [
                    Digit {
                        sections: [0, 0, 1, 0, 0, 0, 1],
                    },
                    Digit {
                        sections: [0, 0, 1, 0, 0, 0, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 0, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 0, 0, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![5, 1, 4, 6, 2, 3],
                    },
                    SignalPattern {
                        pattern: vec![2, 1, 3],
                    },
                    SignalPattern {
                        pattern: vec![0, 3, 2, 4, 5, 1],
                    },
                    SignalPattern {
                        pattern: vec![3, 0, 6, 4, 1],
                    },
                    SignalPattern {
                        pattern: vec![0, 5, 2, 1],
                    },
                    SignalPattern {
                        pattern: vec![1, 2],
                    },
                    SignalPattern {
                        pattern: vec![0, 4, 5, 3, 2],
                    },
                    SignalPattern {
                        pattern: vec![4, 2, 3, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![5, 6, 3, 4, 2, 0],
                    },
                    SignalPattern {
                        pattern: vec![5, 2, 3, 1, 4, 6, 0],
                    },
                ],
                digits: [
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 0],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 1, 0, 0],
                    },
                    Digit {
                        sections: [1, 0, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 0, 0, 0],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![0, 4, 2, 1, 5, 3, 6],
                    },
                    SignalPattern {
                        pattern: vec![5, 1, 6],
                    },
                    SignalPattern {
                        pattern: vec![6, 5],
                    },
                    SignalPattern {
                        pattern: vec![1, 0, 5, 4, 6],
                    },
                    SignalPattern {
                        pattern: vec![3, 1, 4, 5, 0],
                    },
                    SignalPattern {
                        pattern: vec![5, 2, 6, 4],
                    },
                    SignalPattern {
                        pattern: vec![6, 2, 1, 4, 0],
                    },
                    SignalPattern {
                        pattern: vec![5, 2, 0, 4, 6, 1],
                    },
                    SignalPattern {
                        pattern: vec![3, 6, 2, 4, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![5, 2, 1, 3, 6, 0],
                    },
                ],
                digits: [
                    Digit {
                        sections: [0, 0, 1, 0, 1, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 0, 0, 0, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 0, 0, 1, 1, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![5, 6, 4, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![2, 0],
                    },
                    SignalPattern {
                        pattern: vec![0, 5, 2, 4, 1, 6],
                    },
                    SignalPattern {
                        pattern: vec![1, 3, 0, 2, 5, 4, 6],
                    },
                    SignalPattern {
                        pattern: vec![2, 5, 0, 4, 3, 6],
                    },
                    SignalPattern {
                        pattern: vec![6, 2, 5, 3, 1],
                    },
                    SignalPattern {
                        pattern: vec![1, 0, 4, 2],
                    },
                    SignalPattern {
                        pattern: vec![1, 5, 0, 3, 4, 6],
                    },
                    SignalPattern {
                        pattern: vec![1, 0, 5, 6, 2],
                    },
                    SignalPattern {
                        pattern: vec![0, 2, 5],
                    },
                ],
                digits: [
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 0, 1, 0, 0],
                    },
                    Digit {
                        sections: [1, 0, 1, 0, 0, 0, 0],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![3, 1, 2, 5, 6],
                    },
                    SignalPattern {
                        pattern: vec![5, 6, 3],
                    },
                    SignalPattern {
                        pattern: vec![1, 3, 4, 6, 2, 0, 5],
                    },
                    SignalPattern {
                        pattern: vec![5, 6, 4, 2],
                    },
                    SignalPattern {
                        pattern: vec![0, 4, 6, 1, 3, 5],
                    },
                    SignalPattern {
                        pattern: vec![4, 2, 3, 5, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![5, 1, 4, 3, 2],
                    },
                    SignalPattern {
                        pattern: vec![3, 0, 2, 6, 1],
                    },
                    SignalPattern {
                        pattern: vec![6, 3, 2, 4, 1, 5],
                    },
                    SignalPattern {
                        pattern: vec![6, 5],
                    },
                ],
                digits: [
                    Digit {
                        sections: [0, 0, 1, 0, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 1, 1, 1, 0],
                    },
                    Digit {
                        sections: [0, 0, 1, 0, 1, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![1, 3, 5, 4, 6, 2],
                    },
                    SignalPattern {
                        pattern: vec![2, 1, 4, 6, 0, 5],
                    },
                    SignalPattern {
                        pattern: vec![6, 4, 2, 1, 5],
                    },
                    SignalPattern {
                        pattern: vec![3, 5, 2, 0, 6, 4],
                    },
                    SignalPattern {
                        pattern: vec![1, 3, 0, 2, 6],
                    },
                    SignalPattern {
                        pattern: vec![4, 3],
                    },
                    SignalPattern {
                        pattern: vec![1, 4, 3, 5],
                    },
                    SignalPattern {
                        pattern: vec![2, 4, 3],
                    },
                    SignalPattern {
                        pattern: vec![0, 3, 2, 1, 4, 5, 6],
                    },
                    SignalPattern {
                        pattern: vec![6, 4, 1, 2, 3],
                    },
                ],
                digits: [
                    Digit {
                        sections: [0, 0, 0, 1, 1, 0, 0],
                    },
                    Digit {
                        sections: [1, 1, 1, 0, 1, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 1, 0, 0, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 1, 1, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![4, 6, 0, 3, 5, 1],
                    },
                    SignalPattern {
                        pattern: vec![2, 3, 1, 5, 4, 6],
                    },
                    SignalPattern {
                        pattern: vec![2, 4, 6, 3],
                    },
                    SignalPattern {
                        pattern: vec![5, 4, 2, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![2, 6, 1],
                    },
                    SignalPattern {
                        pattern: vec![6, 1, 3, 4, 5, 2, 0],
                    },
                    SignalPattern {
                        pattern: vec![2, 6],
                    },
                    SignalPattern {
                        pattern: vec![5, 6, 2, 3, 0, 1],
                    },
                    SignalPattern {
                        pattern: vec![4, 6, 5, 3, 1],
                    },
                    SignalPattern {
                        pattern: vec![1, 5, 2, 4, 6],
                    },
                ],
                digits: [
                    Digit {
                        sections: [1, 1, 1, 1, 1, 1, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 0, 0, 1],
                    },
                    Digit {
                        sections: [0, 0, 1, 0, 0, 0, 1],
                    },
                    Digit {
                        sections: [0, 1, 1, 0, 0, 0, 1],
                    },
                ],
            },
            Row {
                patterns: [
                    SignalPattern {
                        pattern: vec![6, 2, 0, 5, 1],
                    },
                    SignalPattern {
                        pattern: vec![6, 2, 5],
                    },
                    SignalPattern {
                        pattern: vec![3, 2, 0, 4, 1, 5, 6],
                    },
                    SignalPattern {
                        pattern: vec![4, 2, 0, 6, 1],
                    },
                    SignalPattern {
                        pattern: vec![6, 5],
                    },
                    SignalPattern {
                        pattern: vec![0, 1, 2, 3, 4, 6],
                    },
                    SignalPattern {
                        pattern: vec![6, 0, 4, 5],
                    },
                    SignalPattern {
                        pattern: vec![2, 0, 5, 1, 6, 4],
                    },
                    SignalPattern {
                        pattern: vec![5, 3, 1, 0, 2],
                    },
                    SignalPattern {
                        pattern: vec![5, 4, 6, 1, 3, 2],
                    },
                ],
                digits: [
                    Digit {
                        sections: [1, 0, 0, 0, 1, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 0, 0, 1, 1],
                    },
                    Digit {
                        sections: [0, 0, 0, 0, 0, 1, 1],
                    },
                    Digit {
                        sections: [1, 1, 1, 0, 1, 0, 1],
                    },
                ],
            },
        ]
    }
}
