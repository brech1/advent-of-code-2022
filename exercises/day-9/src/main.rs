use std::{collections::HashMap, fs};

const KNOTS: u32 = 10;

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

    for line in lines.clone() {
        rope.move_head(parse_command(line));
    }

    println!("Part one: {}", rope.tail_history.len());

    // Part Two -- Simulate knots with ropes

    // Initialize Ropes
    let mut ropes: Vec<Rope> = Vec::new();

    for _ in 0..KNOTS {
        ropes.push(Rope::new());
    }

    // Execute commands

    for line in lines.clone() {
        ropes[0].move_head(parse_command(line));

        for i in 1..KNOTS as usize {
            // Adjust head for the tail of the next rope
            let next_tail = ropes[i - 1].tail.clone();

            // Get difference from head with next tail
            let x_difference = ropes[i].head.x.abs_diff(next_tail.x);
            let y_difference = ropes[i].head.y.abs_diff(next_tail.y);

            if x_difference > 0 {
                if next_tail.x > ropes[i].head.x {
                    ropes[i].move_head(Command {
                        direction: Direction::Right,
                        steps: x_difference,
                    });
                } else {
                    ropes[i].move_head(Command {
                        direction: Direction::Left,
                        steps: x_difference,
                    });
                }
            }

            if y_difference > 0 {
                if next_tail.y > ropes[i].head.y {
                    ropes[i].move_head(Command {
                        direction: Direction::Up,
                        steps: y_difference,
                    });
                } else {
                    ropes[i].move_head(Command {
                        direction: Direction::Down,
                        steps: y_difference,
                    });
                }
            }

            if ropes[i].head != next_tail {
                println!("not working")
            }
        }
    }

    println!("Part two: {}", ropes[9].tail_history.len());

    // 2909 - Too high
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
