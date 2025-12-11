use std::{error::Error, fs, usize};

fn check_recursive(target: usize, mut state: usize, buttons: &[usize], map: &mut [Option<usize>; 2 << 10], depth: usize, mut max_depth: usize) -> usize {
    if depth >= max_depth {
        return usize::MAX;
    }
    
    if let Some(other) = map[state] {
        if other < depth {
            return usize::MAX;
        }

        map[state] = Some(depth);
    }

    if state == target {
        depth
    } else {
        for btn in buttons {
            state ^= *btn;
            let val = check_recursive(target, state, buttons, map, depth + 1, max_depth);
            if val < max_depth {
                max_depth = val;
            }
            state ^= *btn;
        }

        max_depth
    }
}

fn activate_generator((target, buttons, _): (Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {
    assert!(target.len() <= 10);

    let mut map = [None; 2 << 10];
    
    let target = target.iter().rev().fold(0, |acc, n| (acc << 1) | *n as usize);
    let buttons = buttons.iter().map(|btn| btn.iter().fold(0, |acc, n| acc | (1 << *n))).collect::<Vec<usize>>();

    let start = 0;

    check_recursive(target, start, &buttons, &mut map, 0, 10) // lol 10 works :)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let sum = input
        .lines()
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
