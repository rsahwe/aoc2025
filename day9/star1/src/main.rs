use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;
    // let input = fs::read_to_string("../inputt")?;

    let points = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(',').expect("Invalid point!!!"))
        .map(|(x, y)| (x.parse().expect("Invalid int!!!"), y.parse().expect("Invalid int!!!")))
        .collect::<Vec<(usize, usize)>>();

    let sum = (0..(points.len() - 1))
        .map(|a| {
            ((a + 1)..points.len())
                .map(|b| (points[a].0.abs_diff(points[b].0) + 1) * (points[a].1.abs_diff(points[b].1) + 1))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Sum: {sum}");

    Ok(())
}
