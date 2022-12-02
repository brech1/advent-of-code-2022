use std::fs;

// Play options with points
#[derive(PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// Play outcome with points
enum Outcome {
    Draw = 3,
    Win = 6,
}

fn main() {
    // Read file
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Initialize vec
    let mut p1_scores: Vec<u32> = Vec::new();

    // Get points
    let mut get_points = |line: &str| {
        let round = get_p1_round(line);

        let score = get_p1_score(round);

        p1_scores.push(score);

        // Round Two
    };

    // Split lines
    let lines = contents.lines();

    lines.for_each(|x| get_points(x));

    println!("Part One Total: {:?}", p1_scores.into_iter().sum::<u32>());
}

fn get_p1_score(round: (Play, Play)) -> u32 {
    let opponent = round.0;
    let own = round.1;
    let mut score = own as u32;

    if opponent == own {
        return score + Outcome::Draw as u32;
    }

    match opponent {
        Play::Rock => {
            if own == Play::Paper {
                score += Outcome::Win as u32;
            }
        }
        Play::Paper => {
            if own == Play::Scissors {
                score += Outcome::Win as u32;
            }
        }
        Play::Scissors => {
            if own == Play::Rock {
                score += Outcome::Win as u32;
            }
        }
    }

    return score;
}

fn get_p1_round(line: &str) -> (Play, Play) {
    let plays = line.split(" ").collect::<Vec<&str>>();

    return (get_p1_play(plays[0]), get_p1_play(plays[1]));
}

fn get_p1_play(code: &str) -> Play {
    match code {
        "A" => return Play::Rock,
        "B" => return Play::Paper,
        "C" => return Play::Scissors,
        "X" => return Play::Rock,
        "Y" => return Play::Paper,
        "Z" => return Play::Scissors,
        _ => todo!(),
    }
}
