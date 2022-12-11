use std::fs;

// Instruction - Cycles
enum Instruction {
    NOOP = 1,
    ADDX = 2,
}

struct Operation {
    instruction: Instruction,
    data: Option<i32>,
}

impl Operation {
    fn new(instruction: Instruction, data: Option<i32>) -> Operation {
        Operation { instruction, data }
    }
}

struct CPU {
    register: i32,
    cycle_count: u32,
    magic_memory: Vec<i32>,
    crt: Vec<char>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            register: 1,
            cycle_count: 0,
            magic_memory: vec![1],
            crt: Vec::new(),
        }
    }

    fn run(&mut self, program: Vec<Operation>) {
        for operation in program {
            // Update memory for current instruction cycles
            for _ in 0..operation.instruction as u32 {
                if self.cycle_count > 39 {
                    self.cycle_count = 0
                }

                self.magic_memory.push(self.register);

                if self.cycle_count.abs_diff(self.register as u32) < 2 {
                    self.crt.push('#');
                } else {
                    self.crt.push('.');
                }

                self.cycle_count += 1;
            }

            // Execute instruction
            match operation.instruction {
                Instruction::ADDX => self.register += operation.data.expect("no instruction data"),
                Instruction::NOOP => (),
            }
        }
    }

    fn get_signal_strength(&self, cycles: Vec<i32>) {
        let mut strength: i32 = 0;

        for cycle in cycles {
            let cycle_strength = self.magic_memory[cycle as usize] * cycle;

            strength += cycle_strength;

            println!(
                "cycle {} - register {} - {} strength",
                cycle, self.magic_memory[cycle as usize], cycle_strength
            );
        }

        println!("total: {}", strength);
    }

    fn draw(&self) {
        let lines = self
            .crt
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        println!("{lines}")
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Start cpu
    let mut cpu = CPU::new();

    let interesting_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    // Get input lines
    let lines = input.lines().into_iter();

    let mut program: Vec<Operation> = Vec::new();

    // Execute commands
    for line in lines {
        let op = parse_operation(line).expect("wrong instruction");
        program.push(op);
    }

    cpu.run(program);

    println!("\n-- Part One --\n");

    cpu.get_signal_strength(interesting_cycles);

    println!("\n-- Part Two --\n");

    cpu.draw();

    println!("\n-----\n");
}

fn parse_operation(line: &str) -> Option<Operation> {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    match split_line[0] {
        "noop" => Some(Operation::new(Instruction::NOOP, None)),
        "addx" => {
            let value = split_line[1].parse().expect("NaN");

            Some(Operation::new(Instruction::ADDX, Some(value)))
        }
        _ => None,
    }
}
