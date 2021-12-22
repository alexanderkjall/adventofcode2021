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
    literal: Option<u64>,
    length_type_id: Option<bool>,
    length: Option<u16>,
    sub_packets: Option<Vec<Packet>>,
}

fn take_3_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}

fn parse_literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Vec<u8>, usize)> {
    let mut last = false;
    let mut state = i;
    let mut buf = "".to_owned();

    let mut iter = 0;
    while !last {
        let (state1, last_bit): ((&[u8], usize), u8) = take(1usize)(state)?;
        state = state1;
        let (state1, val): ((&[u8], usize), u8) = take(4usize)(state)?;
        state = state1;

        buf = format!("{}{:X?}", buf, val);

        if last_bit == 0 {
            last = true;
        }
        iter += 1;
    }
    if buf.len() % 2 == 1 {
        buf = format!("0{}", buf);
    }

    Ok((state, (decode_hex(&buf).unwrap(), iter * 5)))
}

fn to_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), (Packet, usize)> {
    let (rest, version) = take_3_bits(input)?;
    let (rest, type_id) = take_3_bits(rest)?;

    if type_id == 4 {
        let (rest, (literal, bit_length)) = parse_literal(rest)?;

        let literal = literal
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |s, (i, v)| s + 256_u64.pow(i as u32) * (v as u64));
        Ok((
            rest,
            (
                Packet {
                    version,
                    type_id,
                    literal: Some(literal),
                    length_type_id: None,
                    length: None,
                    sub_packets: None,
                },
                bit_length + 6,
            ),
        ))
    } else {
        let (rest, length_type_id): ((&[u8], usize), u8) = take(1usize)(rest)?;

        if length_type_id > 0 {
            let (rest, length): ((&[u8], usize), u16) = take(11usize)(rest)?;

            let mut sub_packets = vec![];
            let mut rest = rest;
            let mut bit_length = 18;
            while {
                let p = to_packets(rest);

                if let Ok((r, p)) = p {
                    sub_packets.push(p.0);
                    bit_length += p.1;
                    rest = r;
                    sub_packets.len() < length as usize
                } else {
                    false
                }
            } {}
            Ok((
                rest,
                (
                    Packet {
                        version,
                        type_id,
                        literal: None,
                        length_type_id: Some(true),
                        length: Some(length),
                        sub_packets: Some(sub_packets),
                    },
                    bit_length,
                ),
            ))
        } else {
            let (rest, length): ((&[u8], usize), u16) = take(15usize)(rest)?;

            let mut sub_packets = vec![];
            let mut rest = rest;
            let mut bit_length = 0;
            while {
                let p = to_packets(rest);

                if let Ok((r, p)) = p {
                    sub_packets.push(p.0);
                    rest = r;
                    bit_length += p.1;
                    bit_length < length as usize
                } else {
                    false
                }
            } {}
            Ok((
                rest,
                (
                    Packet {
                        version,
                        type_id,
                        literal: None,
                        length_type_id: Some(false),
                        length: Some(length),
                        sub_packets: Some(sub_packets),
                    },
                    bit_length + 22,
                ),
            ))
        }
    }
}

fn add_version(packet: &Packet) -> u64 {
    match &packet.sub_packets {
        None => packet.version.into(),
        Some(sb) => (packet.version as u64) + sb.iter().map(add_version).sum::<u64>(),
    }
}

fn eval(packet: &Packet) -> u64 {
    match packet.type_id {
        0 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .fold(0u64, |s, p| s + eval(p)),
        1 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .fold(1u64, |s, p| s * eval(p)),
        2 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .fold(u64::MAX, |s, p| {
                let e = eval(p);
                if e < s {
                    e
                } else {
                    s
                }
            }),
        3 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .fold(u64::MIN, |s, p| {
                let e = eval(p);
                if e > s {
                    e
                } else {
                    s
                }
            }),
        4 => packet.literal.unwrap(),
        5 => {
            if eval(&packet.sub_packets.as_ref().unwrap()[0])
                > eval(&packet.sub_packets.as_ref().unwrap()[1])
            {
                1
            } else {
                0
            }
        }
        6 => {
            if eval(&packet.sub_packets.as_ref().unwrap()[0])
                < eval(&packet.sub_packets.as_ref().unwrap()[1])
            {
                1
            } else {
                0
            }
        }
        7 => {
            if eval(&packet.sub_packets.as_ref().unwrap()[0])
                == eval(&packet.sub_packets.as_ref().unwrap()[1])
            {
                1
            } else {
                0
            }
        }
        _ => panic!("unreachable"),
    }
}

