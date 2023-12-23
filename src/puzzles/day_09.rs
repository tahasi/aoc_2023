use std::cmp::Ordering;

use num::Signed;

use crate::{read_input_file, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(9, None)?;
    let sum = part_one::solve(input.trim())?;
    println!("Day nine part one: sum of next forecasted values: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(9, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day nine part one: sum of prior forecasted values: {sum}");
    Ok(())
}

#[derive(PartialEq, Debug)]
struct OASISReport {
    report: Vec<Vec<i64>>,
}

impl OASISReport {
    fn new(report: Vec<Vec<i64>>) -> OASISReport {
        OASISReport { report }
    }

    fn forecast_next(&self) -> Vec<i64> {
        self.report
            .iter()
            .map(|history| OASISReport::forecast_next_values(history))
            .collect()
    }

    fn forecast_prior(&self) -> Vec<i64> {
        self.report
            .iter()
            .map(|history| OASISReport::forecast_prior_values(history))
            .collect()
    }

    fn forecast_next_values(values: &[i64]) -> i64 {
        let differences = OASISReport::differences(values);
        let last_value = values[values.len() - 1];
        if differences.iter().any(|difference| *difference != 0) {
            last_value + OASISReport::forecast_next_values(&differences)
        } else {
            last_value
        }
    }

    fn forecast_prior_values(values: &[i64]) -> i64 {
        let differences = OASISReport::differences(values);
        let first_value = values[0];
        if differences.iter().any(|difference| *difference != 0) {
            first_value - OASISReport::forecast_prior_values(&differences)
        } else {
            first_value
        }
    }

    fn differences(values: &[i64]) -> Vec<i64> {
        values
            .iter()
            .fold((vec![], None::<i64>), |(mut differences, prior), value| {
                if let Some(prior) = prior {
                    let difference =
                        match (prior.is_negative(), value.is_negative(), prior.cmp(value)) {
                            (_, _, Ordering::Equal) => 0,
                            (true, true, Ordering::Less) => prior.abs() + value,
                            (true, true, Ordering::Greater) => value + prior.abs(),
                            (true, false, Ordering::Less) => value + prior.abs(),
                            (false, true, Ordering::Greater) => value - prior,
                            (false, false, Ordering::Less) => value - prior,
                            (false, false, Ordering::Greater) => value - prior,
                            _ => panic!("impossible pattern"),
                        };
                    assert!(prior + difference == *value);
                    differences.push(difference)
                }
                (differences, Some(*value))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::OASISReport;

    #[test]
    fn forecast_next_18() {
        let report = OASISReport::new(vec![vec![0, 3, 6, 9, 12, 15]]);
        assert_eq!(vec![18], report.forecast_next());
    }

    #[test]
    fn forecast_next_28() {
        let report = OASISReport::new(vec![vec![1, 3, 6, 10, 15, 21]]);
        assert_eq!(vec![28], report.forecast_next());
    }

    #[test]
    fn forecast_next_68() {
        let report = OASISReport::new(vec![vec![10, 13, 16, 21, 30, 45]]);
        assert_eq!(vec![68], report.forecast_next());
    }

    #[test]
    fn forecast_next_negative_39() {
        let report = OASISReport::new(vec![vec![-6, -9, -13, -18, -24, -31]]);
        assert_eq!(vec![-39], report.forecast_next());
    }

    #[test]
    fn forecast_next_429() {
        let report = OASISReport::new(vec![vec![5, -1, 24, 79, 168, 290]]);
        assert_eq!(vec![429], report.forecast_next());
    }

    #[test]
    fn forecast_next_negative_3() {
        let report = OASISReport::new(vec![vec![-39, -33, -27, -21, -15, -9]]);
        assert_eq!(vec![-3], report.forecast_next());
    }

    #[test]
    fn forecast_prior_negative_3() {
        let report = OASISReport::new(vec![vec![0, 3, 6, 9, 12, 15]]);
        assert_eq!(vec![-3], report.forecast_prior());
    }

    #[test]
    fn forecast_prior_0() {
        let report = OASISReport::new(vec![vec![1, 3, 6, 10, 15, 21]]);
        assert_eq!(vec![0], report.forecast_prior());
    }

    #[test]
    fn forecast_prior_5() {
        let report = OASISReport::new(vec![vec![10, 13, 16, 21, 30, 45]]);
        assert_eq!(vec![5], report.forecast_prior());
    }
}

mod part_one {
    use super::{parser::parse, Result};

    pub fn solve(input: &str) -> Result<i64> {
        let report = parse(input)?;
        Ok(report.forecast_next().iter().copied().sum())
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        const INPUT: &str = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        #[test]
        fn part_one() {
            assert_eq!(Ok(114), solve(INPUT));
        }
    }
}

mod part_two {
    use super::{parser::parse, Result};

    pub fn solve(input: &str) -> Result<i64> {
        let report = parse(input)?;
        Ok(report.forecast_prior().iter().copied().sum())
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        const INPUT: &str = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        #[test]
        fn part_one() {
            assert_eq!(Ok(2), solve(INPUT));
        }
    }
}

mod parser {
    use crate::parser::HasNumbers;

    use super::{OASISReport, Result};

    pub fn parse(input: &str) -> Result<OASISReport> {
        let report = input
            .lines()
            .filter_map(|input| {
                let input = input.trim();
                if input.is_empty() {
                    None
                } else {
                    Some(input)
                }
            })
            .map(|input| input.signed_numbers().collect::<Vec<i64>>())
            .collect::<Vec<Vec<_>>>();
        Ok(OASISReport::new(report))
    }

    #[cfg(test)]
    mod tests {
        use super::{parse, OASISReport};

        const INPUT: &str = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        #[test]
        fn parse_input() {
            let expected_report = OASISReport::new(vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ]);
            assert_eq!(Ok(expected_report), parse(INPUT));
        }
    }
}
