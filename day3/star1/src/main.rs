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
        let (first, index) = chars.clone()
            .take(count - 1)
            .enumerate()
            .fold(('0', 0), |(ac, aci), (ni, n)| if n as usize > ac as usize { (n, ni) } else { (ac, aci) });

        let second = chars
            .skip(index + 1)
            .fold('0', |acc, n| if n as usize > acc as usize { n } else { acc });

        sum += (first as usize - '0' as usize) * 10 + (second as usize - '0' as usize);
    }

    println!("Sum: {sum}");

    Ok(())
}
