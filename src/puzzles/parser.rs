use std::str::Chars;

pub(crate) struct Numbers<'a> {
    chars: Chars<'a>,
}

impl<'a> Iterator for Numbers<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut number = None;
        for c in self.chars.by_ref() {
            if c.is_ascii_digit() {
                number = Some(number.unwrap_or_default() * 10 + (c as u8 - b'0') as usize);
            } else if let Some(number) = number.take() {
                return Some(number);
            }
        }
        if let Some(number) = number.take() {
            return Some(number);
        }
        None
    }
}

pub(crate) trait HasNumbers<'a> {
    fn numbers(&self) -> Numbers<'a>;
}

impl<'a> HasNumbers<'a> for &'a str {
    fn numbers(&self) -> Numbers<'a> {
        Numbers {
            chars: self.chars(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HasNumbers;

    #[test]
    fn numbers_test() {
        assert_eq!(vec![123], "123".numbers().collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], "1 2 3".numbers().collect::<Vec<_>>());
        assert_eq!(
            vec![629551616, 310303897, 265998072, 58091853],
            "629551616 310303897 265998072 58091853"
                .numbers()
                .collect::<Vec<_>>()
        );
    }
}
