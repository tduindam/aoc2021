use std::collections::HashMap;

use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map_res;
use nom::multi::{count, many1, many_till};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::day16::PacketPayload::Literal;

lazy_static! {
    static ref HEX_MAP: HashMap<char, &'static str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u64,
    type_id: u64,
    payload: PacketPayload,
}

#[derive(Debug, Eq, PartialEq)]
enum PacketPayload {
    Literal(u64),
    SubPacket(Vec<Packet>),
}

fn from_num(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 2)
}

fn num_primary(input: &str) -> IResult<&str, u64> {
    map_res(take(3usize), from_num)(input)
}

fn parse_header(input: &str) -> IResult<&str, (u64, u64)> {
    tuple((num_primary, num_primary))(input)
}

fn parse_literal(input: &str) -> IResult<&str, PacketPayload> {
    let (input, (mut chunks, last)) = many_till(
        preceded(tag("1"), take(4usize)),
        preceded(tag("0"), take(4usize)),
    )(input)?;
    chunks.push(last);
    let joined = chunks.join("");
    let payload = u64::from_str_radix(joined.as_str(), 2).unwrap();
    Ok((input, PacketPayload::Literal(payload)))
}

fn parse_fixed_length(input: &str) -> IResult<&str, PacketPayload> {
    let (input, payload_size) = preceded(
        tag("0"),
        map_res(take(15usize), |s: &str| u32::from_str_radix(s, 2)),
    )(input)?;

    let (rest, payload) = take(payload_size)(input)?;
    let (_, payload) = many1(packet)(payload)?;
    Ok((rest, PacketPayload::SubPacket(payload)))
}

fn parse_number_packets(input: &str) -> IResult<&str, PacketPayload> {
    let (input, num_packets) = preceded(
        tag("1"),
        map_res(take(11usize), |s: &str| u32::from_str_radix(s, 2)),
    )(input)?;

    let (input, payload) = count(packet, num_packets as usize)(input)?;

    Ok((input, PacketPayload::SubPacket(payload)))
}

fn parse_operator(input: &str) -> IResult<&str, PacketPayload> {
    alt((parse_fixed_length, parse_number_packets))(input)
}

fn payload(input: &str, type_id: u64) -> IResult<&str, PacketPayload> {
    match type_id {
        4 => parse_literal(input),
        _ => parse_operator(input),
    }
}

fn parse_packet_from_hex(input: &str) -> Packet {
    let bits: String = input
        .chars()
        .filter_map(|c| HEX_MAP.get(&c))
        .map(|r| r.to_string())
        .collect();
    packet(&bits).unwrap().1
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (rest, (version, type_id)) = parse_header(&input)?;
    let (rest, payload) = payload(rest, type_id)?;
    Ok((
        rest,
        Packet {
            version,
            type_id,
            payload,
        },
    ))
}

impl Packet {
    fn from_str(input: &str) -> Self {
        parse_packet_from_hex(input)
    }

    fn version_sum(&self) -> u64 {
        self.version + {
            match &self.payload {
                Literal(_) => 0,
                PacketPayload::SubPacket(packets) => packets.iter().map(|p| p.version_sum()).sum(),
            }
        }
    }
}

/// Transmission contains single Packet which contains other Packets.
/// Hex representation might be padded with trailing 0s up to the next multiple of 4 / 16?
/// Packet:
/// Header:
///     Version 3 bits (number)
///     TypeID 3 bits (number)
/// Body | TypeId 4 (Literal)
/// Left padded with 0 until multiple of 4 (skip 0s?)
/// Broken in groups of 5 bits (1 status / 4 payload)
/// Concat into single binary string
/// Body | TypeId != 4 (Operator)
/// If first body bit is 0, length in bits of all sub-packets is 15 bit number
/// If first body bit is 1, next 11 bits represent number of sub-packets immediately contained by this packet
#[cfg(test)]
mod test {
    use crate::day16::PacketPayload::SubPacket;

    use super::*;

    #[test]
    fn part_one_small_1() {
        assert_eq!(
            Packet {
                version: 6,
                type_id: 4,
                payload: Literal(2021),
            },
            Packet::from_str("D2FE28")
        );
    }

