use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize sum
    let mut total_overlapped: u32 = 0;

    for line in lines {
        if is_assignment_contained(line) {
            total_overlapped += 1;
        }
    }

    println!("{}", total_overlapped);
}

fn is_assignment_contained(line: &str) -> bool {
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
        println!("First contains second - {:?}", sections);
        return true;
    }

    // Check if the second section contains the first
    if second_section.contains(&first_section.start())
        && second_section.contains(&first_section.end())
    {
        println!("Second contains first - {:?}", sections);
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::is_assignment_contained;

    #[test]
    fn checks_assignments() {
        assert_eq!(is_assignment_contained("2-4,6-8"), false);
        assert_eq!(is_assignment_contained("2-3,4-5"), false);
        assert_eq!(is_assignment_contained("5-7,7-9"), false);
        assert_eq!(is_assignment_contained("2-8,3-7"), true);
        assert_eq!(is_assignment_contained("6-6,4-6"), true);
        assert_eq!(is_assignment_contained("2-6,4-8"), false);
    }
}
