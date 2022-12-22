use std::{cmp::Ordering, fs};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Two lists
            (Self::List(a), Self::List(b)) => a.cmp(b),
            // List and number
            (Self::List(a), Self::Integer(b)) => a.cmp(&vec![Self::Integer(*b)]),
            // Number and list
            (Self::Integer(a), Self::List(b)) => vec![Self::Integer(*a)].cmp(&b),
            // Two Numbers
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get pairs
    let pairs = input.split("\n\n");

    // Pair index vec
    let mut good_packets: Vec<u32> = Vec::new();
    let mut index = 0;

    // Process pairs
    for pair in pairs {
        index += 1;

        if process_pair(pair) {
            good_packets.push(index);
            println!("Good packet: {index}");
        };
    }

    println!("Sum: {}", good_packets.iter().sum::<u32>());
}

fn process_pair(lines: &str) -> bool {
    let mut processed_packets: Vec<Packet> = Vec::new();

    for line in lines.split("\n") {
        let (packet, _) = process_packet(line);

        processed_packets.push(packet);
    }

    if processed_packets[1] < processed_packets[0] {
        return false;
    }

    return true;
}

fn process_packet(list: &str) -> (Packet, &str) {
    let mut local_packets = Vec::new();
    // Trim first char
    let mut unparsed = list.split_at(1).1;

    loop {
        match unparsed.chars().nth(0).unwrap() {
            // Return packets
            ']' => {
                trim_start(&mut unparsed);
                break;
            }
            // Trim comma
            ',' => trim_start(&mut unparsed),
            // Proccess packet
            '[' => {
                let (new_packet, new_unparsed) = process_packet(unparsed);
                local_packets.push(new_packet);
                unparsed = new_unparsed;
            }
            // Process number
            _ => {
                let (new_number, new_unparsed) = process_number(unparsed);
                local_packets.push(new_number);
                unparsed = new_unparsed;
            }
        }
    }

    // Return packets and unparsed
    (Packet::List(local_packets), unparsed)
}

fn trim_start(string: &mut &str) {
    *string = string.split_at(1).1;
}

fn process_number(unparsed: &str) -> (Packet, &str) {
    let splitted: Vec<&str> = unparsed.split(&[',', ']']).collect();

    let raw_number = splitted[0];

    let parsed_int: u32 = raw_number.parse().unwrap();

    let new_unparsed = unparsed.split_at(raw_number.len()).1;

    (Packet::Integer(parsed_int), new_unparsed)
}
