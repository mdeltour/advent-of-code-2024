use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_safe(levels: &[i32]) -> bool {
        if levels.len() < 2 {
            return false;
        }

        levels
            .windows(2)
            .map(|w| w[1] - w[0])
            .try_fold(0i32, |prev, diff| -> Result<i32, Error> {
                if !(1..=3).contains(&diff) && !(-3..=-1).contains(&diff) {
                    anyhow::bail!("Difference out of range");
                }

                match prev {
                    0 => Ok(diff.signum()),
                    _ if prev.signum() != diff.signum() => anyhow::bail!("Inconsistent direction"),
                    _ => Ok(prev),
                }
            })
            .is_ok()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let safe_reports = reader
            .lines()
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .filter(|levels| is_safe(&levels))
            .count();

        Ok(safe_reports)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn is_safe_dampener(levels: &[i32]) -> bool {
        if is_safe(levels) {
            return true;
        }

        (0..levels.len()).any(|i| {
            is_safe(
                &levels
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &v)| if i == j { None } else { Some(v) })
                    .collect::<Vec<_>>(),
            )
        })
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let safe_reports = reader
            .lines()
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .filter(|levels| is_safe_dampener(&levels))
            .count();

        Ok(safe_reports)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
