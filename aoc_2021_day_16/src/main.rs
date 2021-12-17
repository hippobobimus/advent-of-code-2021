use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut data = parse(path).unwrap();
    let pkt = data.get_packet();

    let res_1 = pkt.sum_versions();
    let res_2 = pkt.evaluate();

    println!("*-*-*-*-*- Day 16 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parse(path: &Path) -> io::Result<Data> {
    let file = File::open(path)?;
    let mut buf = BufReader::new(file);

    let mut input = String::new();
    buf.read_line(&mut input)?;
    let data = Data::new(&input);

    Ok(data)
}

struct Data {
    bit_vec: Vec<u8>,
    pos: usize,
}

impl Data {
    fn new(input: &str) -> Self {
        let bit_vec = input
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|x| {
                ((x[0].to_digit(16).unwrap() << 4) + x[1].to_digit(16).unwrap()) as u8
            })
            .collect::<Vec<u8>>();

        Self { bit_vec, pos: 0 }
    }

    fn parse_n_bits(&mut self, n: usize) -> usize {
        let mut result = 0;

        for i in (0..n).rev() {
            result += (self.next_bit() as usize) << i;
        }

        result
    }

    fn next_bit(&mut self) -> u8 {
        let chunk = self.pos / 8;
        let idx = self.pos % 8;

        let result = if (self.bit_vec[chunk] & (128 >> idx)) == 0 {
            0
        } else {
            1
        };

        self.pos += 1;

        result
    }

    fn get_packet(&mut self) -> Packet {
        let header = Header {
            ver: self.parse_n_bits(3),
            type_id: self.parse_n_bits(3),
        };

        let mut sub_pkts: Vec<Packet> = vec![];

        let body = if header.type_id == 4 {
            // contains a literal value.
            let mut literal = 0;

            while self.parse_n_bits(1) == 1 {
                literal <<= 4;
                literal += self.parse_n_bits(4);
            }
            literal <<= 4;
            literal += self.parse_n_bits(4);

            Body::Literal(literal)
        } else {
            // an operation on sub-packets.
            let len_type_id = self.parse_n_bits(1);

            // recursively parse sub-packets.
            match len_type_id {
                0 => {
                    let total_subpkt_len = self.parse_n_bits(15);
                    let start_pos = self.pos;

                    while (self.pos - start_pos) < total_subpkt_len {
                        sub_pkts.push(self.get_packet());
                    }
                },
                1 => {
                    let num_subpkts = self.parse_n_bits(11);

                    for _ in 0..num_subpkts {
                        sub_pkts.push(self.get_packet());
                    }
                },
                _ => unreachable!(),
            }

            Body::Operator
        };

        Packet { header, body, sub_pkts }
    }

}

#[derive(Debug)]
struct Packet {
    header: Header,
    body: Body,
    sub_pkts: Vec<Packet>,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        let mut result = self.header.ver;

        for subpkt in self.sub_pkts.iter() {
            result += subpkt.sum_versions();
        }

