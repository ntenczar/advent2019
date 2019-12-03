use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILENAME: &'static str = "input";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILENAME)?;
    let mut input: Vec<usize> = Vec::new();
    for line in BufReader::new(file).lines() {
        // there's only one line but whatever
        let l = line.unwrap();
        let line_vals = l.split(",").collect::<Vec<&str>>();
        for v in line_vals {
            input.push(v.parse::<usize>().unwrap());
        }
    }

    part_one(input.clone());
    part_two(input, 19690720);

    Ok(())
}

fn part_one(input: Vec<usize>) {
    let mut cloned_input = input.clone();
    let final_input = run_prog(&mut cloned_input);
    println!("Part one: {}", final_input[0]);
}

fn part_two(input: Vec<usize>, magic_num: usize) {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut input = input.clone();
            input[1] = i;
            input[2] = j;
            let output = run_prog(&mut input);
            if output[0] == magic_num {
                println!("Big yeet: {}", 100 * i + j);
            }
        }
    }
}

fn run_prog(input: &mut Vec<usize>) -> &mut Vec<usize> {
    let jump = 4;
    let mut pos: usize = 0;

    while pos < input.len() {
        let opcode = input[pos];
        match opcode {
            1 => do_op(input, pos, &add),
            2 => do_op(input, pos, &mult),
            99 => {
                break;
            }
            o => panic!(format!("Unexpected opcode: {} encountered!", o)),
        }
        pos += jump;
    }

    return input;
}

fn do_op(input: &mut Vec<usize>, pos: usize, f: &dyn Fn(usize, usize) -> usize) {
    let result = f(input[input[pos + 1]], input[input[pos + 2]]);
    let val_pos = input[pos + 3];
    input[val_pos] = result;
}

fn add(v1: usize, v2: usize) -> usize {
    v1 + v2
}

fn mult(v1: usize, v2: usize) -> usize {
    v1 * v2
}

#[test]
fn test_run_prog() {
    let mut input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(run_prog(&mut input), &mut output);
}
