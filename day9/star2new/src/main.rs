use std::{convert::identity, error::Error, fs, sync::{Mutex, atomic::AtomicUsize}};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn in_area(point: (usize, usize), ylines: &[(usize, (usize, usize))], xlines: &[(usize, (usize, usize))], points: &[(usize, usize)]) -> bool {
    (ylines
        .iter()
        .filter_map(|(x, (ya, yb))| (*ya <= point.1 && point.1 <= *yb).then_some(*x))
        .take_while(|x| *x <= point.0)
        .count() % 2) == 1 // Either crosses an odd number of boundaries -> is inside
    ||
    (ylines
        .windows(2)
        .any(|sl| {
            let [(xa, (yla, yha)), (xb, (ylb, yhb))] = sl else { unreachable!() };

            *yla < point.1 && point.1 < *yha && *ylb < point.1 && point.1 < *yhb && *xa + 1 == point.0 && point.0 + 1 == *xb
        })) // Or is inbetween two ylines (not possible in original situation) -> considered inside
    ||
    (xlines
        .windows(2)
        .any(|sl| {
            let [(ya, (xla, xha)), (yb, (xlb, xhb))] = sl else { unreachable!() };

            *xla < point.0 && point.0 < *xha && *xlb < point.0 && point.0 < *xhb && *ya + 1 == point.1 && point.1 + 1 == *yb
        })) // Or is inbetween two xlines (not possible in original situation) -> considered inside
    ||
    (
            points.iter().any(|(x, y)| x + 1 == point.0 && y + 1 == point.1)
        &&  points.iter().any(|(x, y)| x + 1 == point.0 && y - 1 == point.1)
        &&  points.iter().any(|(x, y)| x - 1 == point.0 && y + 1 == point.1)
        &&  points.iter().any(|(x, y)| x - 1 == point.0 && y - 1 == point.1)
    ) // Or is surrounded by points (not possible in original situation) -> considered inside
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;
    // let input = fs::read_to_string("../inputt")?;

    let points = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(',').expect("Invalid point!!!"))
        .map(|(x, y)| (x.parse::<usize>().expect("Invalid int!!!") * 2, y.parse::<usize>().expect("Invalid int!!!") * 2))// Multiply by 2 to create a dual grid
        .collect::<Vec<(usize, usize)>>();

    let mut ylines = (0..points.len())
        .filter_map(|i| {
            let j = (i + 1) % points.len();

            (points[j].1 != points[i].1).then_some((points[i].0, (points[i].1.min(points[j].1), points[i].1.max(points[j].1))))
        })
        .collect::<Vec<_>>(); // These are all of the vertical lines

    ylines.sort_by(|(xa, (_, _)), (xb, (_, _))| xa.cmp(xb)); // Sort according to x position

    let mut xlines = (0..points.len())
        .filter_map(|i| {
            let j = (i + 1) % points.len();

            (points[j].0 != points[i].0).then_some((points[i].1, (points[i].0.min(points[j].0), points[i].0.max(points[j].0))))
        })
        .collect::<Vec<_>>(); // These are all of the horizontal lines

    xlines.sort_by(|(ya, (_, _)), (yb, (_, _))| ya.cmp(yb)); // Sort according to y position

    let progress_and_index = Mutex::new((0, 0));
    let count = points.len() - 1;

    let sum = (0..(points.len() - 1)) // All possible first corners
        .into_par_iter() // rayon ftw
        .map(|a| {
            {
                let mut progress_and_index = progress_and_index.lock().unwrap();

                progress_and_index.1 += 1;

                if progress_and_index.1 * 100 / count > progress_and_index.0 {
                    println!("{:02}%", progress_and_index.0);
                    progress_and_index.0 = progress_and_index.1 * 100 / count;
                }
            }

            ((a + 1)..points.len()) // All possible second corners
                .filter(|b| points[a].0 != points[*b].0 && points[a].1 != points[*b].1) // Take only corners
                .map(|b| (
                    points[a].0.min(points[b].0),
                    points[a].0.max(points[b].0),
                    points[a].1.min(points[b].1),
                    points[a].1.max(points[b].1)
                )) // Get "normalized" corners
                .filter(|(xa, xb, ya, yb)| {
                    for dy in 0..((yb - ya) / 2) { // Treat each line separately for optimization
                        let y = ya + 2 * dy + 1; // Dual grid y

                        if xlines.iter().find(|(ly, (lxa, lxb))| (y - 1 == *ly || y + 1 == *ly) && lxa.min(xa) <= lxb.max(xb)).is_some() { // If a xline is parallel, check all points :(
                            if !(0..((xb - xa) / 2)).all(|dx| in_area((xa + 1 + 2 * dx, y), &ylines, &xlines, &points)) {
                                return false
                            }
                        } else {
                            let mut position = *xa; // Saved position

                            if !ylines // in_area only changes output if the point crosses a line so do that
                                .iter()
                                .filter(|(_, (yla, ylb))| *yla < y && y < *ylb) // Only relevant lines
                                .skip_while(|(xl, (_, _))| xl <= xa)
                                .map_while(|(xl, (_, _))| {
                                    if position + 1 >= *xb {
                                        None
                                    } else {
                                        let val = in_area((position + 1, y), &ylines, &xlines, &points);
                                        position = if val { *xl } else { *xb };
                                        Some(val)
                                    }
                                })
                                .all(identity)
                            {
                                if position + 1 >= *xb || !in_area((position + 1, y), &ylines, &xlines, &points) {
                                    return false
                                };
                            }
                        }
                    }

                    true
                })
                .map(|(xa, xb, ya, yb)| (xb - xa + 2 * 1) * (yb - ya + 2 * 1)) // Due to original grid being weird
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap() / 4; // Back from dual grid

    println!("{:02}%", 100);
    println!();

    println!("Sum: {sum}");

    Ok(())
}
