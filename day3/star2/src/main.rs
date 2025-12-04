fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(println!(
        "Sum: {}",
        std::fs::read_to_string("../input1")?
            .lines()
            .map(str::trim)
            .filter(|line| line.len() != 0)
            .map(|line| (line.bytes(), line.bytes().count()))
            .map(|(chars, count)| (0..12)
                .fold((0, 0), |(skip, number), i| chars
                    .clone()
                    .enumerate()
                    .skip(skip)
                    .take(count + i + 1 - skip - 12)
                    .try_fold((0, b'0'), |(ai, ac), (ni, nc)| Some(
                        (nc > ac).then_some((ni, nc)).unwrap_or((ai, ac))
                    ))
                    .map(|(index, chr)| (index + 1, number * 10 + (chr - b'0') as usize))
                    .unwrap())
                .1)
            .sum::<usize>()
    ))
}
