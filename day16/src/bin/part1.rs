use aoc::*;

fn to_int(input: &[usize]) -> usize {
    input.iter().fold(0, |acc, input| (acc << 1) + input)
}

fn main() {
    let input = parser::input::<String>()
        .trim()
        .chars()
        .filter_map(|c| usize::from_str_radix(&c.to_string(), 16).ok())
        .flat_map(|c| {
            format!("{:04b}", c)
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();

    let packet = parse_packet(&input).0;

    println!("{:?}", packet.version_number());
}

#[derive(Debug)]
pub struct Packet {
    id: usize,
    version: usize,
    kind: Kind,
}

impl Packet {
    pub fn version_number(&self) -> usize {
        self.version
            + match self.kind {
                Kind::Literal(_) => 0,
                Kind::Operator(ref packets) => {
                    packets.iter().map(|packet| packet.version_number()).sum()
                }
            }
    }
}

#[derive(Debug)]
pub enum Kind {
    Literal(usize),
    Operator(Vec<Packet>),
}

fn parse_packet(packet: &[usize]) -> (Packet, &[usize]) {
    let (version, packet) = packet.split_at(3);
    let (id, packet) = packet.split_at(3);
    let (version, id) = (to_int(version), to_int(id));

    let (kind, remaining) = match id {
        4 => parse_literal(packet),
        _ => parse_operator(packet),
    };

    (Packet { id, version, kind }, remaining)
}

fn parse_literal(mut packet: &[usize]) -> (Kind, &[usize]) {
    let mut res = vec![];

    while packet[0] == 1 {
        res.push(packet[1]);
        res.push(packet[2]);
        res.push(packet[3]);
        res.push(packet[4]);
        packet = &packet[5..];
    }
    // when packet[0] == 0 we do it one last time
    res.push(packet[1]);
    res.push(packet[2]);
    res.push(packet[3]);
    res.push(packet[4]);
    packet = &packet[5..];

    (Kind::Literal(to_int(&res)), packet)
}

fn parse_operator(mut packet: &[usize]) -> (Kind, &[usize]) {
    let length_type_id = packet[0];
    packet = &packet[1..];
    match length_type_id {
        0 => parse_length_packet(packet),
        1 => parse_number_packet(packet),
        _ => unreachable!(),
    }
}

fn parse_length_packet(packet: &[usize]) -> (Kind, &[usize]) {
    let (length, packet) = packet.split_at(15);
    let length = to_int(length);

    let (mut sub_packets, remaining) = packet.split_at(length);

    let mut vec = Vec::new();

    while !sub_packets.is_empty() {
        let (p, progress) = parse_packet(sub_packets);
        sub_packets = progress;
        vec.push(p);
    }

    (Kind::Operator(vec), remaining)
}

fn parse_number_packet(packet: &[usize]) -> (Kind, &[usize]) {
    let (nb_packets, mut packet) = packet.split_at(11);
    let nb_packets = to_int(nb_packets);

    let mut vec = Vec::new();

    for _ in 0..nb_packets {
        let (p, progress) = parse_packet(packet);
        packet = progress;
        vec.push(p);
    }

    (Kind::Operator(vec), packet)
}
