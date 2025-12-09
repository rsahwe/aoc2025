use std::{error::Error, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn in_area(point: (usize, usize), lines: &[(usize, (usize, usize))]) -> bool {
    (lines
        .iter()
        .filter_map(|(x, (ya, yb))| (*ya <= point.1 && point.1 <= *yb).then_some(*x))
        .take_while(|x| *x <= point.0)
        .count() % 2) == 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;
    // let input = fs::read_to_string("../inputt")?;

    let points = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(',').expect("Invalid point!!!"))
        .map(|(x, y)| (x.parse::<usize>().expect("Invalid int!!!") * 2, y.parse::<usize>().expect("Invalid int!!!") * 2))
        .collect::<Vec<(usize, usize)>>();

    let mut ylines = (0..points.len())
        .filter_map(|i| {
            let j = (i + 1) % points.len();

            (points[j].1 != points[i].1).then_some((points[i].0, (points[i].1.min(points[j].1), points[i].1.max(points[j].1))))
        })
        .collect::<Vec<_>>();

    ylines.sort_by(|(xa, (_, _)), (xb, (_, _))| xa.cmp(xb));

    let sum = (0..(points.len() - 1))
        .into_par_iter()
        .map(|a| {
            ((a + 1)..points.len())
                .into_par_iter()
                .filter(|b| points[a].0 != points[*b].0 && points[a].1 != points[*b].1)
                .map(|b| (
                    points[a].0.min(points[b].0),
                    points[a].0.max(points[b].0),
                    points[a].1.min(points[b].1),
                    points[a].1.max(points[b].1))
                )
                .filter(|(xa, xb, ya, yb)|
                    (0..((xb - xa) / 2)).all(|x| (0..((yb - ya) / 2)).all(|y| in_area((xa + 1 + 2 * x, ya + 1 + 2 * y), &ylines)))
                )
                .map(|(xa, xb, ya, yb)| (xb - xa + 2) * (yb - ya + 2))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap() / 4;

    println!("Sum: {sum}");

    Ok(())
}
