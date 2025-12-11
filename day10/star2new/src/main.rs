use std::{error::Error, fs, sync::Mutex, usize};

use::rayon::prelude::*;
use::selen::prelude::*;

fn activate_generator((_, buttons, joltage): (Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let mut buttons = buttons.iter().map(|btn| btn.iter().fold(vec![0; joltage.len()], |mut acc, n| { acc[*n] = 1; acc })).collect::<Vec<_>>();

    buttons.sort_by(|a, b| b.iter().sum::<usize>().cmp(&a.iter().sum::<usize>()));

    let mut m = Model::with_config(SolverConfig::unlimited());

    let buttons = buttons.into_iter().map(|v| v.into_iter().map(|e| m.int(e as i32, e as i32)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_counts = m.ints(buttons.len(), 0, *joltage.iter().max().unwrap() as i32);
    let button_count_sum = m.sum(&button_counts);
    let joltage = joltage.into_iter().map(|j| m.int(j as i32, j as i32)).collect::<Vec<_>>();
    let button_products = buttons.into_iter().zip(button_counts.iter()).map(|(b, c)| b.into_iter().map(|v| m.mul(*c, v)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_results = (0..joltage.len()).map(|i| button_products.iter().map(|b| b[i]).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_joltage = button_results.into_iter().map(|br| m.sum(&br)).collect::<Vec<_>>();

    for (bj, j) in button_joltage.into_iter().zip(joltage.into_iter()) {
        m.eq_op(bj, j);
    }

    m.minimize(button_count_sum).expect("Pls work...").as_int(button_count_sum).expect("Pls work...") as usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let count = input.lines().map(str::trim).filter(|line| line.len() > 0).count();
    let progress_and_index = Mutex::new((0, 0));

    let sum = input
        .par_lines()
        // .lines()
        .map(str::trim)
        .filter(|line| line.len() > 0)
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|els| {
            match els.as_slice() {
                [first, middle @ .., last] => (first.to_owned(), middle.to_owned(), last.to_owned()),
                _ => unreachable!("Input error!!!"),
            }
        })
        .map(|(first, middle, last)| {
            (
                first[1..(first.len() - 1)].chars().map(|l| l == '#').collect::<Vec<_>>(),
                middle.iter().map(|btn| btn[1..(btn.len() - 1)]
                    .split(',')
                    .map(|el| el.parse::<usize>().expect("Int error!!!"))
                    .collect::<Vec<_>>()
                ).collect::<Vec<_>>(),
                last[1..(last.len() - 1)].split(',').map(|el| el.parse::<usize>().expect("Int error!!!")).collect::<Vec<_>>()
            )
        })
        .map(activate_generator)
        .inspect(|_| {
            let mut progress_and_index = progress_and_index.lock().unwrap();

            *progress_and_index = (progress_and_index.0, progress_and_index.1 + 1);
            if progress_and_index.1 * 100 / count > progress_and_index.0 {
            *progress_and_index = (progress_and_index.1 * 100 / count, progress_and_index.1);
                println!("{:02}%", progress_and_index.0);
            }
        })
        .sum::<usize>();

    println!();
    println!("Sum: {sum}");

    Ok(())
}