    #[test]
    fn part_one_small_2() {
        assert_eq!(
            Packet {
                version: 1,
                type_id: 6,
                payload: SubPacket(vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        payload: Literal(10),
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        payload: Literal(20),
                    },
                ]),
            },
            Packet::from_str("38006F45291200")
        );
    }

    #[test]
    fn part_one_small_3() {
        assert_eq!(
            Packet {
                version: 7,
                type_id: 3,
                payload: SubPacket(vec![
                    Packet {
                        version: 2,
                        type_id: 4,
                        payload: Literal(1),
                    },
                    Packet {
                        version: 4,
                        type_id: 4,
                        payload: Literal(2),
                    },
                    Packet {
                        version: 1,
                        type_id: 4,
                        payload: Literal(3),
                    },
                ]),
            },
            Packet::from_str("EE00D40C823060")
        );
    }

    #[test]
    fn part_one_small_4() {
        let packet = Packet::from_str("8A004A801A8002F478");
        assert_eq!(
            Packet {
                version: 4,
                type_id: 2,
                payload: PacketPayload::SubPacket(vec![Packet {
                    version: 1,
                    type_id: 2,
                    payload: PacketPayload::SubPacket(vec![Packet {
                        version: 5,
                        type_id: 2,
                        payload: PacketPayload::SubPacket(vec![Packet {
                            version: 6,
                            type_id: 4,
                            payload: Literal(15),
                        }]),
                    }]),
                }]),
            },
            packet
        );
        assert_eq!(16, packet.version_sum());
    }

    #[test]
    fn part_one_small_5() {
        let packet = Packet::from_str("620080001611562C8802118E34");

        assert_eq!(12, packet.version_sum());
    }

    #[test]
    fn part_one_small_6() {
        assert_eq!(
            23,
            Packet::from_str("C0015000016115A2E0802F182340").version_sum()
        );
    }

    #[test]
    fn part_one_small_7() {
        assert_eq!(
            31,
            Packet::from_str("A0016C880162017C3686B18A3D4780").version_sum()
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(10, Packet::from_str("E20D41802B2984BD00540010F82D09E35880350D61A41D3004E5611E585F40159ED7AD7C90CF6BD6BE49C802DEB00525272CC1927752698693DA7C70029C0081002140096028C5400F6023C9C00D601ED88070070030005C2201448400E400F40400C400A50801E20004C1000809D14700B67676EE661137ADC64FF2BBAD745B3F2D69026335E92A0053533D78932A9DFE23AC7858C028920A973785338832CFA200F47C81D2BBBC7F9A9E1802FE00ACBA44F4D1E775DDC19C8054D93B7E72DBE7006AA200C41A8510980010D8731720CB80132918319804738AB3A8D3E773C4A4015A498E680292B1852E753E2B29D97F0DE6008CB3D4D031802D2853400D24DEAE0137AB8210051D24EB600844B95C56781B3004F002B99D8F635379EDE273AF26972D4A5610BA51004C12D1E25D802F32313239377B37100105343327E8031802B801AA00021D07231C2F10076184668693AC6600BCD83E8025231D752E5ADE311008A4EA092754596C6789727F069F99A4645008247D2579388DCF53558AE4B76B257200AAB80107947E94789FE76E36402868803F0D62743F00043A1646288800084C3F8971308032996A2BD8023292DF8BE467BB3790047F2572EF004A699E6164C013A007C62848DE91CC6DB459B6B40087E530AB31EE633BD23180393CBF36333038E011CBCE73C6FB098F4956112C98864EA1C2801D2D0F319802D60088002190620E479100622E4358952D84510074C0188CF0923410021F1CE1146E3006E3FC578EE600A4B6C4B002449C97E92449C97E92459796EB4FF874400A9A16100A26CEA6D0E5E5EC8841C9B8FE37109C99818023A00A4FD8BA531586BB8B1DC9AE080293B6972B7FA444285CC00AE492BC910C1697B5BDD8425409700562F471201186C0120004322B42489A200D4138A71AA796D00374978FE07B2314E99BFB6E909678A0").version_sum());
    }

    #[test]
    fn part_two() {
        assert!(false)
    }
}
