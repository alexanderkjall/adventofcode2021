use crate::Error;
use nom::bits::complete::take;
use nom::character::complete::alphanumeric1;
use nom::IResult;
use std::fs::read_to_string;
use std::num::ParseIntError;

pub fn calculate() -> Result<(String, String), Error> {
    let input = parse_input(&read_to_string("input/day16")?)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn parse_input(input: &str) -> Result<Vec<u8>, Error> {
    let (_, data) = alphanumeric1(input)?;

    Ok(decode_hex(data)?)
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal: Option<Vec<u8>>,
    length_type_id: Option<bool>,
    length: Option<u16>,
    sub_packets: Option<Vec<Packet>>,
}

fn take_3_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}

fn parse_literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u8>> {
    let mut last = false;
    let mut state = i;
    let mut buf = "".to_owned();

    while !last {
        let (state1, last_bit): ((&[u8], usize), u8) = take(1usize)(state)?;
        state = state1;
        let (state1, val): ((&[u8], usize), u8) = take(4usize)(state)?;
        state = state1;

        buf = format!("{}{:X?}", buf, val);

        if last_bit == 0 {
            last = true;
        }
    }
    if buf.len() % 2 == 1 {
        buf = format!("0{}", buf);
    }

    Ok((state, decode_hex(&buf).unwrap()))
}

fn to_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (rest, version) = take_3_bits(input)?;
    let (rest, type_id) = take_3_bits(rest)?;

    if type_id == 4 {
        let (rest, literal) = parse_literal(rest)?;

        Ok((
            rest,
            Packet {
                version,
                type_id,
                literal: Some(literal),
                length_type_id: None,
                length: None,
                sub_packets: None,
            },
        ))
    } else {
        let (rest, length_type_id): ((&[u8], usize), u8) = take(1usize)(rest)?;

        if length_type_id > 0 {
            let (rest, length): ((&[u8], usize), u16) = take(11usize)(rest)?;

            let mut sub_packets = vec![];
            let mut rest = rest;
            while {
                let p = to_packets(rest);

                if let Ok((r, p)) = p {
                    sub_packets.push(p);
                    rest = r;
                    true
                } else {
                    false
                }
            } {}
            Ok((
                rest,
                Packet {
                    version,
                    type_id,
                    literal: None,
                    length_type_id: Some(true),
                    length: Some(length),
                    sub_packets: Some(sub_packets),
                },
            ))
        } else {
            let (rest, length): ((&[u8], usize), u16) = take(15usize)(rest)?;

            let mut sub_packets = vec![];
            let mut rest = rest;
            while {
                let p = to_packets(rest);

                if let Ok((r, p)) = p {
                    sub_packets.push(p);
                    rest = r;
                    true
                } else {
                    false
                }
            } {}
            Ok((
                rest,
                Packet {
                    version,
                    type_id,
                    literal: None,
                    length_type_id: Some(false),
                    length: Some(length),
                    sub_packets: Some(sub_packets),
                },
            ))
        }
    }
}

fn add_version(packet: &Packet) -> u32 {
    match &packet.sub_packets {
        None => packet.version.into(),
        Some(sb) => (packet.version as u32) + sb.iter().map(add_version).sum::<u32>(),
    }
}

fn part1(input: &[u8]) -> Result<String, Error> {
    let packet = to_packets((input, 0)).unwrap();

    Ok(format!("{}", add_version(&packet.1)))
}

fn part2(_input: &[u8]) -> Result<String, Error> {
    Ok(format!(""))
}

#[cfg(test)]
mod tests {
    use crate::day16::decode_hex;
    use crate::day16::parse_input;
    use crate::day16::parse_literal;
    use crate::day16::part1;
    use crate::day16::part2;
    use crate::day16::to_packets;
    use crate::day16::Packet;

    fn input_1() -> Vec<u8> {
        vec![56, 0, 111, 69, 41, 18, 0]
    }

    fn input_2() -> Vec<u8> {
        vec![238, 0, 212, 12, 130, 48, 96]
    }

    fn input_3() -> Vec<u8> {
        vec![138, 0, 74, 128, 26, 128, 2, 244, 120]
    }

    fn input_4() -> Vec<u8> {
        vec![98, 0, 128, 0, 22, 17, 86, 44, 136, 2, 17, 142, 52]
    }

    fn input_5() -> Vec<u8> {
        vec![192, 1, 80, 0, 1, 97, 21, 162, 224, 128, 47, 24, 35, 64]
    }

    fn input_6() -> Vec<u8> {
        vec![
            160, 1, 108, 136, 1, 98, 1, 124, 54, 134, 177, 138, 61, 71, 128,
        ]
    }

    #[test]
    pub fn test_decode_hex() {
        assert_eq!(vec![0b00000111, 0b11100101], decode_hex("07E5").unwrap());
    }

    #[test]
    pub fn test_parse_literal() {
        let input = vec![0b10111111, 0b10001010, 0b00000000];
        let (rest, data) = parse_literal((&input, 0)).unwrap();

        assert_eq!(vec![0b00000111, 0b11100101], data);
        assert_eq!(7, rest.1);
    }

    #[test]
    pub fn test_to_packets() {
        assert_eq!(
            Packet {
                version: 1,
                type_id: 6,
                literal: None,
                length_type_id: Some(false),
                length: Some(27),
                sub_packets: Some(vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        literal: Some(vec![10]),
                        length_type_id: None,
                        length: None,
                        sub_packets: None
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        literal: Some(vec![20]),
                        length_type_id: None,
                        length: None,
                        sub_packets: None
                    }
                ]),
            },
            to_packets((&input_1(), 0)).unwrap().1
        );
    }

    #[test]
    pub fn test_parse_1() {
        assert_eq!(input_1(), parse_input("38006F45291200\n").unwrap());
    }

    #[test]
    pub fn test_parse_2() {
        assert_eq!(input_2(), parse_input("EE00D40C823060\n").unwrap());
    }

    #[test]
    pub fn test_parse_3() {
        assert_eq!(input_3(), parse_input("8A004A801A8002F478\n").unwrap());
    }

    #[test]
    pub fn test_parse_4() {
        assert_eq!(
            input_4(),
            parse_input("620080001611562C8802118E34\n").unwrap()
        );
    }

    #[test]
    pub fn test_parse_5() {
        assert_eq!(
            input_5(),
            parse_input("C0015000016115A2E0802F182340\n").unwrap()
        );
    }

    #[test]
    pub fn test_parse_6() {
        assert_eq!(
            input_6(),
            parse_input("A0016C880162017C3686B18A3D4780\n").unwrap()
        );
    }

    #[test]
    pub fn test_part1_1() {
        assert_eq!("9", part1(&input_1()).unwrap());
    }

    #[test]
    pub fn test_part1_2() {
        assert_eq!("14", part1(&input_2()).unwrap());
    }

    #[test]
    pub fn test_part1_3() {
        assert_eq!("16", part1(&input_3()).unwrap());
    }

    #[test]
    pub fn test_part1_4() {
        assert_eq!("12", part1(&input_4()).unwrap());
    }

    #[test]
    pub fn test_part1_5() {
        assert_eq!("23", part1(&input_5()).unwrap());
    }

    #[test]
    pub fn test_part1_6() {
        assert_eq!("31", part1(&input_6()).unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!("", part2(&vec![]).unwrap());
    }
}
