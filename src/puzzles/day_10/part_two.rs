use super::{parser::parse, Result};

pub(crate) fn solve(input: &str) -> Result<u64> {
    let map = parse(input)?;
    Ok(map.enclosed_tile_count())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn part_two_simple() {
        const INPUT: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(Ok(4), solve(INPUT));
    }

    #[test]
    fn part_two_larger() {
        const INPUT: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(Ok(8), solve(INPUT));
    }
}
