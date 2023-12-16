use std::ffi::OsStr;
use std::path::Path;
use std::{env, io};

use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum PuzzleError {
    #[error("load input file '{path}' failure: {source}")]
    LoadInputFailure {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid input at {line}: {reason}")]
    InvalidInput { line: usize, reason: String },
    #[error("not implemented")]
    NotImplemented,
    #[error("{message}")]
    Unexpected { message: String },
    #[error("{message}: {source}")]
    UnexpectedErr {
        message: String,
        #[source]
        source: anyhow::Error,
    },
}

impl PuzzleError {
    fn from_io_error<S: AsRef<OsStr> + ?Sized>(path: &S, source: io::Error) -> Self {
        let path = path.as_ref().to_string_lossy().to_string();
        PuzzleError::LoadInputFailure { path, source }
    }

    fn invalid_input(line: usize, reason: &str) -> Self {
        let reason = reason.to_owned();
        PuzzleError::InvalidInput { line, reason }
    }

    fn unexpected(message: &str) -> Self {
        let message = message.to_owned();
        PuzzleError::Unexpected { message }
    }

    #[allow(dead_code)]
    fn unexpected_err(message: &str, err: anyhow::Error) -> Self {
        let message = message.to_owned();
        PuzzleError::UnexpectedErr {
            message,
            source: err,
        }
    }
}

pub type Result<T> = core::result::Result<T, PuzzleError>;
type Fn = fn() -> Result<()>;

#[allow(clippy::type_complexity)]
const PUZZLES: [(&str, Fn, Fn); 25] = [
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
    part_one: &'static Fn,
    part_two: &'static Fn,
}

impl Puzzle {
    fn new(name: &'static str, part_one: &'static Fn, part_two: &'static Fn) -> Self {
        Puzzle {
            name,
            part_one,
            part_two,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run_part_one(&self) -> Result<()> {
        (*self.part_one)()
    }

    pub fn run_part_two(&self) -> Result<()> {
        (*self.part_two)()
    }
}

fn read_input_file(day: i32, part: Option<i32>) -> Result<String> {
    let data_file_name = if let Some(part) = part {
        format!("day_{day:02}.part_{part:02}.input")
    } else {
        format!("day_{day:02}.input")
    };
    let exe_path =
        env::current_exe().map_err(|err| PuzzleError::from_io_error(&data_file_name, err))?;
    let exe_dir_path = exe_path
        .parent()
        .ok_or_else(|| PuzzleError::unexpected("failed to get executable parent path"))?;
    let path = Path::new(exe_dir_path).join(data_file_name);
    match std::fs::read_to_string(&path) {
        Ok(string) => Ok(string),
        Err(err) => Err(PuzzleError::from_io_error(&path, err)),
    }
}
