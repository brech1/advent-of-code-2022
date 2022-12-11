use std::{collections::HashMap, fs};

const KNOTS: u32 = 9;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, Clone)]
struct Rope {
    head: Position,
    knots: Vec<Knot>,
}

#[derive(Debug, Clone)]
struct Knot {
    pos: Position,
    history: HashMap<Position, bool>,
}

impl Position {
    fn get_distance(&self, new_position: &Position) -> [u32; 2] {
        return [
            self.x.abs_diff(new_position.x),
            self.y.abs_diff(new_position.y),
        ];
    }

    fn follow(&self, head: &Position) -> Position {
        let mut new_pos = self.clone();

        let [x_distance, y_distance] = self.get_distance(&head);

        if x_distance > 1 {
            if self.x > head.x {
                new_pos.x -= 1;
            } else {
                new_pos.x += 1;
            }

            if y_distance == 1 {
                if self.y > head.y {
                    new_pos.y -= 1;
                } else {
                    new_pos.y += 1;
                }
            }
        }

        if y_distance > 1 {
            if self.y > head.y {
                new_pos.y -= 1;
            } else {
                new_pos.y += 1;
            }

            if x_distance == 1 {
                if self.x > head.x {
                    new_pos.x -= 1;
                } else {
                    new_pos.x += 1;
                }
            }
        }

        return new_pos;
    }
}

impl Rope {
    fn new() -> Rope {
        let mut knots = Vec::new();

        for _ in 0..KNOTS {
            let mut history: HashMap<Position, bool> = HashMap::new();
            history.insert(Position { x: 0, y: 0 }, true);

            knots.push(Knot {
                pos: Position { x: 0, y: 0 },
                history,
            })
        }

        Rope {
            head: Position { x: 0, y: 0 },
            knots,
        }
    }

    fn move_head(&mut self, command: Command) {
        for _ in 0..command.steps {
            match command.direction {
                Direction::Left => self.head.x -= 1,
                Direction::Right => self.head.x += 1,
                Direction::Up => self.head.y += 1,
                Direction::Down => self.head.y -= 1,
            }

            println!("\n\n-------------------------\n\n");
            println!("Head -- {:?}", self.head);

            // Update knots
            for i in 0..self.knots.len() {
                let new_pos: Position;

                if i == 0 {
                    new_pos = self.knots[i].pos.follow(&self.head);
                } else {
                    new_pos = self.knots[i].pos.follow(&self.knots[i - 1].pos)
                }

                self.knots[i].pos = new_pos.clone();

                self.knots[i].history.insert(new_pos, true);

                println!("Knot {}, {:?}", i + 1, self.knots[i].pos);
            }
        }
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize Ropes
    let mut rope = Rope::new();

    // Execute commands

    for line in lines {
        rope.move_head(parse_command(line));
    }

    println!("Part one: {}", rope.knots[0].history.len());
    println!("Part two: {}", rope.knots[8].history.len());
}

fn parse_command(line: &str) -> Command {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    let direction = match split_line[0] {
        "L" => Some(Direction::Left),
        "R" => Some(Direction::Right),
        "U" => Some(Direction::Up),
        "D" => Some(Direction::Down),
        _ => None,
    }
    .expect("Not a valid direction");

    let steps = split_line[1].parse::<u32>().expect("NaN");

    return Command { direction, steps };
}
