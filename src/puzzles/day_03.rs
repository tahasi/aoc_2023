use super::{read_input_file, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(3, None)?;
    let sum = part_one::solve(input.trim())?;
    println!("Day three part one: sum of part numbers: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(3, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day three part two: sum of gear ratios: {sum}");
    Ok(())
}

#[derive(Debug)]
struct EngineSchematic {
    part_numbers: Vec<u32>,
    gear_ratios: Vec<u32>,
}

impl EngineSchematic {
    fn part_numbers(&self) -> &[u32] {
        &self.part_numbers
    }

    fn gear_ratios(&self) -> &[u32] {
        &self.gear_ratios
    }
}

mod part_one {
    use super::{parser, Result};

    pub fn solve(input: &str) -> Result<u32> {
        let schematic = parser::parse(input)?;
        let sum = schematic.part_numbers().iter().sum();
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_one() {
            let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
            if let Ok(sum) = solve(input.trim()) {
                assert_eq!(4361, sum);
            } else {
                assert!(false);
            }
        }
    }
}

mod part_two {
    use super::{parser, Result};

    pub fn solve(input: &str) -> Result<u32> {
        let schematic = parser::parse(input)?;
        let sum = schematic.gear_ratios().iter().sum();
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_one() {
            let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
            if let Ok(sum) = solve(input.trim()) {
                assert_eq!(467835, sum);
            } else {
                assert!(false);
            }
        }
    }
}

mod parser {
    use super::{EngineSchematic, Result};

    pub fn parse(input: &str) -> Result<EngineSchematic> {
        let state = input
            .chars()
            .fold(State::initial(), |state, c| state.handle(c));
        let (numbers, symbols) = state.complete();
        let part_numbers: Vec<Number> = numbers
            .into_iter()
            .filter(|number| number.adjacent_to_any(&symbols))
            .collect();
        let gear_ratios = symbols
            .iter()
            .filter(|symbol| symbol.is_gear())
            .map(|symbol| {
                part_numbers
                    .iter()
                    .filter(|part_number| part_number.adjacent_to(symbol))
                    .collect::<Vec<&Number>>()
            })
            .filter(|part_numbers| part_numbers.len() == 2)
            .map(|part_numbers| {
                part_numbers
                    .iter()
                    .map(|part_number| part_number.number())
                    .product()
            })
            .collect();
        Ok(EngineSchematic {
            part_numbers: part_numbers
                .into_iter()
                .map(|part_number| part_number.number())
                .collect(),
            gear_ratios,
        })
    }

    enum State {
        Searching {
            numbers: Vec<Number>,
            symbols: Vec<Symbol>,
            row: usize,
            column: Option<usize>,
        },
        InNumber {
            numbers: Vec<Number>,
            symbols: Vec<Symbol>,
            row: usize,
            column: usize,
            builder: NumberBuilder,
        },
    }

    impl State {
        fn handle(self, c: char) -> State {
            if c == '\r' || c == '.' {
                return match self {
                    State::Searching {
                        numbers,
                        symbols,
                        row,
                        column,
                    } => State::searching(
                        numbers,
                        symbols,
                        row,
                        Some(column.map_or(0, |column| column + 1)),
                    ),
                    Self::InNumber {
                        mut numbers,
                        symbols,
                        row,
                        column,
                        builder,
                    } => {
                        numbers.push(builder.build());
                        State::searching(numbers, symbols, row, Some(column + 1))
                    }
                };
            }

            if c == '\n' {
                return match self {
                    State::Searching {
                        numbers,
                        symbols,
                        row,
                        column: _,
                    } => State::searching(numbers, symbols, row + 1, None),
                    Self::InNumber {
                        mut numbers,
                        symbols,
                        row,
                        column: _,
                        builder,
                    } => {
                        numbers.push(builder.build());
                        State::searching(numbers, symbols, row + 1, None)
                    }
                };
            }

            if c.is_ascii_digit() {
                return match self {
                    State::Searching {
                        numbers,
                        symbols,
                        row,
                        column,
                    } => {
                        let column = column.map_or(0, |column| column + 1);
                        State::in_number(
                            numbers,
                            symbols,
                            row,
                            column,
                            NumberBuilder::new(row, column, c),
                        )
                    }
                    State::InNumber {
                        numbers,
                        symbols,
                        row,
                        column,
                        mut builder,
                    } => {
                        builder.add_digit(c);
                        State::in_number(numbers, symbols, row, column + 1, builder)
                    }
                };
            }

            match self {
                State::Searching {
                    numbers,
                    mut symbols,
                    row,
                    column,
                } => {
                    let column = column.map_or(0, |column| column + 1);
                    symbols.push(Symbol::new(c, row, column));
                    State::searching(numbers, symbols, row, Some(column))
                }
                State::InNumber {
                    mut numbers,
                    mut symbols,
                    row,
                    column,
                    builder,
                } => {
                    let column = column + 1;
                    numbers.push(builder.build());
                    symbols.push(Symbol::new(c, row, column));
                    State::searching(numbers, symbols, row, Some(column))
                }
            }
        }

