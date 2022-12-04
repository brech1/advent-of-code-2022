use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize sum
    let mut full_overlap: u32 = 0;
    let mut partial_overlap: u32 = 0;

    for line in lines {
        if is_full_overlap(line) {
            full_overlap += 1;
        }

        if is_partial_overlap(line) {
            partial_overlap += 1;
        }
    }

    println!("Full overlap - {}", full_overlap);
    println!("Partial overlap - {}", partial_overlap);
}

fn is_full_overlap(line: &str) -> bool {
    let sections: Vec<u32> = line
        .split(&['-', ','])
        .map(|section| section.parse::<u32>().expect("NaN"))
        .collect();

    // Setup section ranges
    let first_section = sections[0]..=sections[1];
    let second_section = sections[2]..=sections[3];

    // Check if the first section contains the second
    if first_section.contains(&second_section.start())
        && first_section.contains(&second_section.end())
    {
        return true;
    }

    // Check if the second section contains the first
    if second_section.contains(&first_section.start())
        && second_section.contains(&first_section.end())
    {
        return true;
    }

    return false;
}

fn is_partial_overlap(line: &str) -> bool {
    let sections: Vec<u32> = line
        .split(&['-', ','])
        .map(|section| section.parse::<u32>().expect("NaN"))
        .collect();

    // Setup section ranges
    let first_section = sections[0]..=sections[1];
    let second_section = sections[2]..=sections[3];

    // Check if the first section overlaps with the second
    if first_section.contains(&second_section.start())
        || first_section.contains(&second_section.end())
    {
        return true;
    }

    // Check if the second section overlaps with the first
    if second_section.contains(&first_section.start())
        || second_section.contains(&first_section.end())
    {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::{is_full_overlap, is_partial_overlap};

    #[test]
    fn check_full_overlap() {
        assert_eq!(is_full_overlap("2-4,6-8"), false);
        assert_eq!(is_full_overlap("2-3,4-5"), false);
        assert_eq!(is_full_overlap("5-7,7-9"), false);
        assert_eq!(is_full_overlap("2-8,3-7"), true);
        assert_eq!(is_full_overlap("6-6,4-6"), true);
        assert_eq!(is_full_overlap("2-6,4-8"), false);
    }

    #[test]
    fn check_partial_overlap() {
        assert_eq!(is_partial_overlap("2-4,6-8"), false);
        assert_eq!(is_partial_overlap("2-3,4-5"), false);
        assert_eq!(is_partial_overlap("5-7,7-9"), true);
        assert_eq!(is_partial_overlap("2-8,3-7"), true);
        assert_eq!(is_partial_overlap("6-6,4-6"), true);
        assert_eq!(is_partial_overlap("2-6,4-8"), true);
    }
}
