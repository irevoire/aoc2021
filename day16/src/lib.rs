fn to_int(input: &[usize]) -> usize {
    input.iter().fold(0, |acc, input| (acc << 1) + input)
}

#[derive(Debug)]
pub struct Packet {
    _id: usize,
    version: usize,
    kind: Kind,
}

impl Packet {
    pub fn version_number(&self) -> usize {
        self.version
            + match self.kind {
                Kind::Literal(_) => 0,
                Kind::Sum(ref packets)
                | Kind::Product(ref packets)
                | Kind::Minimum(ref packets)
                | Kind::Maximum(ref packets)
                | Kind::GreaterThan(ref packets)
                | Kind::LessThan(ref packets)
                | Kind::EqualTo(ref packets) => {
                    packets.iter().map(|packet| packet.version_number()).sum()
                }
            }
    }

    pub fn value(&self) -> usize {
        match self.kind {
            Kind::Literal(v) => v,
            Kind::Sum(ref packets) => packets.iter().map(|packet| packet.value()).sum(),
            Kind::Product(ref packets) => packets.iter().map(|packet| packet.value()).product(),
            Kind::Minimum(ref packets) => {
                packets.iter().map(|packet| packet.value()).min().unwrap()
            }
            Kind::Maximum(ref packets) => {
                packets.iter().map(|packet| packet.value()).max().unwrap()
            }
            Kind::GreaterThan(ref packets) => (packets[0].value() > packets[1].value()) as usize,
            Kind::LessThan(ref packets) => (packets[0].value() < packets[1].value()) as usize,
            Kind::EqualTo(ref packets) => (packets[0].value() == packets[1].value()) as usize,
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

pub fn parse_packet(packet: &[usize]) -> (Packet, &[usize]) {
    let (version, packet) = packet.split_at(3);
    let (id, packet) = packet.split_at(3);
    let (version, id) = (to_int(version), to_int(id));

    let (kind, remaining) = match id {
        4 => parse_literal(packet),
        n => {
            let (packets, rem) = parse_operator(packet);
            let kind = match n {
                0 => Kind::Sum(packets),
                1 => Kind::Product(packets),
                2 => Kind::Minimum(packets),
                3 => Kind::Maximum(packets),
                5 => Kind::GreaterThan(packets),
                6 => Kind::LessThan(packets),
                7 => Kind::EqualTo(packets),
                _ => unreachable!(),
            };
            (kind, rem)
        }
    };

    (
        Packet {
            _id: id,
            version,
            kind,
        },
        remaining,
    )
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

fn parse_operator(mut packet: &[usize]) -> (Vec<Packet>, &[usize]) {
    let length_type_id = packet[0];
    packet = &packet[1..];
    match length_type_id {
        0 => parse_length_packet(packet),
        1 => parse_number_packet(packet),
        _ => unreachable!(),
    }
}

fn parse_length_packet(packet: &[usize]) -> (Vec<Packet>, &[usize]) {
    let (length, packet) = packet.split_at(15);
    let length = to_int(length);

    let (mut sub_packets, remaining) = packet.split_at(length);

    let mut vec = Vec::new();

    while !sub_packets.is_empty() {
        let (p, progress) = parse_packet(sub_packets);
        sub_packets = progress;
        vec.push(p);
    }

    (vec, remaining)
}

fn parse_number_packet(packet: &[usize]) -> (Vec<Packet>, &[usize]) {
    let (nb_packets, mut packet) = packet.split_at(11);
    let nb_packets = to_int(nb_packets);

    let mut vec = Vec::new();

    for _ in 0..nb_packets {
        let (p, progress) = parse_packet(packet);
        packet = progress;
        vec.push(p);
    }

    (vec, packet)
}
