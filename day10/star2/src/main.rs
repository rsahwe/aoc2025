use std::{error::Error, fs, sync::Mutex, usize};

use::rayon::prelude::*;

fn check_recursive(target: &Vec<usize>, state: &mut Vec<usize>, buttons: &[Vec<usize>], min_index: usize, depth: usize, mut max_depth: usize) -> usize {
    if depth >= max_depth {
        return usize::MAX;
    }

    if state == target {
        depth
    } else {
        'outer: for (i, btn) in buttons.iter().enumerate().skip(min_index) {
            for i in 0..target.len() {
                state[i] += btn[i];
                if state[i] > target[i] {
                    for j in 0..=i {
                        state[j] -= btn[j];
                    }

                    continue 'outer;
                }
            }

            let val = check_recursive(target, state, buttons, i, depth + 1, max_depth);
            if val < max_depth {
                max_depth = val;
            }
            
            for i in 0..target.len() {
                state[i] -= btn[i];
            }
        }

        max_depth
    }
}

fn activate_generator((_, buttons, joltage): (Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let mut buttons = buttons.iter().map(|btn| btn.iter().fold(vec![0; joltage.len()], |mut acc, n| { acc[*n] = 1; acc })).collect::<Vec<_>>();

    buttons.sort_by(|a, b| b.iter().sum::<usize>().cmp(&a.iter().sum::<usize>()));

    check_recursive(&joltage, &mut vec![0; joltage.len()], &buttons, 0, 0, usize::MAX)
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
