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
}

const MEMORY_SIZE: usize = 30000;

fn simulate(commands: &mut Vec<Command>) {
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
            Operation::Inp => {
                let mut userinput: [u8; 1] = [0];
                std::io::stdin().read(&mut userinput).unwrap();
                // println!("{}", userinput[0]);
                memory[sp] = userinput[0];
                ip += 1
            }
        }
    }

    println!("");
    // println!("{:?}", commands);
}

fn compile(commands: &mut Vec<Command>) {
    match_loops(commands);

    let mut file = File::create("./output.asm").unwrap();

    // HEADER
    writeln!(file, "section .text").unwrap();
    writeln!(file, "%define SYS_exit 60").unwrap();
    writeln!(file, "%define SYS_write 1").unwrap();
    writeln!(file, "%define SYS_exit 60").unwrap();
    writeln!(file, "%define STDIN  0").unwrap();
    writeln!(file, "%define STDOUT 1").unwrap();
    writeln!(file, "%define STDERR 2").unwrap();
    writeln!(file, "global _start").unwrap();
    writeln!(file, "_start:").unwrap();

    for (idx, command) in commands.iter().enumerate() {
        match command.operation {
            Operation::Inc => {
                writeln!(file, "    ;; -- INC").unwrap();
                // calculate offset
                writeln!(file, "    mov r8, stack").unwrap();
                writeln!(file, "    add r8, [stackpointer]").unwrap();

                // get value and increment
                writeln!(file, "    mov r9, [r8]").unwrap();
                writeln!(file, "    inc r9").unwrap();

                // mov it back to stack+offset
                writeln!(file, "    mov [r8], r9").unwrap();
            }
            Operation::Dec => {
                writeln!(file, "    ;; -- DEC").unwrap();
                // calculate offset
                writeln!(file, "    mov r8, stack").unwrap();
                writeln!(file, "    add r8, [stackpointer]").unwrap();

                // get value and decrement
                writeln!(file, "    mov r9, [r8]").unwrap();
                writeln!(file, "    dec r9").unwrap();

                // mov it back to stack+offset
                writeln!(file, "    mov [r8], r9").unwrap();
            }
            Operation::MvR => {
                writeln!(file, "    ;; -- MvR").unwrap();
                writeln!(file, "    mov r8, [stackpointer]").unwrap();
                writeln!(file, "    inc r8").unwrap();
                writeln!(file, "    mov [stackpointer], r8").unwrap();
            }
            Operation::MvL => {
                writeln!(file, "    ;; -- MvL").unwrap();
                writeln!(file, "    mov r8, [stackpointer]").unwrap();
                writeln!(file, "    dec r8").unwrap();
                writeln!(file, "    mov [stackpointer], r8").unwrap();
            }
            Operation::LoopIn => {
                writeln!(file, "    mov r8, stack").unwrap();
                writeln!(file, "    add r8, [stackpointer]").unwrap();
                writeln!(file, "    cmp byte [r8], 0").unwrap();
                writeln!(file, "    je label{}", command.matching_bracket.unwrap()).unwrap();
                writeln!(file, "label{}:", idx).unwrap();
            }
            Operation::LoopOut => {
                writeln!(file, "    mov r8, stack").unwrap();
                writeln!(file, "    add r8, [stackpointer]").unwrap();
                writeln!(file, "    cmp byte [r8], 0").unwrap();
                writeln!(file, "    jne label{}", command.matching_bracket.unwrap()).unwrap();
                writeln!(file, "label{}:", idx).unwrap();
            }
            Operation::Out => {
                // calculate offset
                writeln!(file, "    mov r8, stack").unwrap();
                writeln!(file, "    add r8, [stackpointer]").unwrap();

                // prepare syscall
                writeln!(file, "    mov rax, SYS_write").unwrap();
                writeln!(file, "    mov rdi, STDOUT").unwrap();
                writeln!(file, "    mov rsi, r8").unwrap();
                writeln!(file, "    mov rdx, 1").unwrap();
                writeln!(file, "    syscall").unwrap();
            }
            Operation::Inp => {
                assert!(false, "not implemented")
            }
        }
    }

    // FOOTER
    writeln!(file, "    mov rax, SYS_write").unwrap();
    writeln!(file, "    mov rdi, STDOUT").unwrap();
    writeln!(file, "    mov rsi, newline").unwrap();
    writeln!(file, "    mov rdx, 1").unwrap();
    writeln!(file, "    syscall").unwrap();

    writeln!(file, "    mov rax, SYS_exit").unwrap();
    writeln!(file, "    mov rdi, 0").unwrap();
    writeln!(file, "    syscall").unwrap();
    writeln!(file, "section .data").unwrap();
    writeln!(file, "    newline: db 10").unwrap();
    writeln!(file, "segment .bss").unwrap();
    writeln!(file, "    stack: resb 30000").unwrap();
    writeln!(file, "    stackpointer: resb 1").unwrap();
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

fn usage(program_name: String) {
    println!("usage: {} [sim|com] program_path", program_name)
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().rev().collect();

    let program_name = args.pop().unwrap();

    if args.len() < 1 {
        usage(program_name);
        eprintln!("ERROR: no command specified");
        std::process::exit(1);
    }

    let command = args.pop().unwrap();

    if args.len() < 1 {
        usage(program_name);
        eprintln!("ERROR: no program specified");
        std::process::exit(1);
    }
    let program_path: String = args.pop().unwrap();
    let file: File = File::open(program_path)?;
    let mut commands = lex(&file)?;

    match command.as_str() {
        "sim" => {
            simulate(&mut commands);
        }

        "com" => {
            compile(&mut commands);

            std::process::Command::new("nasm")
                .arg("-f")
                .arg("elf64")
                .arg("./output.asm")
                .spawn()
                .expect("error running nasm");
        }
        other => {
            usage(program_name);
            eprintln!("ERROR: unknown command `{}`", other);
            std::process::exit(1);
        }
    }

    Ok(())
}
