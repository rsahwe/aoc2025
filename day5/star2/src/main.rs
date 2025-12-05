use std::{error::Error, fs, ops::Range};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    Ok(println!(
        "Sum: {}",
        input
            .lines()
            .map(str::trim)
            .take_while(|l| l.len() != 0)
            .map(|l| l.split_once('-').expect("Invalid range!!!"))
            .map(|l| l.0.parse().expect("Invalid range!!!")..(l.1.parse::<usize>().expect("Invalid range!!!") + 1))
            .fold(Vec::<Range<_>>::new(), |mut vec, mut n| {
                while let Some((i, r)) = vec.iter().enumerate().find(|(_, r)| {
                    (n.start <= r.start && n.end >= r.start)
                        || (n.start <= r.end && n.end >= r.end)
                        || (r.start <= n.start && r.end >= n.start)
                }) {
                    n = n.start.min(r.start)..n.end.max(r.end);
                    vec.remove(i);
                }

                vec.push(n);

                vec
            })
            .into_iter()
            .fold(0, |c, r| c + r.count())
    ))
}
