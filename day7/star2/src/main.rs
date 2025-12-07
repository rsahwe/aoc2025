use std::{collections::HashMap, error::Error, fs};

fn check_recursive(matrix: &Vec<Vec<u8>>, beam: usize, y: usize, hm: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(val) = hm.get(&(beam, y)) {
        *val
    } else {
        let val = if y >= matrix.len() {
            1
        } else {
            if matrix[y][beam] == b'^' {
                check_recursive(matrix, beam - 1, y + 1, hm) + check_recursive(matrix, beam + 1, y + 1, hm)
            } else {
                check_recursive(matrix, beam, y + 1, hm)
            }
        };
        hm.insert((beam, y), val);
        val
    }
}

fn do_input(input: &str) -> usize {
    let matrix = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(str::bytes)
        .map(Iterator::collect::<Vec<_>>)
        .collect::<Vec<_>>();

    let start = matrix[0].iter().enumerate().find_map(|(i, c)| (*c == b'S').then_some(i)).expect("Missing S!!!");

    let mut hm = HashMap::new();

    check_recursive(&matrix, start, 1, &mut hm)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    println!("Sum: {}", do_input(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use crate::do_input;

    #[test]
    fn test_input() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("../inputt")?;

        assert_eq!(do_input(&input), 40);

        Ok(())
    }
}
