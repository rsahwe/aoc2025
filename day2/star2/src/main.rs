use std::{collections::HashSet, error::Error, fs};

fn do_range(mut first: usize, start: usize, end: Option<usize>, diglen: usize, count: usize, fv: &mut HashSet<usize>) -> usize {
    let mut sum = 0;

    let max = (0..diglen).fold(1, |acc, _| acc * 10);

    let first_val = (0..count).fold(0, |acc, _| acc * max + first);

    if first_val < start {
        first += 1;
    }

    loop {
        if first >= max {
            return sum;
        }

        let val = (0..count).fold(0, |acc, _| acc * max + first);

        if end.is_some_and(|end| val > end) {
            return sum;
        }

        if !fv.contains(&val) {
            sum += val;
            fv.insert(val);
        }

        first += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut sum = 0;

    let mut found_values = HashSet::new();

    for range in input.split(',') {
        let (start, end) = range.split_once('-').ok_or("Invalid range!!!")?;
        let (start, end) = (start.trim(), end.trim());

        let old_sum = sum;

        for count in 2..=end.len() {
            if (2..count).find(|val| count % val == 0).is_some() {
                continue
            }

            if start.len() == end.len() {
                if (start.len() % count) == 0 {
                    let (first, _) = start.split_at(start.len() / count);
                    let first = first.trim();
                    sum += do_range(first.parse::<usize>()?, start.parse::<usize>()?, Some(end.parse::<usize>()?), start.len() / count, count, &mut found_values);
                }

                continue;
            }

            let (mut start, end) = (start.chars().collect::<Vec<_>>(), end.chars().collect::<Vec<_>>());
            
            while start.len() <= end.len() {
                if (start.len() % count) == 0 {
                    let (first, _) = start.split_at(start.len() / count);
                    let first = first.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize));
                    let end = (start.len() == end.len()).then(|| end.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize)));
                    sum += do_range(first, start.iter().fold(0, |acc, n| acc * 10 + (*n as usize - '0' as usize)), end, start.len() / count, count, &mut found_values);
                }

                start.fill('0');
                start.insert(0, '1');
            }
        }

        println!("Range {start}-{end} has {}", sum - old_sum);
    }

    println!("Sum: {sum}");

    Ok(())
}
