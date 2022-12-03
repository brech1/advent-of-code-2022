use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize priorities vec
    let mut priorities: Vec<u32> = Vec::new();

    for line in lines {
        priorities.push(get_priority(line));
    }

    println!("{:?}", priorities.into_iter().sum::<u32>());
}

fn get_priority(line: &str) -> u32 {
    let repeated_char = get_repeated_char(line).expect("Same char not found");

    // I'll use the ASCII table to get the priorities
    // A-Z -> 65-90
    // a-z -> 97-122
    let char_ascii_value = repeated_char as u32;
    let priority: u32;

    if char_ascii_value > 96 {
        priority = char_ascii_value - 96;
    } else {
        priority = char_ascii_value - 65 + 27;
    }

    return priority;
}

fn get_repeated_char(line: &str) -> Option<char> {
    let halves = line.split_at(line.len() / 2);

    for char_a in halves.0.chars() {
        for char_b in halves.1.chars() {
            if char_a == char_b {
                return Some(char_a);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::get_priority;

    #[test]
    fn get_your_priorities_right() {
        assert_eq!(get_priority("aa"), 1);
        assert_eq!(get_priority("zz"), 26);
        assert_eq!(get_priority("AA"), 27);
        assert_eq!(get_priority("ZZ"), 52);
    }
}
