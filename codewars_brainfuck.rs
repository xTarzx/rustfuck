enum Operation {
    Inc,
    Dec,
    MvR,
    MvL,
    Out,
    Inp,
    LoopIn,
    LoopOut,
}

struct Command {
    operation: Operation,
    matching_bracket: Option<usize>,
}

fn lex(code: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    
    for c in code.chars() {
        match c {
            '+' => commands.push(Command {
                    operation: Operation::Inc,
                    matching_bracket: None,
                }),
            '-' => commands.push(Command {
                    operation: Operation::Dec,
                    matching_bracket: None,
                }),
            '>' => commands.push(Command {
                    operation: Operation::MvR,
                    matching_bracket: None,
                }),
            '<' => commands.push(Command {
                    operation: Operation::MvL,
                    matching_bracket: None,
                }),
            '.' => commands.push(Command {
                    operation: Operation::Out,
                    matching_bracket: None,
                }),
            ',' => commands.push(Command {
                    operation: Operation::Inp,
                    matching_bracket: None,
                }),
            '[' => commands.push(Command {
                    operation: Operation::LoopIn,
                    matching_bracket: None,
                }),
            ']' => commands.push(Command {
                    operation: Operation::LoopOut,
                    matching_bracket: None,
                }),
                _ => {}
            }
    }
    
    commands
}

fn match_loops(commands: &mut Vec<Command>) {
    let mut loop_stack: Vec<usize> = Vec::new();

    let mut idx: usize = 0;
    while idx < commands.len() {
        let mut command = &mut commands[idx];

        match command.operation {
            Operation::LoopIn => loop_stack.push(idx),
            Operation::LoopOut => {
                let v = loop_stack.pop();
                command.matching_bracket = v;
                commands[v.unwrap()].matching_bracket = Some(idx);
            }
            _ => {}
        }

        idx += 1;
    }
}

const MEMORY_SIZE: usize = 4096;

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut commands = lex(code);
    match_loops(&mut commands);
    
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut sp: usize = 0;
    let mut ip: usize = 0;
    let mut inp: usize = 0;
    
    
    let mut output: Vec<u8> = Vec::new();
    
    while ip < commands.len() {
        let command = &commands[ip];

        match command.operation {
            Operation::Inc => {
                memory[sp] = memory[sp].wrapping_add(1);
                ip += 1;
            }
            Operation::Dec => {
                memory[sp] = memory[sp].wrapping_sub(1);
                ip += 1;
            }
            Operation::MvR => {
                sp += 1;
                ip += 1;
            }
            Operation::MvL => {
                sp -= 1;
                ip += 1;
            }
            Operation::LoopIn => {
                if memory[sp] == 0 {
                    ip = command.matching_bracket.unwrap() + 1;
                    continue;
                }
                ip += 1;
            }
            Operation::LoopOut => {
                if memory[sp] != 0 {
                    ip = command.matching_bracket.unwrap() + 1;
                    continue;
                }
                ip += 1;
            }
            Operation::Out => {
                output.push(memory[sp]);
                ip += 1;
            }
            Operation::Inp => {
                memory[sp] = input[inp];
                inp += 1;
                ip += 1;
            }
        }
    }
    
    
    return output
}
