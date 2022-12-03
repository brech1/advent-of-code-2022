use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize priorities vec
    let mut priorities: Vec<u32> = Vec::new();

    // Initialize group priorities vec
    let mut group_priorities: Vec<u32> = Vec::new();

    // Initialize line groups vec - part 2
    let mut line_group: Vec<&str> = Vec::new();
    let mut group_index: u8 = 0;

    for line in lines {
        let repeated_char = get_repeated_char(line).expect("Same char not found");

        priorities.push(get_priority(repeated_char));

        // Part two
        group_index += 1;
        line_group.push(line);

        if group_index > 2 {
            group_index = 0;

            let group_char =
                get_repeated_char_for_lines(&line_group).expect("Group char not found");

            group_priorities.push(get_priority(group_char));

            line_group.clear();
        }
    }

    println!(
        "Single line priority: {:?}",
        priorities.into_iter().sum::<u32>()
    );

    println!(
        "Lines group priority: {:?}",
        group_priorities.into_iter().sum::<u32>()
    );
}

// Gets the priority for a given char
// Uses the ASCII table to get the priorities
// A-Z -> 65-90
// a-z -> 97-122
fn get_priority(ch: char) -> u32 {
    let char_ascii_value = ch as u32;
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

fn get_repeated_char_for_lines(lines_vec: &Vec<&str>) -> Option<char> {
    let lines = lines_vec.clone();

    for char_a in lines[0].chars() {
        for char_b in lines[1].chars() {
            for char_c in lines[2].chars() {
                if char_a == char_b && char_a == char_c {
                    return Some(char_a);
                }
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::{get_priority, get_repeated_char_for_lines};

    #[test]
    fn get_your_priorities_right() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn get_repeated_char_for_lines_right() {
        assert_eq!(
            get_repeated_char_for_lines(&vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]),
            Some('r')
        );
        assert_eq!(
            get_repeated_char_for_lines(&vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]),
            Some('Z')
        );
    }
}
