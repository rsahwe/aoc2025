use std::{error::Error, fs, sync::Mutex};

use::rayon::prelude::*;

fn check_board(_: &Vec<Vec<Vec<Vec<bool>>>>, (width, height, counts): (usize, usize, Vec<usize>)) -> bool {
    let count_sum = counts.iter().sum::<usize>();

    // trolled ig
    if (width - (width % 3)) * (height - (height % 3)) < count_sum * 9 {
        false
    } else {
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut lines = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 2)
        .peekable();

    let pieces = (0..)
        .map_while(|_| lines.next_if(|i| i.bytes().nth(0).unwrap() == b'.' || i.bytes().nth(0).unwrap() == b'#'))
        .collect::<Vec<_>>();

    assert_eq!(pieces.len() % 3, 0);

    let pieces = pieces
        .chunks_exact(3)
        .map(|chunk| {
            let base = chunk
                .iter()
                .map(|line| line
                    .as_bytes()
                    .iter()
                    .map(|c| if *c == b'#' { true } else { false })
                    .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();

            let mut res = (0..8)
                .map(|i| (i % 2, (i % 4) / 2, i / 4))
                .map(|(xrev, yrev, transp)|
                    (
                        if xrev == 1 { [0, 1, 2] } else { [2, 1, 0] },
                        if yrev == 1 { [0, 1, 2] } else { [2, 1, 0] },
                        transp
                    )
                )
                .map(|(xs, ys, transp)|
                    if transp == 1 {
                        xs.iter().map(|x| ys.iter().map(|y| base[*y][*x]).collect::<Vec<_>>()).collect::<Vec<_>>()
                    } else {
                        ys.iter().map(|y| xs.iter().map(|x| base[*y][*x]).collect::<Vec<_>>()).collect::<Vec<_>>()
                    }
                )
                .collect::<Vec<_>>();

            res.sort();
            res.dedup();

            res
        })
        .collect::<Vec<_>>();
    
    // for piece in pieces {
    //     println!("{{");
    //     println!("    ---");
    //     for variant in piece {
    //         for y in variant {
    //             print!("    ");
    //             for x in y {
    //                 print!("{}", if x { '#' } else { '.' });
    //             }
    //             println!()
    //         }
    //         println!("    ---");
    //     }
    //     println!("}}");
    // }

    let progress = Mutex::new(0);

    let sum = lines
        .par_bridge()
        .map(|line| {
            let (size, counts) = line.split_once(':').expect("Invalid line!!!");
            let (width, height) = size.split_once('x').expect("Invalid size!!!");
            let (width, height) = (width.parse::<usize>().expect("Invalid width!!!"), height.parse::<usize>().expect("Invalid height!!!"));
            let counts = counts
                .trim()
                .split(' ')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>().expect("Invalid count!!!");

            (width, height, counts)
        })
        .filter_map(|(width, height, counts)| {
            let val = check_board(&pieces, (width, height, counts)).then_some(());

            let mut progress = progress.lock().unwrap();

            *progress += 1usize;

            println!("{:02}.{}%", *progress / 10, *progress % 10);

            val
        })
        .count();

    println!();
    println!("Sum: {sum}");

    Ok(())
}
