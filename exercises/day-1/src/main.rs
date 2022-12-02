use std::fs;

fn main() {
    // Read file
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Initialize variables
    let mut elves_calories: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    // Split lines
    let lines = contents.lines();

    // Evaluate calories
    let mut eval_cals = |cals: &str| {
        if cals == "" {
            elves_calories.push(sum);
            sum = 0;
        } else {
            let int_cals: u32 = cals.parse().unwrap();
            sum += int_cals;
        }
    };

    lines.for_each(|x| eval_cals(x));

    // Push last sum
    elves_calories.push(sum);

    // Sort calories
    elves_calories.sort();

    println!(
        "The elf with the most cals have {}",
        elves_calories.last().unwrap()
    );

    // Part Two - Get the top three elves total cals

    let last_three_cals: u32 = elves_calories.iter().rev().take(3).sum();

    println!("The last three elves carry {} cals", last_three_cals);
}
