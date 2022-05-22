# Brainfuck interpreter in Rust

can compile to x86_64 linux executable

## quickstart:

```console
$cargo build && cp target/debug/bf ./
$./bf
```

## usage:

```
bf [sim|com] program_path
    sim    simlutate the program
    com    compile to native x86_64 linux executable (doesnt link automatically)
```
