use itertools::Itertools;

use crate::solver::Solver;

pub struct Solution;
impl Solver<u64, u64> for Solution {
    const DAY: u8 = 16;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<u64> {
        let input = self.input().get()?;
        let packet = decode(&input).unwrap();
        Ok(sum_versions(&[packet]))
    }

    fn part_two(&self) -> anyhow::Result<u64> {
        let input = self.input().get()?;
        let packet = decode(&input).unwrap();
        Ok(packet.evaluate())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Packet {
    version: u64,
    type_id: PacketType,
    payload: Vec<Packet>,
}
impl Packet {
    fn evaluate(&self) -> u64 {
        match self.type_id {
            PacketType::Literal(val) => val,
            PacketType::Operator(op) => match op {
                Operator::Sum => self.payload.iter().map(|p| p.evaluate()).sum(),
                Operator::Prd => self.payload.iter().map(|p| p.evaluate()).product(),
                Operator::Min => self.payload.iter().map(|p| p.evaluate()).min().unwrap(),
                Operator::Max => self.payload.iter().map(|p| p.evaluate()).max().unwrap(),
                Operator::Gt => {
                    let (l, r) = (self.payload[0].evaluate(), self.payload[1].evaluate());
                    if l > r {
                        1
                    } else {
                        0
                    }
                }
                Operator::Lt => {
                    let (l, r) = (self.payload[0].evaluate(), self.payload[1].evaluate());
                    if l < r {
                        1
                    } else {
                        0
                    }
                }
                Operator::Eq => {
                    let (l, r) = (self.payload[0].evaluate(), self.payload[1].evaluate());
                    if l == r {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Operator(Operator),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Sum,
    Prd,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}
impl From<u64> for Operator {
    fn from(v: u64) -> Self {
        match v {
            0 => Operator::Sum,
            1 => Operator::Prd,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::Gt,
            6 => Operator::Lt,
            7 => Operator::Eq,
            _ => panic!("Invalid operator {}", v),
        }
    }
}

trait Decodable {
    fn get_decoder(&self) -> Decoder;
    fn to_binary(&self) -> String;
}
impl Decodable for str {
    fn get_decoder(&self) -> Decoder {
        let source = self.chars().collect_vec();
        Decoder {
            source,
            pos: 0,
            buffer: 0,
            bsize: 0,
            read: 0,
        }
    }

    fn to_binary(&self) -> String {
        self.chars()
            .map(|c| {
                format!(
                    "{:04b}",
                    c.to_digit(16).expect(&format!("Invalid hex char {}", c))
                )
            })
            .collect::<String>()
    }
}
struct Decoder {
    source: Vec<char>,
    pos: usize,
    buffer: u64,
    bsize: i8,
    read: i8,
}
impl Decoder {
    fn next(&mut self, bits: i8) -> Option<u64> {
        assert!(bits < 32, "Too many bits, Mr. Mozart");
        if bits < 1 || self.pos > self.source.len() {
            return None;
        }
        let mut req = bits;
        while self.bsize < req {
            if let Some(c) = self.source.get(self.pos) {
                self.pos += 1;
                self.bsize += 4;
                self.buffer <<= 4;
                self.buffer += c
                    .to_digit(16)
                    .expect(&format!("Invalid hexadecimal character: {}", c))
                    as u64;
            } else {
                // Out of characters, flush the buffer if any
                if self.bsize == 0 {
                    return None;
                }
                req = self.bsize;
            }
        }
        let remainder = self.bsize - req;
        let val = self.buffer >> remainder;
        self.buffer &= !(u64::MAX << remainder);
        self.read = 4 - remainder;
        self.bsize = remainder;
        Some(val)
    }

    fn skip(&mut self, bits: i8) -> Option<()> {
        if bits == 0 {
            Some(())
        } else {
            self.next(bits).and(Some(()))
        }
    }
}

fn sum_versions(packets: &[Packet]) -> u64 {
    packets
        .iter()
        .map(|p| p.version + sum_versions(&p.payload))
        .sum()
}

const LITERAL: u64 = 4;

fn decode(source: &str) -> Option<Packet> {
    _decode(source, 0).and_then(|(_, _, p)| Some(p))
}

fn _decode(source: &str, skip: i8) -> Option<(usize, i8, Packet)> {
    let mut decoder = source.get_decoder();
    let mut payload: Vec<Packet> = vec![];
    let mut consumed = 0;
    let mut offset = 0;

    let mut packet: Option<Packet> = None;

    decoder.skip(skip)?;
    let version = decoder.next(3)?;
    let type_id = decoder.next(3)?;

    // println!("Got version {}, type {}", version, type_id);

    if type_id == LITERAL {
        // println!("  Processing literal");
        let mut value = 0;
        loop {
            let ind = decoder.next(1)?;
            let cur = decoder.next(4)?;
            value <<= 4;
            value += cur;
            if ind == 0 {
                break;
            }
        }
        // println!("  Got value {}", value);
        packet = Some(Packet {
            version,
            type_id: PacketType::Literal(value),
            payload,
        });
        consumed = decoder.pos;
        offset = decoder.read;
    } else {
        // println!("  Processing operator");
        let length_type = decoder.next(1)?;
        if length_type == 0 {
            // Fixed length inner packet(s)
            let length = decoder.next(15)? as usize;
            // println!("    Fixed length operands: {} bits", length);
            let mut start = decoder.pos - 1;
            let end = start + (length / 4);
            let mut read = decoder.read;
            while start < end {
                // println!("        Processing char {} offset {}", start, read);
                let (ic, ir, inner) = _decode(&source[start..], read).expect("Expected packet");
                start += ic - 1;
                read = ir;
                payload.push(inner);
            }
            consumed += start + 1;
            offset = read;
        } else {
            // Variable length inner packet(s)
            let length = decoder.next(11)? as usize;
            // println!("    Variable length operands: {} packets", length);
            let mut start = decoder.pos - 1;
            let mut read = decoder.read;
            for _ in 0..length {
                // println!("        Processing char {} offset {}", start, read);
                let (ic, ir, inner) = _decode(&source[start..], read).expect("Expected packet");
                start += ic - 1;
                read = ir;
                payload.push(inner);
            }
            consumed += start + 1;
            offset = read;
        }
        packet = Some(Packet {
            version,
            type_id: PacketType::Operator(type_id.into()),
            payload,
        });
    }

    packet.and_then(|p| Some((consumed, offset, p)))
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn should_decode() {
        let mut decoder = "FF".get_decoder();
        let test = decoder.next(3).unwrap();
        assert_eq!(7, test);
        assert_eq!(1, decoder.bsize);
        assert_eq!(1, decoder.buffer);

        let test = decoder.next(2).unwrap();
        assert_eq!(3, test);
        assert_eq!(3, decoder.bsize);
        assert_eq!(7, decoder.buffer);

        let test = decoder.next(3).unwrap();
        assert_eq!(7, test);
        assert_eq!(0, decoder.bsize);
        assert_eq!(0, decoder.buffer);
    }

    #[test]
    fn should_get_binary() {
        let input = "F0";
        assert_eq!("11110000", input.to_binary());
    }

    #[test]
    fn should_decode_literal() {
        let input = "D2FE28";
        let packet = decode(input).unwrap();
        assert_eq!(6, packet.version);
        assert_eq!(PacketType::Literal(2021), packet.type_id);
    }

    #[test]
    fn should_decode_operator() {
        let input = "38006F45291200";
        let packet = decode(input).unwrap();
        assert_eq!(1, packet.version);
        assert_eq!(PacketType::Operator(Operator::Lt), packet.type_id);
        assert_eq!(
            vec![
                Packet {
                    version: 6,
                    type_id: PacketType::Literal(10),
                    payload: vec![]
                },
                Packet {
                    version: 2,
                    type_id: PacketType::Literal(20),
                    payload: vec![]
                }
            ],
            packet.payload
        );
    }

    #[test]
    fn should_decode_variable_length_operator() {
        let input = "EE00D40C823060";
        let packet = decode(input).unwrap();
        assert_eq!(7, packet.version);
        assert_eq!(PacketType::Operator(Operator::Max), packet.type_id);
        assert_eq!(
            vec![
                Packet {
                    version: 2,
                    type_id: PacketType::Literal(1),
                    payload: vec![]
                },
                Packet {
                    version: 4,
                    type_id: PacketType::Literal(2),
                    payload: vec![]
                },
                Packet {
                    version: 1,
                    type_id: PacketType::Literal(3),
                    payload: vec![]
                },
            ],
            packet.payload
        );
    }

    #[test]
    fn should_sum_versions() {
        let input = "38006F45291200";
        let packet = decode(input).unwrap();
        let sum = sum_versions(&[packet]);
        assert_eq!(9, sum);
    }

    #[test]
    fn should_solve_part1_examples() {
        let input = "8A004A801A8002F478";
        let packet = decode(input).unwrap();
        let sum = sum_versions(&[packet]);
        assert_eq!(16, sum);

        let input = "620080001611562C8802118E34";
        let packet = decode(input).unwrap();
        let sum = sum_versions(&[packet]);
        assert_eq!(12, sum);

        let input = "C0015000016115A2E0802F182340";
        let packet = decode(input).unwrap();
        let sum = sum_versions(&[packet]);
        assert_eq!(23, sum);

        let input = "A0016C880162017C3686B18A3D4780";
        let packet = decode(input).unwrap();
        let sum = sum_versions(&[packet]);
        assert_eq!(31, sum);
    }

    #[test]
    fn should_solve_part2_examples() {
        let input = "C200B40A82";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(3, val);

        let input = "04005AC33890";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(54, val);

        let input = "880086C3E88112";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(7, val);

        let input = "CE00C43D881120";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(9, val);

        let input = "D8005AC2A8F0";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(1, val);

        let input = "F600BC2D8F";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(0, val);

        let input = "9C005AC2F8F0";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(0, val);

        let input = "9C0141080250320F1802104A08";
        let packet = decode(input).unwrap();
        let val = packet.evaluate();
        assert_eq!(1, val);
    }
}
