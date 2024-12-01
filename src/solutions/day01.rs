// src/solutions/dayXX.rs
use crate::utils::input;
use anyhow::Result;
use itertools::DedupBy;

pub fn solve() -> Result<()> {
    let input = input::read_day_input(1)?;

    // Solve part 1
    let part1_result = solve_part1(&input);
    println!("Part 1: {}", part1_result);

    // Solve part 2
    let part2_result = solve_part2(&input);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    // Parse input into two vectors
    let (left, right): (Vec<i64>, Vec<i64>) = input
        .lines()
        .filter(|line| !line.is_empty()) // Skip empty lines
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();
            (numbers[0], numbers[1])
        })
        .unzip();

    (left, right)
}

fn solve_part1(input: &str) -> i64 {
    let (left, right) = parse_input(input);

    // Sort both vectors
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();

    // Calculate sum of differences
    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    // Parse input into two vectors
    let (left, right) = parse_input(input);

    // Calculate similarity score
    let mut total: i64 = 0;
    for &num in &left {
        // Count how many times this number appears in right list
        let count = right.iter().filter(|&&r| r == num).count() as i64;
        // Add to total (num * how many times it appears)
        total += num * count;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 31);
    }
}