        result
    }

    fn evaluate(&self) -> usize {
        match self.body {
            Body::Literal(val) => val,
            Body::Operator => self.evaluate_operation(),
        }
    }

    fn evaluate_operation(&self) -> usize {
        match self.header.type_id {
            0 => self.sub_pkts.iter().map(|p| p.evaluate()).sum(),
            1 => self.sub_pkts.iter().map(|p| p.evaluate()).product(),
            2 => self.sub_pkts.iter().map(|p| p.evaluate()).min().unwrap(),
            3 => self.sub_pkts.iter().map(|p| p.evaluate()).max().unwrap(),
            5 => {
                if self.sub_pkts[0].evaluate() > self.sub_pkts[1].evaluate() {
                    1
                } else {
                    0
                }
            },
            6 => {
                if self.sub_pkts[0].evaluate() < self.sub_pkts[1].evaluate() {
                    1
                } else {
                    0
                }
            },
            7 => {
                if self.sub_pkts[0].evaluate() == self.sub_pkts[1].evaluate() {
                    1
                } else {
                    0
                }
            },
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
struct Header {
    ver: usize,
    type_id: usize,
}

#[derive(Debug)]
enum Body {
    Literal(usize),
    Operator,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "EE00D40C823060";
    const INPUT_1: &str = "38006F45291200";
    const INPUT_2: &str = "8A004A801A8002F478";
    const INPUT_3: &str = "620080001611562C8802118E34";
    const INPUT_4: &str = "C0015000016115A2E0802F182340";
    const INPUT_5: &str = "A0016C880162017C3686B18A3D4780";

    const PT2_INPUT_0: &str = "C200B40A82";
    const PT2_INPUT_1: &str = "04005AC33890";
    const PT2_INPUT_2: &str = "880086C3E88112";
    const PT2_INPUT_3: &str = "CE00C43D881120";
    const PT2_INPUT_4: &str = "D8005AC2A8F0";
    const PT2_INPUT_5: &str = "F600BC2D8F";
    const PT2_INPUT_6: &str = "9C005AC2F8F0";
    const PT2_INPUT_7: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_data_generation() {
        let mut data = Data::new(INPUT_2);
        let expected = vec![
            0b10001010,
            0b00000000,
            0b01001010,
            0b10000000,
            0b00011010,
            0b10000000,
            0b00000010,
            0b11110100,
            0b01111000,
        ];

        for (expected, chunk) in data.bit_vec.iter().zip(expected.iter()) {
            print!("{:0>8b} ", chunk);
            assert_eq!(expected, chunk);
        }

        assert_eq!(4, data.parse_n_bits(3));
        assert_eq!(2, data.parse_n_bits(3));
        assert_eq!(1, data.parse_n_bits(1));
        assert_eq!(1, data.parse_n_bits(11));
        assert_eq!(1, data.parse_n_bits(3));
        assert_eq!(2, data.parse_n_bits(3));
        assert_eq!(1, data.parse_n_bits(1));
        assert_eq!(1, data.parse_n_bits(11));
        assert_eq!(5, data.parse_n_bits(3));
        assert_eq!(2, data.parse_n_bits(3));
        assert_eq!(0, data.parse_n_bits(1));
        assert_eq!(11, data.parse_n_bits(15));
        assert_eq!(6, data.parse_n_bits(3));
        assert_eq!(4, data.parse_n_bits(3));
        assert_eq!(15, data.parse_n_bits(5));
    }

    #[test]
    fn test_part_1() {
        let mut data_0 = Data::new(INPUT_0);
        let mut data_1 = Data::new(INPUT_1);
        let mut data_2 = Data::new(INPUT_2);
        let mut data_3 = Data::new(INPUT_3);
        let mut data_4 = Data::new(INPUT_4);
        let mut data_5 = Data::new(INPUT_5);

        let pkt_0 = data_0.get_packet();
        let pkt_1 = data_1.get_packet();
        let pkt_2 = data_2.get_packet();
        let pkt_3 = data_3.get_packet();
        let pkt_4 = data_4.get_packet();
        let pkt_5 = data_5.get_packet();

        assert_eq!(14, pkt_0.sum_versions());
        assert_eq!(9, pkt_1.sum_versions());
        assert_eq!(16, pkt_2.sum_versions());
        assert_eq!(12, pkt_3.sum_versions());
        assert_eq!(23, pkt_4.sum_versions());
        assert_eq!(31, pkt_5.sum_versions());
    }

    #[test]
    fn test_part_2() {
        let mut data_0 = Data::new(PT2_INPUT_0);
        let mut data_1 = Data::new(PT2_INPUT_1);
        let mut data_2 = Data::new(PT2_INPUT_2);
        let mut data_3 = Data::new(PT2_INPUT_3);
        let mut data_4 = Data::new(PT2_INPUT_4);
        let mut data_5 = Data::new(PT2_INPUT_5);
        let mut data_6 = Data::new(PT2_INPUT_6);
        let mut data_7 = Data::new(PT2_INPUT_7);

        let pkt_0 = data_0.get_packet();
        let pkt_1 = data_1.get_packet();
        let pkt_2 = data_2.get_packet();
        let pkt_3 = data_3.get_packet();
        let pkt_4 = data_4.get_packet();
        let pkt_5 = data_5.get_packet();
        let pkt_6 = data_6.get_packet();
        let pkt_7 = data_7.get_packet();

        assert_eq!(3, pkt_0.evaluate());
        assert_eq!(54, pkt_1.evaluate());
        assert_eq!(7, pkt_2.evaluate());
        assert_eq!(9, pkt_3.evaluate());
        assert_eq!(1, pkt_4.evaluate());
        assert_eq!(0, pkt_5.evaluate());
        assert_eq!(0, pkt_6.evaluate());
        assert_eq!(1, pkt_7.evaluate());
    }
}
