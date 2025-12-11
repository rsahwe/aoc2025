use std::{error::Error, fs, usize};

use::rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

fn activate_generator((_, buttons, joltage): (Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let mut buttons = buttons.iter().map(|btn| btn.iter().fold(vec![0; joltage.len()], |mut acc, n| { acc[*n] = 1; acc })).collect::<Vec<_>>();

    buttons.sort_by(|a, b| b.iter().sum::<usize>().cmp(&a.iter().sum::<usize>()));

    let m = Optimize::new();

    let buttons = buttons.into_iter().map(|v| v.into_iter().map(|e| Int::from_u64(e as u64)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_counts = (0..buttons.len()).map(|i| Int::new_const(i.to_string())).collect::<Vec<_>>();

    for bc in &button_counts {
        m.assert(&bc.ge(Int::from_u64(0)));
    }

    let button_count_sum = button_counts.iter().fold(Int::from_u64(0), |acc, n| acc + n);

    let joltage = joltage.into_iter().map(|j| Int::from_u64(j as u64)).collect::<Vec<_>>();
    let button_products = buttons.into_iter().zip(button_counts.iter()).map(|(b, c)| b.into_iter().map(|v| c * v).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_results = (0..joltage.len()).map(|i| button_products.iter().map(|b| b[i].clone()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let button_joltage = button_results.into_iter().map(|br| br.into_iter().fold(Int::from_u64(0), |acc, n| acc + n)).collect::<Vec<_>>();

    for (bj, j) in button_joltage.into_iter().zip(joltage.into_iter()) {
        m.assert(&bj.eq(j));
    }

    let variable = Int::new_const("result");
    m.assert(&variable.eq(button_count_sum));

    m.minimize(&variable);

    assert_eq!(m.check(&[]), SatResult::Sat);

    m.get_model().expect("Missing model!!!").get_const_interp(&variable).expect("Missing sum!!!").as_u64().expect("Not a u64?") as usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let sum = input
        .par_lines()
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
        .sum::<usize>();

    println!("Sum: {sum}");

    Ok(())
}
