use std::{collections::HashMap, fs};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn get_new_position(&self, direction: Direction) -> Option<Position> {
        let mut new_pos = self.clone();

        match direction {
            Direction::Up => new_pos.y += 1,
            Direction::Right => new_pos.x += 1,
            Direction::Down => {
                if new_pos.y > 0 {
                    new_pos.y -= 1;
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if new_pos.x > 0 {
                    new_pos.x -= 1;
                } else {
                    return None;
                }
            }
        }

        return Some(new_pos);
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct HeightMap {
    grid: HashMap<Position, u32>,
    start: Position,
    end: Position,
    part_two_starts: Vec<Position>,
}

impl HeightMap {
    fn new() -> HeightMap {
        HeightMap {
            grid: HashMap::new(),
            start: Position { x: 0, y: 0 },
            end: Position { x: 0, y: 0 },
            part_two_starts: Vec::new(),
        }
    }

    fn add_position(&mut self, x: u32, y: u32, elevation: u32) {
        self.grid.insert(Position { x, y }, elevation);
    }

    fn map(&self) {
        let mut mapped_positions: HashMap<Position, u32> = HashMap::new();

        mapped_positions.insert(self.start.clone(), 0);

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        while !mapped_positions.contains_key(&self.end) {
            // Map surroundings
            for pos in mapped_positions.clone().keys() {
                // Get current position distance and elevation
                let current_distance = mapped_positions.get(pos).unwrap().clone();
                let current_elevation = self.grid.get(pos).unwrap().clone();

                for direction in directions.clone() {
                    // Check existence
                    let new_pos_option = pos.get_new_position(direction);

                    if new_pos_option.is_none() {
                        continue;
                    }

                    let new_pos = new_pos_option.unwrap();

                    // Check if visited
                    if mapped_positions.contains_key(&new_pos) {
                        continue;
                    }

                    let test_pos_exists = self.grid.get(&new_pos);

                    if test_pos_exists.is_none() {
                        continue;
                    }

                    // Check if can be visited
                    let new_elevation = test_pos_exists.unwrap().clone();

                    if current_elevation + 1 >= new_elevation {
                        mapped_positions.insert(new_pos, current_distance + 1);
                    }
                }
            }
        }

        println!("Steps: {:?}", mapped_positions.get(&self.end));
    }

    fn part_two_map(&self) {
        let mut distances: Vec<u32> = Vec::new();

        for start in self.part_two_starts.clone() {
            let mut mapped_positions: HashMap<Position, u32> = HashMap::new();
            // Prevent dobule checking surroundings for already mapped positions
            let mut surroundings_checked: HashMap<Position, bool> = HashMap::new();

            mapped_positions.insert(start, 0);

            let directions = [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ];

            while !mapped_positions.contains_key(&self.end) {
                let checked = surroundings_checked.len();

                // Map surroundings
                for pos in mapped_positions.clone().keys() {
                    if surroundings_checked.contains_key(pos) {
                        continue;
                    }

                    // Get current position distance and elevation
                    let current_distance = mapped_positions.get(pos).unwrap().clone();
                    let current_elevation = self.grid.get(pos).unwrap().clone();

                    for direction in directions.clone() {
                        // Check existence
                        let new_pos_option = pos.get_new_position(direction);

                        if new_pos_option.is_none() {
                            continue;
                        }

                        let new_pos = new_pos_option.unwrap();

                        // Check if visited
                        if mapped_positions.contains_key(&new_pos) {
                            continue;
                        }

                        let test_pos_exists = self.grid.get(&new_pos);

                        if test_pos_exists.is_none() {
                            continue;
                        }

                        // Check if can be visited
                        let new_elevation = test_pos_exists.unwrap().clone();

                        if current_elevation + 1 >= new_elevation {
                            mapped_positions.insert(new_pos, current_distance + 1);
                        }
                    }

                    surroundings_checked.insert(pos.clone(), true);
                }

                if checked == surroundings_checked.len() {
                    break;
                }
            }

            let dis = mapped_positions.get(&self.end);

            if dis.is_some() {
                println!("Steps: {:?}", dis);

                distances.push(*dis.unwrap());
            }
        }

        distances.sort();

        println!("Steps: {:?}", distances);
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get map rows
    let raw_map: Vec<&str> = input.lines().collect();

    let mut height_map = HeightMap::new();

    load_map(raw_map, &mut height_map);

    println!("Start: {:?}", height_map.start);
    println!("End {:?}", height_map.end);

    // height_map.map();
    height_map.part_two_map();
}

fn load_map(raw_map: Vec<&str>, height_map: &mut HeightMap) {
    for (y, row) in raw_map.iter().enumerate() {
        for (x, elevation) in row.chars().enumerate() {
            match elevation {
                'S' => {
                    height_map.start.x = x as u32;
                    height_map.start.y = y as u32;
                    height_map.add_position(x as u32, y as u32, 'a' as u32)
                }
                'E' => {
                    height_map.end.x = x as u32;
                    height_map.end.y = y as u32;
                    height_map.add_position(x as u32, y as u32, 'z' as u32)
                }
                'a' => {
                    height_map.part_two_starts.push(Position {
                        x: x as u32,
                        y: y as u32,
                    });
                    height_map.add_position(x as u32, y as u32, 'a' as u32)
                }
                _ => height_map.add_position(x as u32, y as u32, elevation as u32),
            }
        }
    }
}
