use std::fs;

type Stack = Vec<char>;
type Supplies = Vec<Stack>;
type Instruction = [u32; 3]; // [crates, init_stack, end_stack]

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("unable to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    // Initialize input vecs
    let mut raw_supplies: Vec<&str> = Vec::new();
    let mut procedure: Vec<&str> = Vec::new();
    let mut is_procedure = false;

    // Split input
    for line in lines {
        if is_procedure {
            procedure.push(line);
        } else {
            if line == "" {
                is_procedure = true;
            } else {
                raw_supplies.push(line)
            }
        }
    }

    let mut supplies: Supplies = parse_supplies(raw_supplies.clone());

    let mut part_two_supplies: Supplies = parse_supplies(raw_supplies);

    order_supplies(procedure.clone(), &mut supplies);

    order_with_cratemover_9001(procedure, &mut part_two_supplies);

    // Print last element of each stack
    for stack in supplies {
        println!("{:?}", stack.last());
    }

    println!("\np2\n");

    for stack in part_two_supplies {
        println!("{:?}", stack.last());
    }
}

fn order_supplies(procedure: Vec<&str>, supplies: &mut Supplies) {
    let mut instructions: Vec<Instruction> = Vec::new();

    // Parse instructions
    for raw_instruction in procedure.iter() {
        instructions.push(
            raw_instruction
                .split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
                .try_into()
                .expect("bad instruction"),
        );
    }

    for [crates, init_stack, end_stack] in instructions {
        for _ in 0..crates {
            let moving_crate = supplies[init_stack as usize - 1]
                .pop()
                .expect("crate stack error");

            supplies[end_stack as usize - 1].push(moving_crate);
        }
    }
}

fn order_with_cratemover_9001(procedure: Vec<&str>, supplies: &mut Supplies) {
    let mut instructions: Vec<Instruction> = Vec::new();

    // Parse instructions
    for raw_instruction in procedure.iter() {
        instructions.push(
            raw_instruction
                .split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
                .try_into()
                .expect("bad instruction"),
        );
    }

    for [crates, init_stack, end_stack] in instructions {
        let init_stack_len = supplies[init_stack as usize - 1].len();

        let mut moving_crates =
            supplies[init_stack as usize - 1].split_off(init_stack_len - crates as usize);

        supplies[end_stack as usize - 1].append(&mut moving_crates);
    }
}

fn parse_supplies(mut input: Vec<&str>) -> Supplies {
    let mut supplies: Supplies = Vec::new();

    // Get stacks
    let stacks: Vec<&str> = input
        .pop()
        .expect("empty initial stack")
        .split_whitespace()
        .collect();

    // Initialize stacks
    for _ in 0..stacks.len() {
        supplies.push(Vec::new())
    }

    for line in input.iter().rev() {
        for (index, ch) in line.chars().enumerate() {
            if ch.is_ascii_uppercase() {
                supplies[(index - 1) / 4].push(ch)
            }
        }
    }

    return supplies;
}

#[cfg(test)]
mod tests {
    use crate::parse_supplies;
    use std::fs;

    #[test]
    fn parse_initial_supplies_correctly() {
        let input = fs::read_to_string("test_supplies.txt").expect("unable to read the file");

        let supplies_vec: Vec<&str> = input.lines().collect();

        assert_eq!(
            parse_supplies(supplies_vec),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D',], vec!['P']]
        );
    }
}
