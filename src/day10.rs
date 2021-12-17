use crate::day10::Command::*;
use crate::Error;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day10")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

#[derive(PartialEq, Clone, Debug)]
enum Command {
    PushA,
    PopA,
    PushB,
    PopB,
    PushC,
    PopC,
    PushP,
    PopP,
}

#[derive(PartialEq, Clone, Debug)]
enum ValidationError {
    Incomplete(Vec<Command>),
    Corrupted(Command),
    StackUnderFlow,
}

impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            '<' => Command::PushA,
            '>' => Command::PopA,
            '[' => Command::PushB,
            ']' => Command::PopB,
            '{' => Command::PushC,
            '}' => Command::PopC,
            '(' => Command::PushP,
            ')' => Command::PopP,
            _ => panic!("illegal input"),
        }
    }
}

fn my_str(input: &str) -> IResult<&str, Vec<Command>> {
    let (rest, data) = take_until("\n")(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data.chars().map(Command::from).collect()))
}

fn multi(i: &str) -> IResult<&str, Vec<Vec<Command>>> {
    many0(my_str)(i)
}

fn parse_input(input: &str) -> Result<Vec<Vec<Command>>, Error> {
    let (_, data) = multi(input)?;

    Ok(data)
}

fn validate(commands: &[Command]) -> Result<(), ValidationError> {
    let mut stack = vec![];

    for c in commands {
        match c {
            PushA | PushB | PushC | PushP => stack.push((*c).clone()),
            PopA => {
                if let Some(val) = stack.pop() {
                    if val != PushA {
                        return Err(ValidationError::Corrupted(PopA));
                    }
                } else {
                    return Err(ValidationError::StackUnderFlow);
                }
            }
            PopB => {
                if let Some(val) = stack.pop() {
                    if val != PushB {
                        return Err(ValidationError::Corrupted(PopB));
                    }
                } else {
                    return Err(ValidationError::StackUnderFlow);
                }
            }
            PopC => {
                if let Some(val) = stack.pop() {
                    if val != PushC {
                        return Err(ValidationError::Corrupted(PopC));
                    }
                } else {
                    return Err(ValidationError::StackUnderFlow);
                }
            }
            PopP => {
                if let Some(val) = stack.pop() {
                    if val != PushP {
                        return Err(ValidationError::Corrupted(PopP));
                    }
                } else {
                    return Err(ValidationError::StackUnderFlow);
                }
            }
        }
    }

    if !stack.is_empty() {
        Err(ValidationError::Incomplete(stack))
    } else {
        Ok(())
    }
}

fn part1(input: &[Vec<Command>]) -> Result<String, Error> {
    let mut result = 0;
    for c in input {
        if let Err(ValidationError::Corrupted(val)) = validate(c) {
            match val {
                PopA => result += 25137,
                PopB => result += 57,
                PopC => result += 1197,
                PopP => result += 3,
                _ => {}
            }
        }
    }
    Ok(format!("{}", result))
}

fn score_stack(stack: &[Command]) -> u64 {
    let mut score = 0;
    for s in stack {
        match s {
            PushA => score = score * 5 + 4,
            PushB => score = score * 5 + 2,
            PushC => score = score * 5 + 3,
            PushP => score = score * 5 + 1,
            _ => {}
        }
    }
    score
}

fn part2(input: &[Vec<Command>]) -> Result<String, Error> {
    let mut result = vec![];
    for c in input {
        if let Err(ValidationError::Incomplete(stack)) = validate(c) {
            let stack: Vec<Command> = stack.into_iter().rev().collect();
            result.push(score_stack(&stack));
        }
    }
    result.sort_unstable();
    Ok(format!("{}", result[result.len() / 2]))
}

#[cfg(test)]
mod tests {
    use crate::day10::parse_input;
    use crate::day10::part1;
    use crate::day10::part2;
    use crate::day10::score_stack;
    use crate::day10::validate;
    use crate::day10::Command;
    use crate::day10::Command::*;
    use crate::day10::ValidationError::Corrupted;

