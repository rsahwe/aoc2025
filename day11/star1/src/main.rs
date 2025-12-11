use std::{collections::HashMap, error::Error, fs};

fn recursive_search(current: usize, (bool_map, mcount): (&Vec<bool>, usize), count_map: &mut HashMap<usize, usize>) -> usize {
    match count_map.get(&current) {
        Some(i) => *i,
        None => {
            let sum = bool_map[(current * mcount)..((current + 1) * mcount)]
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    b.then(|| recursive_search(i, (bool_map, mcount), count_map))
                })
                .sum::<usize>();

            let sum = if sum != 0 {
                sum
            } else {
                1
            };

            count_map.insert(current, sum);

            sum
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../input1")?;

    let mut machine_map = HashMap::new();

    let mut you = None;

    let machines = input
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 1)
        .map(|line| {
            let (m, os) = line.split_once(':').expect("Invalid line!!!");
            let is_you = m == "you";
            let m = match machine_map.get(m) {
                Some(i) => *i,
                None => {
                    let i = machine_map.keys().len();
                    machine_map.insert(m, i);
                    i
                }
            };

            if is_you {
                assert!(you.replace(m).is_none());
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

    let you = you.expect("Missing 'you'!!!");

    let mcount = machines.len() + 1;//because out is not in the vector

    let mut bool_map = vec![false; mcount * mcount];

    for (m, os) in machines {
        for o in os {
            bool_map[m * mcount + o] = true;
        }
    }

    let mut count_map = HashMap::new();

    let sum = recursive_search(you, (&bool_map, mcount), &mut count_map);

    println!("Sum: {sum}");

    Ok(())
}
