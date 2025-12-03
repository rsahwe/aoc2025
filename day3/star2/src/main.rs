use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut sum = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let chars = line.chars();
        let count = chars.clone().count();

        let mut skip = 0;
        let mut number = 0;

        for i in 0..12 {
            let (index, chr) = chars
                .clone()
                .enumerate()
                .skip(skip)
                .take(count + i + 1 - skip - 12)
                .fold((0, '0'), |(ai, ac), (ni, nc)| if nc as usize > ac as usize { (ni, nc) } else { (ai, ac) });

            skip = index + 1;
            number = number * 10 + chr as usize - '0' as usize;
        }

        sum += number;
    }

    println!("Sum: {sum}");

    Ok(())
}
