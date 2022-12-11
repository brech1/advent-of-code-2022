use std::fs;

#[derive(Debug, Clone)]
enum Instruction {
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
struct Operation {
    instruction: Instruction,
    value: i32,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u32,
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    throw: [u32; 2],
    simian_shenanigans: u32,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation: Operation {
                instruction: Instruction::ADD,
                value: 0,
            },
            test: 0,
            throw: [0; 2],
            simian_shenanigans: 0,
        }
    }
}

#[derive(Debug)]
struct Forest {
    monkeys: Vec<Monkey>,
}

impl Forest {
    fn new() -> Forest {
        Forest {
            monkeys: Vec::new(),
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn run(&mut self) {
        let mut monkeys = self.monkeys.clone();

        for i in 0..self.monkeys.len() {
            for item in monkeys[i].items.clone() {
                let mut mut_item = item.clone();

                match monkeys[i].operation.instruction {
                    Instruction::ADD => {
                        if monkeys[i].operation.value == -1 {
                            mut_item += item;
                        } else {
                            mut_item += monkeys[i].operation.value as u32;
                        }
                    }
                    Instruction::MUL => {
                        if monkeys[i].operation.value == -1 {
                            mut_item *= item;
                        } else {
                            mut_item *= monkeys[i].operation.value as u32;
                        }
                    }
                };

                let worry_level = mut_item / 3;
                let divisible_throw = monkeys[i].throw[0] as usize;
                let not_divisible_throw = monkeys[i].throw[1] as usize;

                if worry_level % monkeys[i].test == 0 {
                    monkeys[divisible_throw].items.push(worry_level);
                } else {
                    monkeys[not_divisible_throw].items.push(worry_level);
                }

                monkeys[i].simian_shenanigans += 1;
            }

            monkeys[i].items.clear();
        }

        self.monkeys = monkeys;
    }

    fn get_shenanigans(&self) {
        let mut count: Vec<u32> = Vec::new();

        for monkey in &self.monkeys {
            count.push(monkey.simian_shenanigans);
        }

        count.sort();

        count.reverse();

        println!("{:?}", count[0] * count[1]);
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut forest = Forest::new();

    // Get input lines
    let lines: Vec<&str> = input.lines().collect();

    let monkey_chunks = lines.chunks(7);

    for monkey in monkey_chunks {
        forest.add_monkey(parse_monkey(monkey));
    }

    for _ in 0..20 {
        forest.run();
    }

    forest.get_shenanigans();
}

fn parse_monkey(monkey_chunk: &[&str]) -> Monkey {
    let mut monkey = Monkey::new();
    let mut operation: Operation = Operation {
        instruction: Instruction::ADD,
        value: 0,
    };
    let mut parsed_lines: Vec<Vec<u32>> = Vec::new();

    for (index, line) in monkey_chunk.into_iter().enumerate() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let mut trimmed_line: Vec<u32> = Vec::new();

        if index == 2 {
            if split_line[4] == "*" {
                operation.instruction = Instruction::MUL;
            }

            if split_line[5] == "old" {
                operation.value = -1;
            } else {
                operation.value = split_line[5].parse().unwrap();
            }
        }

        for item in &split_line {
            let mut output: String = String::new();

            for char_a in item.chars() {
                if char_a.to_digit(10).is_some() {
                    output.push(char_a);
                }
            }

            if output.len() > 0 {
                trimmed_line.push(output.parse().unwrap());
            }
        }

        parsed_lines.push(trimmed_line);
    }

    monkey.id = parsed_lines[0][0];
    monkey.items = parsed_lines[1].clone();
    monkey.operation = operation;
    monkey.test = parsed_lines[3][0];
    monkey.throw = [parsed_lines[4][0], parsed_lines[5][0]];

    return monkey;
}
