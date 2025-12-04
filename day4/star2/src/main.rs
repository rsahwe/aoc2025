use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let grid = input.lines().map(str::trim).filter(|line| line.len() != 0).map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut previous = grid;
    let mut next = previous.clone();
    let mut cont = true;
    let mut sum = 0;

    while cont {
        previous = next;
        next = previous.clone();

        cont = false;

        for y in 0..height {
            for x in 0..width {
                if previous[y][x] != '@' {
                    continue;
                }

                let mut local_sum = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        local_sum += (previous.get(y.wrapping_add_signed(dy)).map(|r| r.get(x.wrapping_add_signed(dx))).flatten().unwrap_or(&'.') == &'@') as usize;
                    }
                }

                if local_sum < 5 {
                    next[y][x] = '.';
                    sum += 1;
                    cont = true;
                }
            }
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
