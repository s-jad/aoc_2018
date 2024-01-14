use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let mut data = input
        .lines()
        .map(|l| {
            let parts = l.split_once(']').unwrap();
            let ts = parts.0.trim_start_matches("[");
            let mut ev = parts.1.trim();

            if ev.contains("#") {
                let ev_parts = ev.split_terminator(&['#', ' ']).collect_vec();

                ev = ev_parts
                    .into_iter()
                    .find(|s| s.parse::<usize>().is_ok())
                    .unwrap();
            }

            let nd = NaiveDate::parse_from_str(&ts[0..10], "%Y-%m-%d").unwrap();
            let nt = NaiveTime::parse_from_str(&ts[11..], "%H:%M").unwrap();
            let ndt = NaiveDateTime::new(nd, nt);

            (ndt, ev)
        })
        .collect_vec();

    data.sort_by(|a, b| a.0.cmp(&b.0));

    let mut current_guard = "";
    let mut timetable: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    let mut awake = 1;
    let mut prev_time = NaiveTime::parse_from_str("11:11", "%H:%M").unwrap();

    for (ts, ev) in data {
        if prev_time.to_string() != "11:11" {
            let mut mins: Vec<(usize, usize)> = vec![];

            for t in prev_time.minute()..ts.minute() {
                mins.push((t as usize, awake));
            }

            timetable
                .entry(current_guard)
                .and_modify(|m| m.append(&mut mins))
                .or_insert(mins);
        }

        prev_time = ts.time();

        match ev {
            _ if ev.parse::<usize>().is_ok() => {
                current_guard = ev;
            }
            "wakes up" => {
                awake = 1;
            }
            "falls asleep" => {
                awake = 0;
            }
            _ => unreachable!(),
        }
    }

    let (guard, min, _) = timetable
        .iter()
        .fold(
            None,
            |mut acc: Option<(&&str, &usize, i32)>, (guard, mins)| {
                let min_count = mins.into_iter().filter(|(_, a)| *a == 0).fold(
                    HashMap::new(),
                    |mut acc, (m, _)| {
                        acc.entry(m).and_modify(|c| *c += 1).or_insert(1);

                        acc
                    },
                );

                let val = min_count.into_iter().max_by(|(_, v1), (_, v2)| v1.cmp(&v2));

                if val.is_some() {
                    let (m, c) = val.unwrap();
                    if acc.is_some() {
                        if c > acc.unwrap().2 {
                            acc = Some((guard, m, c));
                            acc
                        } else {
                            acc
                        }
                    } else {
                        acc = Some((guard, m, c));
                        acc
                    }
                } else {
                    acc
                }
            },
        )
        .unwrap();

    guard.parse::<usize>().unwrap() * min
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
