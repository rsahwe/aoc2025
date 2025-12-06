use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let matrix = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() != 0)
        .map(str::split_whitespace)
        .map(Iterator::collect::<Vec<_>>)
        .collect::<Vec<_>>();

    let mut sum = 0;

    for x in 0..matrix[0].len() {
        let operator = matrix.last().expect("Huh?")[x];

        sum += match operator {
            "*" => (0..(matrix.len() - 1))
                .fold(1, |acc, y| matrix[y][x].parse::<usize>().expect("Int error!!!") * acc),
            "+" => (0..(matrix.len() - 1))
                .fold(0, |acc, y| matrix[y][x].parse::<usize>().expect("Int error!!!") + acc),
            _ => unreachable!("-_-"),
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
