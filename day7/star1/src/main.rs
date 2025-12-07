use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let matrix = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(str::bytes)
        .map(Iterator::collect::<Vec<_>>)
        .collect::<Vec<_>>();

    let mut sum = 0;

    let start = matrix[0].iter().enumerate().find_map(|(i, c)| (*c == b'S').then_some(i)).expect("Missing S!!!");

    let mut beams = vec![false; matrix[0].len()];
    beams[start] = true;

    for y in 1..matrix.len() {
        for x in 0..beams.len() {
            if beams[x] && matrix[y][x] == b'^' {
                beams[x] = false;
                beams[x-1] = true;// check unneccessary due to input
                beams[x+1] = true;
                sum += 1;
            }
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
