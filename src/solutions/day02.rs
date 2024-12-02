// src/solutions/day02.rs
use crate::utils::input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = input::read_day_input(2)?; // Replace XX with actual day number

    // Solve part 1
    let part1_result = solve_part1(&input);
    println!("Part 1: {}", part1_result);

    // Solve part 2
    let part2_result = solve_part2(&input);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(levels: &[i64]) -> bool {
    // TODO: Implement the safety check logic here
    // 1. Check if numbers are strictly increasing or decreasing
    // 2. Check if adjacent numbers differ by 1-3

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..levels.len() - 1 {
        let current = levels[i];
        let next = levels[i + 1];
        let diff = (next - current).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        // Check strictly increasing or decreasing
        if current >= next {
            is_increasing = false;
        }
        if current <= next {
            is_decreasing = false;
        }
    }

    // The sequence must be either strictly increasing or strictly decreasing
    is_increasing || is_decreasing
}

fn solve_part1(input: &str) -> i64 {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as i64
}

fn is_safe_with_dampener(levels: &[i64]) -> bool {
    // If already safe, no need to remove anything
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each level once and check if it becomes safe
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_safe_report(&modified) {
            return true;
        }
    }

    false
}

fn solve_part2(input: &str) -> i64 {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 4);
    }
}
