use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut position = 50;
    let mut count = 0;

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let (start, number) = line.split_at_checked(1).ok_or("UTF8 Error")?;

        let number = number.parse::<usize>()?;

        match start {
            "L" => {
                let number = 100 - (number % 100);
                position = (position + number) % 100;
                if position == 0 {
                    count += 1;
                }
            }
            "R" => {
                let number = number % 100;
                position = (position + number) % 100;
                if position == 0 {
                    count += 1;
                }
            }
            _ => Err("Wrong input!")?,
        }
    }

    println!("Output: {count}");

    Ok(())
}
