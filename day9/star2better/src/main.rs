use std::{convert::identity, error::Error, fs};

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

    println!("pre |xlines|: {}, pre |ylines|: {}", xlines.len(), ylines.len());

    loop { // Merge parallel lines to eliminate extra checks
        let mut done_something = false;

        while let Some((i, (x, (ya, yb)))) = ylines.iter().enumerate().find(|(_, (_, (ya, yb)))| yb - ya == 1) {
            let ((ai, (ay, (axa, axb))), (bi, (by, (bxa, bxb)))) = (
                xlines.iter().enumerate().find(|(_, (y, (xa, xb)))| y == ya && (xa == x || xb == x)).expect("Bad code :("),
                xlines.iter().enumerate().find(|(_, (y, (xa, xb)))| y == yb && (xa == x || xb == x)).expect("Bad code :("),
            );

            if axa == bxb || bxa == axb {
                continue;
            }

            done_something = true;
            
            if axa < x {
                if axa < bxa {
                    let (si, (sx, (sya, syb))) = ylines.iter().enumerate().find(|(_, (x, (ya, _)))| x == bxa && ya == by).expect("Bad code :(");

                    xlines[ai] = (*ay, (*axa, *bxa));
                    xlines.remove(bi);
                    ylines[si] = (*sx, (*sya, *syb - 1));
                    ylines.remove(i);
                } else {
                    let (si, (sx, (sya, syb))) = ylines.iter().enumerate().find(|(_, (x, (_, yb)))| x == axa && yb == ay).expect("Bad code :(");

                    xlines[bi] = (*by, (*bxa, *axa));
                    xlines.remove(ai);
                    ylines[si] = (*sx, (*sya, *syb + 1));
                    ylines.remove(i);
                }
            } else {
                if axb > bxb {
                    let (si, (sx, (sya, syb))) = ylines.iter().enumerate().find(|(_, (x, (ya, _)))| x == bxb && ya == by).expect("Bad code :(");

                    xlines[ai] = (*ay, (*bxb, *axb));
                    xlines.remove(bi);
                    ylines[si] = (*sx, (*sya, *syb - 1));
                    ylines.remove(i);
                } else {
                    let (si, (sx, (sya, syb))) = ylines.iter().enumerate().find(|(_, (x, (_, yb)))| x == axb && yb == ay).expect("Bad code :(");

                    xlines[bi] = (*by, (*axb, *bxb));
                    xlines.remove(ai);
                    ylines[si] = (*sx, (*sya, *syb + 1));
                    ylines.remove(i);
                }
            }
        }

        while let Some((i, (y, (xa, xb)))) = xlines.iter().enumerate().find(|(_, (_, (xa, xb)))| xb - xa == 1) {
            let ((ai, (ax, (aya, ayb))), (bi, (bx, (bya, byb)))) = (
                ylines.iter().enumerate().find(|(_, (x, (ya, yb)))| x == xa && (ya == y || yb == y)).expect("Bad code :("),
                ylines.iter().enumerate().find(|(_, (x, (ya, yb)))| x == xb && (ya == y || yb == y)).expect("Bad code :("),
            );

            if aya == byb || bya == ayb {
                continue;
            }

            done_something = true;
            
            if aya < y {
                if aya < bya {
                    let (si, (sy, (sxa, sxb))) = xlines.iter().enumerate().find(|(_, (y, (xa, _)))| y == bya && xa == bx).expect("Bad code :(");

                    ylines[ai] = (*ax, (*aya, *bya));
                    ylines.remove(bi);
                    xlines[si] = (*sy, (*sxa, *sxb - 1));
                    xlines.remove(i);
                } else {
                    let (si, (sy, (sxa, sxb))) = xlines.iter().enumerate().find(|(_, (y, (_, xb)))| y == aya && xb == ax).expect("Bad code :(");

                    ylines[bi] = (*bx, (*bya, *aya));
                    ylines.remove(ai);
                    xlines[si] = (*sy, (*sxa, *sxb + 1));
                    xlines.remove(i);
                }
            } else {
                if ayb > byb {
                    let (si, (sy, (sxa, sxb))) = xlines.iter().enumerate().find(|(_, (y, (xa, _)))| y == byb && xa == bx).expect("Bad code :(");

                    ylines[ai] = (*ax, (*byb, *ayb));
                    ylines.remove(bi);
                    xlines[si] = (*sy, (*sxa, *sxb - 1));
                    xlines.remove(i);
                } else {
                    let (si, (sy, (sxa, sxb))) = xlines.iter().enumerate().find(|(_, (y, (_, xb)))| y == ayb && xb == ax).expect("Bad code :(");

                    ylines[bi] = (*bx, (*ayb, *byb));
                    ylines.remove(ai);
                    xlines[si] = (*sy, (*sxa, *sxb + 1));
                    xlines.remove(i);
                }
            }
        }

        if !done_something {
            break
        }
    }

    println!("post |xlines|: {}, post |ylines|: {}", xlines.len(), ylines.len());

    let sum = (0..(points.len() - 1)) // All possible first corners
        .into_par_iter() // rayon ftw
        .map(|a| {
            ((a + 1)..points.len()) // All possible second corners
                .filter(|b| points[a].0 != points[*b].0 && points[a].1 != points[*b].1) // Deduplicate points wtf??? (no performance hit anyway)
                .map(|b| (
                    points[a].0.min(points[b].0),
                    points[a].0.max(points[b].0),
                    points[a].1.min(points[b].1),
                    points[a].1.max(points[b].1)
                )) // Get "normalized" corners
                .filter(|(xa, xb, ya, yb)| {
                    // for dy in 0..((yb - ya) / 2) { // Treat each line separately for optimization
                    //     let y = ya + 2 * dy + 1; // Dual grid y

                    //     let mut position = *xa; // Saved position

                    //     if !ylines // in_area only changes output if the point crosses a line so do that
                    //         .iter()
                    //         .filter(|(_, (yla, ylb))| *yla < y && y < *ylb) // Only relevant lines
                    //         .skip_while(|(xl, (_, _))| xl <= xa)
                    //         .map_while(|(xl, (_, _))| {
                    //             if position + 1 >= *xb {
                    //                 None
                    //             } else {
                    //                 let val = in_area((position + 1, y), &ylines, &xlines, &points);
                    //                 position = if val { *xl } else { *xb };
                    //                 Some(val)
                    //             }
                    //         })
                    //         .all(identity)
                    //     {
                    //         if position + 1 >= *xb || !in_area((position + 1, y), &ylines, &xlines, &points) {
                    //             return false
                    //         };
                    //     }
                    // }

                    // true
                    (0..((xb - xa) / 2)).all(|dx| (0..((yb - ya) / 2)).all(|dy| in_area((xa + 1 + 2 * dx, ya + 1 + 2 * dy), &ylines, &xlines, &points)))
                })
                .map(|(xa, xb, ya, yb)| (xb - xa + 2 * 1) * (yb - ya + 2 * 1)) // Due to original grid being weird
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap() / 4; // Back from dual grid

    println!("Sum: {sum}");

    Ok(())
}
