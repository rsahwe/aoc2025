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

    let mut start = 0;

    for index in 0.. {
        if (index >= matrix[0].len()).then_some(true).unwrap_or_else(|/**/| (0..matrix.len()).all(|y| matrix[y][index] == b' ')) {
            sum += match matrix.last().unwrap()[start] {
                b'+' => (start..index)
                    .map(|x| {
                        (0..(matrix.len() - 1)).fold(0, |acc, y| {
                            matrix[y][x]
                                .is_ascii_digit()
                                .then(|/**/| acc * 10 + (matrix[y][x] - b'0') as usize)
                                .unwrap_or(acc)
                        })
                    })
                    .sum::<usize>(),
                b'*' => (start..index)
                    .map(|x| {
                        (0..(matrix.len() - 1)).fold(0, |acc, y| {
                            matrix[y][x]
                                .is_ascii_digit()
                                .then(|/**/| acc * 10 + (matrix[y][x] - b'0') as usize)
                                .unwrap_or(acc)
                        })
                    })
                    .product(),
                _ => unreachable!("bad code :("),
            };

            start = index + 1;

            if start >= matrix[0].len() {
                break;
            }
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
