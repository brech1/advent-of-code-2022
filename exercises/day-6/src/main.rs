use std::{collections::VecDeque, fs};

enum Marker {
    Packet = 4,
    Message = 14,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read the file");

    let packet_marker = get_marker_index(&input, Marker::Packet).expect("packet marker not found");

    let message_marker =
        get_marker_index(&input, Marker::Message).expect("message marker not found");

    println!("{packet_marker:?}");
    println!("{message_marker:?}");
}

fn get_marker_index(input: &String, marker: Marker) -> Option<u32> {
    let mut filo_stack: VecDeque<char> = VecDeque::new();

    // Initialize buffer
    let (prefix, message) = input.split_at(marker as usize);

    for prefix_char in prefix.chars() {
        filo_stack.push_front(prefix_char);
    }

    if is_marker(&filo_stack) {
        return Some(marker as u32 + 1);
    }

    for (index, message_char) in message.chars().enumerate() {
        filo_stack.pop_back();
        filo_stack.push_front(message_char);

        if is_marker(&filo_stack) {
            println!("{filo_stack:?}");

            return Some(index as u32 + marker as u32 + 1);
        }
    }

    return None;
}

fn is_marker(stack: &VecDeque<char>) -> bool {
    for (index, pivot) in stack.into_iter().enumerate() {
        // Remove pivot from stack
        let mut substack = stack.clone();

        substack.remove(index);

        for x in substack {
            if *pivot == x {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::{get_marker_index, is_marker, Marker};
    use std::collections::VecDeque;

    #[test]
    fn check_is_marker_correctly() {
        assert_eq!(is_marker(&VecDeque::from(['a', 'a', 'a', 'a'])), false);
        assert_eq!(is_marker(&VecDeque::from(['a', 'b', 'a', 'a'])), false);
        assert_eq!(is_marker(&VecDeque::from(['a', 'b', 'c', 'd'])), true);
    }

    #[test]
    fn get_packet_marker_index_correctly() {
        assert_eq!(
            get_marker_index(
                &String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                Marker::Packet
            ),
            Some(5)
        );
        assert_eq!(
            get_marker_index(
                &String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                Marker::Packet
            ),
            Some(6)
        );
        assert_eq!(
            get_marker_index(
                &String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                Marker::Packet
            ),
            Some(10)
        );
        assert_eq!(
            get_marker_index(
                &String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
                Marker::Packet
            ),
            Some(11)
        );
    }

    #[test]
    fn get_msg_marker_index_correctly() {
        assert_eq!(
            get_marker_index(
                &String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                Marker::Message
            ),
            Some(23)
        );
        assert_eq!(
            get_marker_index(
                &String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                Marker::Message
            ),
            Some(23)
        );
        assert_eq!(
            get_marker_index(
                &String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                Marker::Message
            ),
            Some(29)
        );
        assert_eq!(
            get_marker_index(
                &String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
                Marker::Message
            ),
            Some(26)
        );
    }
}
