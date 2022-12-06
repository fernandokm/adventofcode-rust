use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 16);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let data = input
        .trim()
        .chars()
        .filter_map(|c| Some(c.to_digit(16)? as u64))
        .chunks(16)
        .into_iter()
        .map(|c| {
            c.pad_using(16, |_| 0)
                .reduce(|acc, x| (acc << 4) + x)
                .unwrap()
        })
        .collect_vec();
    let packet = Packet::parse(&mut BitStream::new(data)).context("invalid input")?;

    out.set_part1(packet.version_sum());
    out.set_part2(packet.value());

    Ok(())
}

struct BitStream {
    data: Vec<u64>,
    current: u64,
    bits_left: usize,
    total_bits_read: u64,
}

impl BitStream {
    pub fn new(mut data: Vec<u64>) -> Self {
        data.reverse();
        Self {
            data,
            current: 0,
            bits_left: 0,
            total_bits_read: 0,
        }
    }

    pub fn read_u64(&mut self, n: usize) -> Option<u64> {
        assert!(n <= 64, "Cannot read more that 64 bits into a u64");

        if n <= self.bits_left {
            let val = self.read_u64_from_current(n);
            Some(val)
        } else {
            let missing = n - self.bits_left;
            let val = self.read_u64_from_current(self.bits_left);
            self.current = self.data.pop()?;
            self.bits_left = 64;
            let val = (val << missing) + self.read_u64_from_current(missing);
            Some(val)
        }
    }

    fn read_u64_from_current(&mut self, n: usize) -> u64 {
        self.bits_left -= n;
        self.total_bits_read += n as u64;
        (self.current >> self.bits_left) & ((1 << n) - 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum OpType {
    Add,
    Mul,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl OpType {
    pub fn parse(id: u64) -> Option<OpType> {
        Some(match id {
            0 => Self::Add,
            1 => Self::Mul,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => None?,
        })
    }

    pub fn apply(self, values: impl Iterator<Item = u64>) -> u64 {
        fn comparison(mut values: impl Iterator<Item = u64>, f: impl Fn(u64, u64) -> bool) -> u64 {
            u64::from(f(values.next().unwrap(), values.next().unwrap()))
        }

        match self {
            OpType::Add => values.sum(),
            OpType::Mul => values.product(),
            OpType::Min => values.min().unwrap(),
            OpType::Max => values.max().unwrap(),
            OpType::Gt => comparison(values, |x, y| x > y),
            OpType::Lt => comparison(values, |x, y| x < y),
            OpType::Eq => comparison(values, |x, y| x == y),
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operation(OpType, Vec<Packet>),
}

impl PacketData {
    pub fn parse(s: &mut BitStream) -> Option<PacketData> {
        match s.read_u64(3)? {
            4 => Some(Self::Literal(Self::parse_literal(s)?)),
            id => Some(Self::Operation(
                OpType::parse(id)?,
                Self::parse_subpackets(s)?,
            )),
        }
    }

    pub fn parse_literal(s: &mut BitStream) -> Option<u64> {
        let mut v = 0;
        loop {
            let block = s.read_u64(5)?;
            v = (v << 4) | (block & ((1 << 4) - 1));
            if block & (1 << 4) == 0 {
                break;
            }
        }
        Some(v)
    }

    pub fn parse_subpackets(s: &mut BitStream) -> Option<Vec<Packet>> {
        let length_type_id = s.read_u64(1)?;
        match length_type_id {
            0 => {
                let target_length = s.read_u64(15)?;
                let mut v = Vec::new();
                let mut length = 0;
                while length < target_length {
                    let p = Packet::parse(s)?;
                    length += p.bitlen;
                    v.push(p);
                }
                Some(v)
            }
            1 => {
                let target_count = s.read_u64(11)?;
                (0..target_count).map(|_| Packet::parse(s)).collect()
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    pub version: u64,
    pub data: PacketData,
    pub bitlen: u64,
}

impl Packet {
    pub fn parse(s: &mut BitStream) -> Option<Self> {
        let initial_bits = s.total_bits_read;

        Some(Packet {
            version: s.read_u64(3)?,
            data: PacketData::parse(s)?,
            bitlen: s.total_bits_read - initial_bits,
        })
    }

    fn version_sum(&self) -> u64 {
        self.version
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Operation(_, subpackets) => {
                    subpackets.iter().map(Packet::version_sum).sum::<u64>()
                }
            }
    }

    pub fn value(&self) -> u64 {
        match &self.data {
            &PacketData::Literal(val) => val,
            PacketData::Operation(op, subpackets) => op.apply(subpackets.iter().map(Packet::value)),
        }
    }
}
