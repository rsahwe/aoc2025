use std::{error::Error, fs};

fn do_range(mut first: usize, second: usize, end: Option<usize>, diglen: usize) -> usize {
    let mut sum = 0;

    if second > first {
        first += 1;
    }

    let max = (0..diglen).fold(1, |acc, _| acc * 10);

    loop {
        if first >= max {
            return sum;
        }

        let val = first * max + first;

        if end.is_some_and(|end| val > end) {
            return sum;
        }

        sum += val;
        first += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut sum = 0;

    for range in input.split(',') {
        let (start, end) = range.split_once('-').ok_or("Invalid range!!!")?;
        let (start, end) = (start.trim(), end.trim());

        if start.len() == end.len() {
            if (start.len() % 2) == 0 {
                let (first, second) = start.split_at(start.len() / 2);
                let (first, second) = (first.trim(), second.trim());
                sum += do_range(first.parse::<usize>()?, second.parse::<usize>()?, Some(end.parse::<usize>()?), start.len() / 2);
            }

            continue;
        }

        let (mut start, end) = (start.chars().collect::<Vec<_>>(), end.chars().collect::<Vec<_>>());
        
        while start.len() <= end.len() {
            if (start.len() % 2) == 0 {
                let (first, second) = start.split_at(start.len() / 2);
                let first = first.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize));
                let second = second.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize));
                let end = (start.len() == end.len()).then(|| end.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize)));
                sum += do_range(first, second, end, start.len() / 2);
            }

            start.fill('0');
            start.insert(0, '1');
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
