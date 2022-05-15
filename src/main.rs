use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
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

struct Loc {
    line: usize,
    col: usize,
}

struct Command {
    operation: Operation,
    loc: Loc,
    matching_bracket: Option<usize>,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.matching_bracket {
            None => {
                write!(
                    f,
                    "{:?} @ {}:{}",
                    self.operation, self.loc.line, self.loc.col
                )
            }
            _ => {
                write!(
                    f,
                    "{:?} -> {} @ {}:{}",
                    self.operation,
                    self.matching_bracket.unwrap(),
                    self.loc.line,
                    self.loc.col
                )
            }
        }
    }
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

    // for (idx, command) in commands.iter_mut().enumerate() {
    //     match command.operation {
    //         Operation::LoopIn => loop_stack.push(idx),
    //         Operation::LoopOut => {
    //             let v = loop_stack.pop();
    //             command.matching_bracket = v;
    //             // commands[v.unwrap()].matching_bracket = Some(idx);
    //         }
    //         _ => {}
    //     }
    // }
}

const MEMORY_SIZE: usize = 30000;

fn parse(commands: &mut Vec<Command>) {
    match_loops(commands);

    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut sp: usize = 0;
    let mut ip: usize = 0;

    while ip < commands.len() {
        let command = &commands[ip];

        match command.operation {
            Operation::Inc => {
                memory[sp] += 1;
                ip += 1;
            }
            Operation::Dec => {
                memory[sp] -= 1;
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
                print!("{}", memory[sp] as char);
                ip += 1
            }
            Operation::Inp => panic!("not implemented"),
        }
    }

    println!("");
    // println!("{:?}", commands);
}

fn lex(fd: &File) -> io::Result<Vec<Command>> {
    let reader = BufReader::new(fd);

    let mut commands: Vec<Command> = Vec::new();

    for (line, line_content) in reader.lines().enumerate() {
        for (col, chr) in line_content?.chars().enumerate() {
            match chr {
                '+' => commands.push(Command {
                    operation: Operation::Inc,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                '-' => commands.push(Command {
                    operation: Operation::Dec,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                '>' => commands.push(Command {
                    operation: Operation::MvR,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                '<' => commands.push(Command {
                    operation: Operation::MvL,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                '.' => commands.push(Command {
                    operation: Operation::Out,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                ',' => commands.push(Command {
                    operation: Operation::Inp,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                '[' => commands.push(Command {
                    operation: Operation::LoopIn,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                ']' => commands.push(Command {
                    operation: Operation::LoopOut,
                    loc: Loc { line, col },
                    matching_bracket: None,
                }),
                _ => {}
            }
        }
    }

    Ok(commands)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no program specified");
        std::process::exit(0);
    }

    let program_path: &String = args.get(1).unwrap();

    let file: File = File::open(program_path)?;

    let mut commands = lex(&file)?;
    parse(&mut commands);

    Ok(())
}
