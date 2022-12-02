use std::fs;

fn main() {
    // Read file
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Split lines
    let lines = contents.lines();

    // Assign empty vectors
    let mut elves_calories: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

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

    // Get max value
    let max_cals = elves_calories.iter().max().unwrap();

    println!("{max_cals}");
}
