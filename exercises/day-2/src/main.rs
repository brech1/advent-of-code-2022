use std::fs;

// Play options with points
#[derive(PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;

// Wrong guess: 9302

fn main() {
    // Read file
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Initialize vec
    let mut scores: Vec<u32> = Vec::new();

    // Get points
    let mut get_points = |line: &str| {
        let round = get_round(line);

        let score = get_score(round);

        scores.push(score);
    };

    // Split lines
    let lines = contents.lines();

    lines.for_each(|x| get_points(x));

    println!("All hands: {:?}", scores);

    println!("Total: {:?}", scores.into_iter().sum::<u32>());
}

fn get_score(round: (Play, Play)) -> u32 {
    let opponent = round.0;
    let own = round.1;
    let mut score = own as u32;

    if opponent == own {
        return score + DRAW_POINTS;
    }

    match opponent {
        Play::Rock => {
            if own == Play::Paper {
                score += WIN_POINTS;
            }
        }
        Play::Paper => {
            if own == Play::Scissors {
                score += WIN_POINTS;
            }
        }
        Play::Scissors => {
            if own == Play::Rock {
                score += WIN_POINTS;
            }
        }
    }

    return score;
}

fn get_round(line: &str) -> (Play, Play) {
    let plays = line.split(" ").collect::<Vec<&str>>();

    return (get_play(plays[0]), get_play(plays[1]));
}

fn get_play(code: &str) -> Play {
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
