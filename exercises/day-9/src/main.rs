use std::{collections::HashMap, fs};

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

struct Command {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, Clone)]
struct Rope {
    head: Position,
    tail: Position,
    tail_history: HashMap<Position, bool>,
}

impl Position {
    fn get_distance(&self, new_position: &Position) -> [u32; 2] {
        return [
            self.x.abs_diff(new_position.x),
            self.y.abs_diff(new_position.y),
        ];
    }
}

impl Rope {
    fn new() -> Rope {
        let mut tail_history: HashMap<Position, bool> = HashMap::new();

        tail_history.insert(Position { x: 0, y: 0 }, true);

        Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            tail_history,
        }
    }

    fn move_head(&mut self, command: Command) {
        match command.direction {
            Direction::Left => self.move_left(command.steps),
            Direction::Right => self.move_right(command.steps),
            Direction::Up => self.move_up(command.steps),
            Direction::Down => self.move_down(command.steps),
        }
    }

    fn move_left(&mut self, steps: u32) {
        for _ in 0..steps {
            self.head.x -= 1;

            // Update tail
            // Check distance
            let [x_distance, y_distance] = self.head.get_distance(&self.tail);

            if x_distance > 1 || y_distance > 1 {
                // Check if same row
                if self.head.y == self.tail.y {
                    self.tail.x -= 1;
                } else {
                    self.tail.x -= 1;
                    self.tail.y = self.head.y;
                }

                // Record Tail Position
                self.tail_history.insert(self.tail.clone(), true);
            }
        }
    }

    fn move_right(&mut self, steps: u32) {
        for _ in 0..steps {
            self.head.x += 1;

            // Update tail
            // Check distance
            let [x_distance, y_distance] = self.head.get_distance(&self.tail);

            if x_distance > 1 || y_distance > 1 {
                // Check if same row
                if self.head.y == self.tail.y {
                    self.tail.x += 1;
                } else {
                    self.tail.x += 1;
                    self.tail.y = self.head.y;
                }

                // Record Tail Position
                self.tail_history.insert(self.tail.clone(), true);
            }
        }
    }

    fn move_up(&mut self, steps: u32) {
        for _ in 0..steps {
            self.head.y += 1;

            // Update tail
            // Check distance
            let [x_distance, y_distance] = self.head.get_distance(&self.tail);

            if x_distance > 1 || y_distance > 1 {
                // Check if same column
                if self.head.x == self.tail.x {
                    self.tail.y += 1;
                } else {
                    self.tail.y += 1;
                    self.tail.x = self.head.x;
                }

                // Record Tail Position
                self.tail_history.insert(self.tail.clone(), true);
            }
        }
    }

    fn move_down(&mut self, steps: u32) {
        for _ in 0..steps {
            self.head.y -= 1;

            // Update tail
            // Check distance
            let [x_distance, y_distance] = self.head.get_distance(&self.tail);

            if x_distance > 1 || y_distance > 1 {
                // Check if same column
                if self.head.x == self.tail.x {
                    self.tail.y -= 1;
                } else {
                    self.tail.y -= 1;
                    self.tail.x = self.head.x;
                }

                // Record Tail Position
                self.tail_history.insert(self.tail.clone(), true);
            }
        }
    }
}

// if the head and tail aren't touching and aren't in the same row or column
// the tail always moves one step diagonally to keep up

// 6351 - too high

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Init Rope
    let mut rope = Rope::new();

    for line in lines {
        rope.move_head(parse_command(line));
    }

    println!("{}", rope.tail_history.len());
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
