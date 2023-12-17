use crate::{read_input_file, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(1, None)?;
    let sum = part_one::solve(input.trim())?;
    println!("Day one part one: sum of calibration values: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(1, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day one part two: sum of calibration values: {sum}");
    Ok(())
}

mod part_one {
    use crate::{PuzzleError, Result};

    pub fn solve(input: &str) -> Result<u32> {
        fn u32_from_char(c: char) -> Option<u32> {
            if c.is_ascii_digit() {
                return Some(c as u32 - '0' as u32);
            }
            None
        }
        let calibration_values = input.lines().enumerate().map(|(line, input)| {
            let values = input
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
                    return Err(PuzzleError::invalid_input(
                        line,
                        "line does not have any digits",
                    ))
                }
            }
        }

        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_one() {
            let input = r#"
1abc2
pqr3stu8vwx 
a1b2c3d4e5f
treb7uchet"#;

            assert_eq!(Ok(142), solve(input.trim()));
        }
    }
}

mod part_two {
    use crate::{PuzzleError, Result};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_DIGITS: Regex =
            Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine|[0-9])")
                .expect("failed digits");
        static ref RE_REVERSE_DIGITS: Regex =
            Regex::new("(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])")
                .expect("failed reversed digits");
    }

    pub fn solve(input: &str) -> Result<u32> {
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
        let calibration_values = input.lines().enumerate().map(|(line, input)| {
            let first = RE_DIGITS
                .captures(input)
                .and_then(|capture| capture.get(1).map(|sub_capture| sub_capture.as_str()));
            let input_reverse: String = input.chars().rev().collect();
            let last = RE_REVERSE_DIGITS
                .captures(&input_reverse)
                .and_then(|capture| {
                    capture
                        .get(1)
                        .map(|sub_capture| sub_capture.as_str().chars().rev().collect::<String>())
                });
            let values: Option<(u32, u32)> = match (first, last) {
                (Some(first), Some(last)) => match (u32_from_str(first), u32_from_str(&last)) {
                    (Some(first), Some(last)) => Some((first, last)),
                    _ => None,
                },
                _ => None,
            };
            (line + 1, values)
        });

        let mut sum = 0;
        for (line, values) in calibration_values {
            match values {
                Some((first, last)) => sum += (first * 10) + last,
                None => {
                    return Err(PuzzleError::invalid_input(
                        line,
                        "line does not have any digits",
                    ));
                }
            }
        }

        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_two() {
            let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

            assert_eq!(Ok(281), solve(input.trim()));
        }
    }
}
