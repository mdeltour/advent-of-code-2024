use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead + Debug>(reader: R) -> Result<i32> {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = reader
            .lines()
            .flatten()
            .map(|line| {
                let mut parts = line.split_whitespace();

                (
                    parts.next().unwrap().parse::<i32>().unwrap(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        left.sort_unstable();
        right.sort_unstable();

        let answer = zip(left, right)
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>();

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
