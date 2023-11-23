pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

#[allow(clippy::type_complexity)]
const PUZZLES: [(&str, fn(), fn()); 25] = [
    ("one", day_01::run_part_one, day_01::run_part_two),
    ("two", day_02::run_part_one, day_02::run_part_two),
    ("three", day_03::run_part_one, day_03::run_part_two),
    ("four", day_04::run_part_one, day_04::run_part_two),
    ("five", day_05::run_part_one, day_05::run_part_two),
    ("six", day_06::run_part_one, day_06::run_part_two),
    ("seven", day_07::run_part_one, day_07::run_part_two),
    ("eight", day_08::run_part_one, day_08::run_part_two),
    ("nine", day_09::run_part_one, day_09::run_part_two),
    ("ten", day_10::run_part_one, day_10::run_part_two),
    ("eleven", day_11::run_part_one, day_11::run_part_two),
    ("twelve", day_12::run_part_one, day_12::run_part_two),
    ("thirteen", day_13::run_part_one, day_13::run_part_two),
    ("fourteen", day_14::run_part_one, day_14::run_part_two),
    ("fifteen", day_15::run_part_one, day_15::run_part_two),
    ("sixteen", day_16::run_part_one, day_16::run_part_two),
    ("seventeen", day_17::run_part_one, day_17::run_part_two),
    ("eighteen", day_18::run_part_one, day_18::run_part_two),
    ("nineteen", day_19::run_part_one, day_19::run_part_two),
    ("twenty", day_20::run_part_one, day_20::run_part_two),
    ("twenty-one", day_21::run_part_one, day_21::run_part_two),
    ("twenty-two", day_22::run_part_one, day_22::run_part_two),
    ("twenty-three", day_23::run_part_one, day_23::run_part_two),
    ("twenty-four", day_24::run_part_one, day_24::run_part_two),
    ("twenty-five", day_25::run_part_one, day_25::run_part_two),
];

pub fn puzzle_names() -> Vec<&'static str> {
    PUZZLES.iter().map(|puzzle| puzzle.0).collect()
}

pub fn puzzles() -> Vec<Puzzle> {
    PUZZLES
        .iter()
        .map(|puzzle| Puzzle::new(puzzle.0, &puzzle.1, &puzzle.2))
        .collect()
}

pub fn get_puzzle(name: &str) -> Option<Puzzle> {
    PUZZLES.iter().find_map(|puzzle| match puzzle.0 == name {
        true => Some(Puzzle::new(puzzle.0, &puzzle.1, &puzzle.2)),
        false => None,
    })
}

pub struct Puzzle {
    name: &'static str,
    part_one: &'static fn(),
    part_two: &'static fn(),
}

impl Puzzle {
    fn new(name: &'static str, part_one: &'static fn(), part_two: &'static fn()) -> Self {
        Puzzle {
            name,
            part_one,
            part_two,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run_part_one(&self) {
        (*self.part_one)()
    }

    pub fn run_part_two(&self) {
        (*self.part_two)()
    }
}
