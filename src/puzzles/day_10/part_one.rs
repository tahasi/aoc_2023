use super::{parser::parse, Result};

pub(crate) fn solve(input: &str) -> Result<u64> {
    let map = parse(input)?;
    Ok(map.steps_to_furthest_point())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn part_one() {
        const INPUT: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(Ok(8), solve(INPUT));
    }
}