fn part1(input: &[u8]) -> Result<String, Error> {
    let (_, (packet, _)) = to_packets((input, 0)).unwrap();

    Ok(format!("{}", add_version(&packet)))
}

fn part2(input: &[u8]) -> Result<String, Error> {
    let (_, (packet, _)) = to_packets((input, 0)).unwrap();

    Ok(format!("{}", eval(&packet)))
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

    fn input_7() -> Vec<u8> {
        vec![194, 0, 180, 10, 130]
    }

    fn input_8() -> Vec<u8> {
        vec![4, 0, 90, 195, 56, 144]
    }

    fn input_9() -> Vec<u8> {
        vec![136, 0, 134, 195, 232, 129, 18]
    }

    fn input_10() -> Vec<u8> {
        vec![206, 0, 196, 61, 136, 17, 32]
    }

    fn input_11() -> Vec<u8> {
        vec![216, 0, 90, 194, 168, 240]
    }

    fn input_12() -> Vec<u8> {
        vec![246, 0, 188, 45, 143]
    }

    fn input_13() -> Vec<u8> {
        vec![156, 0, 90, 194, 248, 240]
    }

    fn input_14() -> Vec<u8> {
        vec![156, 1, 65, 8, 2, 80, 50, 15, 24, 2, 16, 74, 8]
    }

    fn input_15() -> Vec<u8> {
        vec![32, 0, 251, 128, 0, 179, 2, 196, 0, 152, 16]
    }

    fn input_16() -> Vec<u8> {
        vec![194, 0, 80, 0, 44, 112, 32, 0, 5, 200, 16]
    }

    fn input_17() -> Vec<u8> {
        vec![160, 1, 126, 0, 2, 139, 3, 68, 0, 216, 24, 0, 2, 228, 8]
    }

    fn input_18() -> Vec<u8> {
        vec![210, 254, 40]
    }

    #[test]
    pub fn test_decode_hex() {
        assert_eq!(vec![0b00000111, 0b11100101], decode_hex("07E5").unwrap());
    }

    #[test]
    pub fn test_parse_literal() {
        let input = vec![0b10111111, 0b10001010, 0b00000000];
        let (rest, data) = parse_literal((&input, 0)).unwrap();

        assert_eq!(vec![0b00000111, 0b11100101], data.0);
        assert_eq!(15, data.1);
        assert_eq!(7, rest.1);
    }

    #[test]
    pub fn test_to_packets_1() {
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
                        literal: Some(10),
                        length_type_id: None,
                        length: None,
                        sub_packets: None
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        literal: Some(20),
                        length_type_id: None,
                        length: None,
                        sub_packets: None
                    }
                ]),
            },
            to_packets((&input_1(), 0)).unwrap().1 .0
        );
    }

    #[test]
    pub fn test_to_packets_14() {
        assert_eq!(
            Packet {
                version: 4,
                type_id: 7,
                literal: None,
                length_type_id: Some(false),
                length: Some(80),
                sub_packets: Some(vec![
                    Packet {
                        version: 2,
                        type_id: 0,
                        literal: None,
                        length_type_id: Some(true),
                        length: Some(2),
                        sub_packets: Some(vec![
                            Packet {
                                version: 2,
                                type_id: 4,
                                literal: Some(1),
                                length_type_id: None,
                                length: None,
                                sub_packets: None
                            },
                            Packet {
                                version: 4,
                                type_id: 4,
                                literal: Some(3),
                                length_type_id: None,
                                length: None,
                                sub_packets: None
                            },
                        ])
                    },
                    Packet {
                        version: 6,
                        type_id: 1,
                        literal: None,
                        length_type_id: Some(true),
                        length: Some(2),
                        sub_packets: Some(vec![
                            Packet {
                                version: 0,
                                type_id: 4,
                                literal: Some(2),
                                length_type_id: None,
                                length: None,
                                sub_packets: None
                            },
                            Packet {
                                version: 2,
                                type_id: 4,
                                literal: Some(2),
                                length_type_id: None,
                                length: None,
                                sub_packets: None
                            }
                        ])
                    }
                ])
            },
            to_packets((&input_14(), 0)).unwrap().1 .0
        );
    }

    #[test]
    pub fn test_to_packets_15() {
        assert_eq!(
            Packet {
                version: 1,
                type_id: 0,
                literal: None,
                length_type_id: Some(false),
                length: Some(62),
                sub_packets: Some(vec![
                    Packet {
                        version: 7,
                        type_id: 0,
                        literal: None,
                        length_type_id: Some(false),
                        length: Some(11),
                        sub_packets: Some(vec![Packet {
                            version: 1,
                            type_id: 4,
                            literal: Some(1),
                            length_type_id: None,
                            length: None,
                            sub_packets: None
                        }])
                    },
                    Packet {
                        version: 3,
                        type_id: 0,
                        literal: None,
                        length_type_id: Some(true),
                        length: Some(1),
                        sub_packets: Some(vec![Packet {
                            version: 1,
                            type_id: 4,
                            literal: Some(1),
                            length_type_id: None,
                            length: None,
                            sub_packets: None
                        }])
                    }
                ])
            },
            to_packets((&input_15(), 0)).unwrap().1 .0
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
    pub fn test_parse_7() {
        assert_eq!(input_7(), parse_input("C200B40A82\n").unwrap());
    }

    #[test]
    pub fn test_parse_8() {
        assert_eq!(input_8(), parse_input("04005AC33890\n").unwrap());
    }

    #[test]
    pub fn test_parse_9() {
        assert_eq!(input_9(), parse_input("880086C3E88112\n").unwrap());
    }

    #[test]
    pub fn test_parse_10() {
        assert_eq!(input_10(), parse_input("CE00C43D881120\n").unwrap());
    }

    #[test]
    pub fn test_parse_11() {
        assert_eq!(input_11(), parse_input("D8005AC2A8F0\n").unwrap());
    }

    #[test]
    pub fn test_parse_12() {
        assert_eq!(input_12(), parse_input("F600BC2D8F\n").unwrap());
    }

    #[test]
    pub fn test_parse_13() {
        assert_eq!(input_13(), parse_input("9C005AC2F8F0\n").unwrap());
    }

    #[test]
    pub fn test_parse_14() {
        assert_eq!(
            input_14(),
            parse_input("9C0141080250320F1802104A08\n").unwrap()
        );
    }

    #[test]
    pub fn test_parse_15() {
        assert_eq!(input_15(), parse_input("2000FB8000B302C4009810\n").unwrap());
    }

    #[test]
    pub fn test_parse_16() {
        assert_eq!(input_16(), parse_input("C20050002C70200005C810\n").unwrap());
    }

    #[test]
    pub fn test_parse_17() {
        assert_eq!(
            input_17(),
            parse_input("A0017E00028B034400D8180002E408\n").unwrap()
        );
    }

    #[test]
    pub fn test_parse_18() {
        assert_eq!(input_18(), parse_input("D2FE28\n").unwrap());
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
    pub fn test_part2_1() {
        assert_eq!("3", part2(&input_7()).unwrap());
    }

    #[test]
    pub fn test_part2_2() {
        assert_eq!("54", part2(&input_8()).unwrap());
    }

    #[test]
    pub fn test_part2_3() {
        assert_eq!("7", part2(&input_9()).unwrap());
    }

    #[test]
    pub fn test_part2_4() {
        assert_eq!("9", part2(&input_10()).unwrap());
    }

    #[test]
    pub fn test_part2_5() {
        assert_eq!("1", part2(&input_11()).unwrap());
    }

    #[test]
    pub fn test_part2_6() {
        assert_eq!("0", part2(&input_12()).unwrap());
    }

    #[test]
    pub fn test_part2_7() {
        assert_eq!("0", part2(&input_13()).unwrap());
    }

    #[test]
    pub fn test_part2_8() {
        assert_eq!("1", part2(&input_14()).unwrap());
    }

    #[test]
    pub fn test_part2_9() {
        assert_eq!("2", part2(&input_15()).unwrap());
    }

    #[test]
    pub fn test_part2_10() {
        assert_eq!("2", part2(&input_16()).unwrap());
    }

    #[test]
    pub fn test_part2_11() {
        assert_eq!("3", part2(&input_17()).unwrap());
    }

    #[test]
    pub fn test_part2_12() {
        assert_eq!("2021", part2(&input_18()).unwrap());
    }
}