    fn test_input() -> Vec<Vec<Command>> {
        vec![
            vec![
                PushB, PushP, PushC, PushP, PushA, PushP, PushP, PopP, PopP, PushB, PopB, PopA,
                PushB, PushB, PushC, PushB, PopB, PushC, PushA, PushP, PopP, PushA, PopA, PopA,
            ],
            vec![
                PushB, PushP, PushP, PopP, PushB, PushA, PopA, PopB, PopP, PopB, PushP, PushC,
                PushB, PushA, PushC, PushA, PushA, PushB, PopB, PopA, PopA, PushP,
            ],
            vec![
                PushC, PushP, PushB, PushP, PushA, PushC, PopC, PushB, PushA, PopA, PushB, PopB,
                PopC, PopA, PushC, PushB, PopB, PushC, PushB, PushP, PushA, PushP, PopP, PopA,
            ],
            vec![
                PushP, PushP, PushP, PushP, PushC, PushA, PopA, PopC, PushA, PushC, PushA, PushC,
                PushA, PopA, PopC, PushC, PushB, PopB, PushC, PushB, PopB, PushC, PopC,
            ],
            vec![
                PushB, PushB, PushA, PushB, PushP, PushB, PopB, PopP, PopP, PushA, PushP, PushB,
                PushB, PushC, PopC, PushB, PushB, PushP, PopP, PopB, PopB, PopB,
            ],
            vec![
                PushB, PushC, PushB, PushC, PushP, PushC, PopC, PopB, PushC, PopC, PopC, PushP,
                PushB, PushC, PushB, PushC, PushC, PushC, PopC, PopC, PushP, PushB, PopB,
            ],
            vec![
                PushC, PushA, PushB, PushB, PopB, PopB, PopA, PopC, PushA, PushC, PushB, PushC,
                PushB, PushC, PushB, PopB, PushC, PushP, PopP, PushB, PushB, PushB, PopB,
            ],
            vec![
                PushB, PushA, PushP, PushA, PushP, PushA, PushP, PushA, PushC, PopC, PopP, PopP,
                PopA, PushA, PushP, PushB, PopB, PushP, PushB, PopB, PushP, PopP,
            ],
            vec![
                PushA, PushC, PushP, PushB, PushP, PushB, PushB, PushP, PushA, PopA, PushP, PopP,
                PopP, PushC, PopC, PopB, PopA, PushP, PushA, PushA, PushC, PushC,
            ],
            vec![
                PushA, PushC, PushP, PushB, PushC, PushC, PopC, PopC, PushB, PushA, PushB, PushB,
                PushB, PushA, PopA, PushC, PopC, PopB, PopB, PopB, PopA, PushB, PopB, PopB,
            ],
        ]
    }

    #[test]
    pub fn test_parse() {
        let res = parse_input(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
",
        );

        assert_eq!(test_input(), res.unwrap());
    }

    #[test]
    pub fn test_parse2() {
        let res = parse_input("[{[{({}]{}}([{[{{{}}([]\n");

        assert_eq!(
            vec![vec![
                PushB, PushC, PushB, PushC, PushP, PushC, PopC, PopB, PushC, PopC, PopC, PushP,
                PushB, PushC, PushB, PushC, PushC, PushC, PopC, PopC, PushP, PushB, PopB
            ]],
            res.unwrap()
        );
    }

    #[test]
    pub fn test_validate() {
        let commands = vec![
            PushB, PushC, PushB, PushC, PushP, PushC, PopC, PopB, PushC, PopC, PopC, PushP, PushB,
            PushC, PushB, PushC, PushC, PushC, PopC, PopC, PushP, PushB, PopB,
        ];

        assert_eq!(Err(Corrupted(PopB)), validate(&commands));
    }

    #[test]
    pub fn test_score_stack() {
        assert_eq!(
            288957,
            score_stack(&vec![
                PushC, PushC, PushB, PushB, PushP, PushC, PushP, PushB
            ])
        );
        assert_eq!(
            5566,
            score_stack(&vec![PushP, PushC, PushA, PushB, PushC, PushP])
        );
        assert_eq!(
            1480781,
            score_stack(&vec![
                PushC, PushC, PushA, PushC, PushA, PushP, PushP, PushP, PushP
            ])
        );
        assert_eq!(
            995444,
            score_stack(&vec![
                PushB, PushB, PushC, PushC, PushB, PushC, PushB, PushC, PushA
            ])
        );
        assert_eq!(294, score_stack(&vec![PushB, PushP, PushC, PushA]));
    }

    #[test]
    pub fn test_part1() {
        assert_eq!("26397", part1(&test_input()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("288957", part2(&test_input()).unwrap());
    }
}
