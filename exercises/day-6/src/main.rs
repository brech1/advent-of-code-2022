use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read the file");

    let index = get_marker_index(input).expect("marker not found");

    println!("{index:?}");
}

fn get_marker_index(input: String) -> Option<u32> {
    let mut filo_stack: VecDeque<char> = VecDeque::new();

    // Initialize buffer
    let (prefix, message) = input.split_at(4);

    for prefix_char in prefix.chars() {
        filo_stack.push_front(prefix_char);
    }

    if is_marker(&filo_stack) {
        return Some(4);
    }

    for (index, message_char) in message.chars().enumerate() {
        filo_stack.pop_back();
        filo_stack.push_front(message_char);

        if is_marker(&filo_stack) {
            println!("{filo_stack:?}");

            return Some(index as u32 + 5);
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
    use crate::{get_marker_index, is_marker};
    use std::collections::VecDeque;

    #[test]
    fn check_is_marker_correctly() {
        assert_eq!(is_marker(&VecDeque::from(['a', 'a', 'a', 'a'])), false);
        assert_eq!(is_marker(&VecDeque::from(['a', 'b', 'a', 'a'])), false);
        assert_eq!(is_marker(&VecDeque::from(['a', 'b', 'c', 'd'])), true);
    }

    #[test]
    fn get_marker_index_correctly() {
        assert_eq!(
            get_marker_index(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            Some(5)
        );
        assert_eq!(
            get_marker_index(String::from("nppdvjthqldpwncqszvftbrmjlhg")),
            Some(6)
        );
        assert_eq!(
            get_marker_index(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            Some(10)
        );
        assert_eq!(
            get_marker_index(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            Some(11)
        );
    }
}