        fn initial() -> State {
            State::Searching {
                numbers: vec![],
                symbols: vec![],
                row: 0,
                column: None,
            }
        }

        fn searching(
            numbers: Vec<Number>,
            symbols: Vec<Symbol>,
            row: usize,
            column: Option<usize>,
        ) -> State {
            State::Searching {
                numbers,
                symbols,
                row,
                column,
            }
        }

        fn in_number(
            numbers: Vec<Number>,
            symbols: Vec<Symbol>,
            row: usize,
            column: usize,
            builder: NumberBuilder,
        ) -> State {
            State::InNumber {
                numbers,
                symbols,
                row,
                column,
                builder,
            }
        }

        fn complete(self) -> (Vec<Number>, Vec<Symbol>) {
            match self {
                State::InNumber {
                    mut numbers,
                    symbols,
                    row: _,
                    column: _,
                    builder,
                } => {
                    numbers.push(builder.build());
                    (numbers, symbols)
                }
                State::Searching {
                    numbers,
                    symbols,
                    row: _,
                    column: _,
                } => (numbers, symbols),
            }
        }
    }

    fn to_u32(c: char) -> u32 {
        c as u32 - '0' as u32
    }

    #[derive(Debug)]
    struct Symbol {
        symbol: char,
        row: usize,
        column: usize,
    }

    impl Symbol {
        fn new(symbol: char, row: usize, column: usize) -> Self {
            Symbol {
                symbol,
                row,
                column,
            }
        }

        fn is_gear(&self) -> bool {
            self.symbol == '*'
        }
    }

    #[derive(Debug)]
    struct Number {
        number: u32,
        row: usize,
        column: usize,
        digit_count: usize,
    }

    impl Number {
        fn number(&self) -> u32 {
            self.number
        }

        fn adjacent_to_any(&self, symbols: &[Symbol]) -> bool {
            let start_row = if self.row > 0 { self.row - 1 } else { 0 };
            let end_row = self.row + 1;
            let start_column = if self.column > 0 { self.column - 1 } else { 0 };
            let end_column = self.column + self.digit_count;
            symbols.iter().any(|symbol| {
                symbol.row >= start_row
                    && symbol.row <= end_row
                    && symbol.column >= start_column
                    && symbol.column <= end_column
            })
        }

        fn adjacent_to(&self, symbol: &Symbol) -> bool {
            let start_row = if self.row > 0 { self.row - 1 } else { 0 };
            let end_row = self.row + 1;
            let start_column = if self.column > 0 { self.column - 1 } else { 0 };
            let end_column = self.column + self.digit_count;
            symbol.row >= start_row
                && symbol.row <= end_row
                && symbol.column >= start_column
                && symbol.column <= end_column
        }
    }

    struct NumberBuilder {
        number: u32,
        row: usize,
        column: usize,
        digit_count: usize,
    }

    impl NumberBuilder {
        fn new(row: usize, column: usize, c: char) -> Self {
            NumberBuilder {
                number: to_u32(c),
                row,
                column,
                digit_count: 1,
            }
        }

        fn add_digit(&mut self, c: char) {
            self.number = (self.number * 10) + to_u32(c);
            self.digit_count += 1;
        }

        fn build(self) -> Number {
            Number {
                number: self.number,
                row: self.row,
                column: self.column,
                digit_count: self.digit_count,
            }
        }
    }
}
