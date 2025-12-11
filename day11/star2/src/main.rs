use std::{collections::HashMap, error::Error, fs};

fn recursive_search(current: usize, dac: usize, fft: usize, mut vdac: bool, mut vfft: bool, (bool_map, mcount): (&Vec<bool>, usize), count_map: &mut HashMap<(usize, bool, bool), usize>) -> usize {
    if current == dac {
        vdac = true;
    } else if current == fft {
        vfft = true;
    }

    match count_map.get(&(current, vdac, vfft)) {
        Some(i) => *i,
        None => {
            let mut end = true;
            let sum = bool_map[(current * mcount)..((current + 1) * mcount)]
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    b.then(|| { end = false; recursive_search(i, dac, fft, vdac, vfft, (bool_map, mcount), count_map) })
                })
                .sum::<usize>();

            let sum = if !end {
                sum
            } else {
                if vdac && vfft {
                    1
                } else {
                    0
                }
            };

            count_map.insert((current, vdac, vfft), sum);

            sum
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;
    // let input = fs::read_to_string("../inputt")?;

    let mut machine_map = HashMap::new();

    let mut svr = None;
    let mut dac = None;
    let mut fft = None;

    let machines = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 1)
        .map(|line| {
            let (m, os) = line.split_once(':').expect("Invalid line!!!");
            let is_svr = m == "svr";
            let is_dac = m == "dac";
            let is_fft = m == "fft";
            let m = match machine_map.get(m) {
                Some(i) => *i,
                None => {
                    let i = machine_map.keys().len();
                    machine_map.insert(m, i);
                    i
                }
            };

            if is_svr {
                assert!(svr.replace(m).is_none());
            } else if is_dac {
                assert!(dac.replace(m).is_none());
            } else if is_fft {
                assert!(fft.replace(m).is_none());
            }

            let os = os
                .trim()
                .split(' ')
                .map(|o| match machine_map.get(o) {
                    Some(i) => *i,
                    None => {
                        let i = machine_map.keys().len();
                        machine_map.insert(o, i);
                        i
                    }
                })
                .collect::<Vec<_>>();

            (m, os)
        })
        .collect::<Vec<_>>();

    let svr = svr.expect("Missing 'svr'!!!");
    let dac = dac.expect("Missing 'dac'!!!");
    let fft = fft.expect("Missing 'fft'!!!");

    let mcount = machines.len() + 1;//because out is not in the vector

    let mut bool_map = vec![false; mcount * mcount];

    for (m, os) in machines {
        for o in os {
            bool_map[m * mcount + o] = true;
        }
    }

    let mut count_map = HashMap::new();

    let sum = recursive_search(svr, dac, fft, false, false, (&bool_map, mcount), &mut count_map);

    println!("Sum: {sum}");

    Ok(())
}

