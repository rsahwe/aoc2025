use std::{error::Error, fs, ops::Range};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;
    let mut lines = input.lines().map(str::trim);

    let ranges = lines
        .by_ref()
        .take_while(|line| line.len() != 0)
        .map(|r| r.split_once('-').expect("Invalid range!!!"))
        .map(|r| (r.0.parse().expect("Invalid range!!!"))..(r.1.parse().expect("Invalid range!!!")))
        .collect::<Vec<Range<usize>>>();

    Ok(println!("Sum: {}",
        lines
            .map(str::parse)
            .flatten()
            .filter(|n| ranges.iter().any(|r| r.contains(&n)))
            .count()
    ))
}
