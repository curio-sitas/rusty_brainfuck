// ! Brainfuck interpreter

use std::char;
use std::env;
use std::fs;

const MEM_CAPACITY: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage : {} <filename>
        * <filename> : brainfuck file to interpret."
        , args[0]);
        return;
    }

    let ref filepath = args[1];

    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");

    let mut memory: [u32; MEM_CAPACITY] = [0; MEM_CAPACITY];
    let content = contents.chars().collect();

    // ! add a cleanup code

    parse(content, &mut memory);
}

fn parse(content: Vec<char>, mem: &mut [u32; MEM_CAPACITY]) {
    let mut ptr: usize = 0;
    let len = content.len();
    let mut idx: usize = 0;

    let bracemap: Vec<usize> = buildbracemap(&content);

    while idx < len {
        let c = content[idx];

        if c == '>' && ptr < MEM_CAPACITY {
            ptr += 1
        }
        if c == '<' && ptr > 0 {
            ptr -= 1
        }
        if c == '+' {
            mem[ptr] += 1
        }
        if c == '-' && mem[ptr] > 0 {
            mem[ptr] -= 1
        }
        if c == '.' {
            print!("{}", char::from_u32(mem[ptr]).unwrap())
        }
        if c == ',' {
            let input = String::new();
            let bytes = input.bytes().nth(0).expect("no byte read");
            mem[ptr] = bytes as u32;
        }
        if c == '[' && mem[ptr] == 0 {
            idx = bracemap[idx]
        }
        if c == ']' && mem[ptr] != 0 {
            idx = bracemap[idx]
        }

        idx += 1
    }
    // ! Inifinte brace while loop
    // TODO create bracemap according to https://github.com/pocmo/Python-Brainfuck/blob/master/brainfuck.py
}

fn buildbracemap(content: &Vec<char>) -> Vec<usize> {
    let code = &*content;

    let mut tmp_stack: Vec<usize> = Vec::new();
    let mut bracemap: Vec<usize> = Vec::with_capacity(code.len());
    for _i in 0..code.len() {
        bracemap.push(0);
    }

    for (i, &c) in code.iter().enumerate() {
        if c == '[' {
            tmp_stack.push(i);
        }
        if c == ']' {
            let start = tmp_stack.pop().unwrap();
            bracemap[start] = i;
            bracemap[i] = start;
        }
    }
    return bracemap;
}
