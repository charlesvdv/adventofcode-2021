use std::iter::Peekable;

use bitvec::{
    prelude::*,
    slice::{BitSliceIndex, Iter},
};

#[derive(Debug, Clone)]
enum Packet {
    Literal(u8, usize),
    Sum(u8, Vec<Packet>),
    Product(u8, Vec<Packet>),
    Minimum(u8, Vec<Packet>),
    Maximum(u8, Vec<Packet>),
    GreaterThan(u8, Box<Packet>, Box<Packet>),
    LessThan(u8, Box<Packet>, Box<Packet>),
    Equal(u8, Box<Packet>, Box<Packet>),
}

fn parse_number<'a, T, O, S>(input: &mut T, size: usize) -> usize
where
    O: BitOrder,
    S: BitStore,
    T: Iterator<Item = <usize as BitSliceIndex<'a, O, S>>::Immut>,
{
    let mut number = 0;
    for _ in 0..size {
        number = number << 1;
        number |= *input.next().unwrap() as usize;
    }

    number
}

fn parse<'a, T, O, S>(input: &mut Peekable<T>) -> Packet
where
    O: BitOrder,
    S: BitStore,
    T: Iterator<Item = <usize as BitSliceIndex<'a, O, S>>::Immut> + Sized,
{
    let version = parse_number(input, 3) as u8;
    let type_id = parse_number(input, 3);

    if type_id == 4 {
        let mut last = false;
        let mut accum: BitVec = BitVec::new();
        while !last {
            last = !*input.next().unwrap();
            (0..4).for_each(|_| accum.push(*input.next().unwrap()))
        }

        Packet::Literal(version, parse_number(&mut accum.iter(), accum.len()))
    } else {
        let length_type_id = *input.next().unwrap();
        let mut packets = Vec::new();
        if length_type_id {
            let number_of_packets = parse_number(input, 11);
            for _ in 0..number_of_packets {
                packets.push(parse(input));
            }
        } else {
            let number_of_bits = parse_number(input, 15);

            let mut subset: BitVec = BitVec::new();
            for _ in 0..number_of_bits {
                subset.push(*input.next().unwrap());
            }

            let mut subset_iter = subset.iter().peekable();
            while subset_iter.peek().is_some() {
                packets.push(parse(&mut subset_iter));
            }
        }

        match type_id {
            0 => Packet::Sum(version, packets),
            1 => Packet::Product(version, packets),
            2 => Packet::Minimum(version, packets),
            3 => Packet::Maximum(version, packets),
            5 => Packet::GreaterThan(
                version,
                Box::new(packets[0].clone()),
                Box::new(packets[1].clone()),
            ),
            6 => Packet::LessThan(
                version,
                Box::new(packets[0].clone()),
                Box::new(packets[1].clone()),
            ),
            7 => Packet::Equal(
                version,
                Box::new(packets[0].clone()),
                Box::new(packets[1].clone()),
            ),
            _ => unreachable!(),
        }
    }
}

fn get_version_total(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(version, _) => *version as usize,
        Packet::Sum(version, child_packets)
        | Packet::Product(version, child_packets)
        | Packet::Minimum(version, child_packets)
        | Packet::Maximum(version, child_packets) => {
            *version as usize + child_packets.iter().map(get_version_total).sum::<usize>()
        }
        Packet::GreaterThan(version, packet_a, packet_b)
        | Packet::LessThan(version, packet_a, packet_b)
        | Packet::Equal(version, packet_a, packet_b) => {
            *version as usize
                + [packet_a, packet_b]
                    .iter()
                    .map(|packet| get_version_total(packet.as_ref()))
                    .sum::<usize>()
        }
    }
}

fn compute(packet: &Packet) -> isize {
    match packet {
        Packet::Literal(_, value) => *value as isize,
        Packet::Sum(_, packets) => packets.iter().map(compute).sum::<isize>(),
        Packet::Product(_, packets) => packets.iter().map(compute).product::<isize>(),
        Packet::Minimum(_, packets) => packets.iter().map(compute).min().unwrap(),
        Packet::Maximum(_, packets) => packets.iter().map(compute).max().unwrap(),
        Packet::GreaterThan(_, packet_a, packet_b) => {
            (compute(packet_a.as_ref()) > compute(packet_b.as_ref())) as isize
        }
        Packet::LessThan(_, packet_a, packet_b) => {
            (compute(packet_a.as_ref()) < compute(packet_b.as_ref())) as isize
        }
        Packet::Equal(_, packet_a, packet_b) => {
            (compute(packet_a.as_ref()) == compute(packet_b.as_ref())) as isize
        }
    }
}

fn main() {
    let input_str = include_str!("../input.txt");

    let mut input: BitVec = BitVec::new();
    input_str
        .chars()
        .map(|v| v.to_digit(16).unwrap() as u8)
        .for_each(|v| input.extend_from_bitslice(&v.view_bits::<Msb0>()[4..8]));

    let root_packet = parse(&mut input.iter().peekable());

    println!("part 1: {}", get_version_total(&root_packet));
    println!("part 2: {}", compute(&root_packet));
}
