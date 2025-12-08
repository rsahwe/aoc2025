use std::{error::Error, fs};

use itertools::Itertools;
use nalgebra::{Point3, distance};

fn explore_region(visited: &mut Vec<bool>, p: usize, conn: &Vec<bool>) -> usize {
    if visited[p] {
        0
    } else {
        visited[p] = true;

        1 + (0..visited.len()).filter_map(|i| conn[i * visited.len() + p].then(|| explore_region(visited, i, conn))).sum::<usize>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let points = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (x, (y, z)) = line
                .split_once(',')
                .map(|(x, yz)| (x, yz.split_once(',').unwrap()))
                .unwrap();

            Point3::new(x.parse::<f64>().expect("Wrong number!!!"), y.parse().expect("Wrong number!!!"), z.parse().expect("Wrong number!!!"))
        })
        .collect::<Vec<_>>();

    let mut connected = (0..points.len())
        .cartesian_product(0..points.len())
        .map(|(a, b)| a == b)
        .collect::<Vec<_>>();

    for _ in 0..1000 {
        let (i, _) = connected
            .iter()
            .enumerate()
            .filter(|(_, c)| !**c)
            .map(|(i, _)| {
                let x = i % points.len();
                let y = i / points.len();

                (i, distance(&points[x], &points[y]))
            })
            .sorted_by(|(_, ad), (_, bd)| f64::total_cmp(&ad, bd))
            .next()
            .expect("No connection left?");

        let x = i % points.len();
        let y = i / points.len();
        connected[x * points.len() + y] = true;
        connected[y * points.len() + x] = true;
    }

    let mut visited = vec![false; points.len()];

    let mut sizes = Vec::new();

    while let Some(point) = visited.iter().enumerate().find_map(|(i, v)| (!v).then_some(i)) {
        sizes.push(explore_region(&mut visited, point, &connected));
    }

    let sum = sizes
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product::<usize>();

    println!("Sum: {sum}");

    Ok(())
}
