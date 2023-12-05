use regex::Regex;

use super::{read_data_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let data = read_data_file(1, 1)?;
    let sum = solve_part_one(data.trim())?;
    println!("Sum of calibration values: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let data = read_data_file(1, 2)?;
    let sum = solve_part_two(data.trim())?;
    println!("Sum of calibration values: {sum}");
    Ok(())
}

fn solve_part_one(data: &str) -> Result<u32> {
    fn u32_from_char(c: char) -> Option<u32> {
        if c.is_ascii_digit() {
            return Some(c as u32 - '0' as u32);
        }
        None
    }
    let calibration_values = data.lines().enumerate().map(|(line, data)| {
        let values = data
            .chars()
            .fold(None, |values, c| match (values, u32_from_char(c)) {
                (None, Some(value)) => Some((value, value)),
                (Some((first, _)), Some(value)) => Some((first, value)),
                (Some(values), None) => Some(values),
                _ => None,
            });
        (line + 1, values)
    });
    let mut sum = 0;
    for (line, values) in calibration_values {
        match values {
            Some((first, last)) => sum += (first * 10) + last,
            None => {
                return Err(PuzzleError::InvalidData {
                    line,
                    reason: "line does not have any digits".to_owned(),
                })
            }
        }
    }

    Ok(sum)
}

fn solve_part_two(data: &str) -> Result<u32> {
    fn u32_from_str(s: &str) -> Option<u32> {
        match s {
            "0" | "zero" => Some(0),
            "1" | "one" => Some(1),
            "2" | "two" => Some(2),
            "3" | "three" => Some(3),
            "4" | "four" => Some(4),
            "5" | "five" => Some(5),
            "6" | "six" => Some(6),
            "7" | "seven" => Some(7),
            "8" | "eight" => Some(8),
            "9" | "nine" => Some(9),
            _ => None,
        }
    }
    let re =
        Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").expect("is valid");
    let re_reverse =
        Regex::new("(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").expect("is valid");
    let calibration_values = data.lines().enumerate().map(|(line, data)| {
        let first = re
            .captures(data)
            .expect("is valid")
            .get(1)
            .unwrap()
            .as_str();
        let data_reverse: String = data.chars().rev().collect();
        let last: String = re_reverse
            .captures(&data_reverse)
            .expect("is valid")
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect();
        let first = u32_from_str(first);
        let last = u32_from_str(&last);
        let values = match (first, last) {
            (Some(first), Some(last)) => Some((first, last)),
            _ => None,
        };
        (line + 1, values)
    });

    let mut sum = 0;
    for (line, values) in calibration_values {
        match values {
            Some((first, last)) => sum += (first * 10) + last,
            None => {
                return Err(PuzzleError::InvalidData {
                    line,
                    reason: "line does not have any digits".to_owned(),
                })
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn part_one() {
        let data = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        match solve_part_one(data.trim()) {
            Ok(sum) => assert_eq!(142, sum),
            _ => assert!(false),
        }
    }

    #[test]
    fn part_two() {
        let data = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        match solve_part_two(data.trim()) {
            Ok(sum) => assert_eq!(281, sum),
            _ => assert!(false),
        }
    }
}
