use std::{
    marker::PhantomData,
    ops::{Add, Mul},
    str::Chars,
};

pub(crate) struct SignedNumbers<'a, T> {
    chars: Chars<'a>,
    _marker: PhantomData<T>,
}

pub(crate) struct UnsignedNumbers<'a, T> {
    chars: Chars<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> Iterator for SignedNumbers<'a, T>
where
    T: Default + Mul<T, Output = T> + Add<T, Output = T> + From<i32> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ten_of_type = T::from(10_i32);
        let mut sign_of_type = T::from(1_i32);
        let mut prior_c = None;
        let mut number = None::<T>;
        for c in self.chars.by_ref() {
            if c.is_ascii_digit() {
                if let Some(prior_c) = prior_c {
                    if prior_c == '-' {
                        sign_of_type = T::from(-1_i32)
                    }
                }
                number = Some(
                    (number.unwrap_or_default() * ten_of_type) + T::from((c as u8 - b'0') as i32),
                );
            } else if let Some(number) = number.take() {
                return Some(number * sign_of_type);
            }
            prior_c = Some(c);
        }
        if let Some(number) = number.take() {
            return Some(number * sign_of_type);
        }
        None
    }
}

impl<'a, T> Iterator for UnsignedNumbers<'a, T>
where
    T: Default + Mul<T, Output = T> + Add<T, Output = T> + From<u32> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ten_of_type = T::from(10_u32);
        let mut number = None;
        for c in self.chars.by_ref() {
            if c.is_ascii_digit() {
                number = Some(
                    (number.unwrap_or_default() * ten_of_type) + T::from((c as u8 - b'0') as u32),
                );
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

pub(crate) trait HasNumbers<'a, T> {
    fn signed_numbers(&self) -> SignedNumbers<'a, T>;
    fn unsigned_numbers(&self) -> UnsignedNumbers<'a, T>;
}

impl<'a, T> HasNumbers<'a, T> for &'a str {
    fn signed_numbers(&self) -> SignedNumbers<'a, T> {
        SignedNumbers {
            chars: self.chars(),
            _marker: PhantomData,
        }
    }

    fn unsigned_numbers(&self) -> UnsignedNumbers<'a, T> {
        UnsignedNumbers {
            chars: self.chars(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HasNumbers;

    #[test]
    fn signed_numbers_test() {
        assert_eq!(vec![-123], "-123".signed_numbers().collect::<Vec<_>>());
        assert_eq!(
            vec![-1, -2, 3],
            "-1 -2 3".signed_numbers().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![629551616, -310303897, 265998072, -58091853],
            "629551616 -310303897 265998072 -58091853"
                .signed_numbers()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn unsigned_numbers_test() {
        assert_eq!(vec![123], "123".unsigned_numbers().collect::<Vec<u32>>());
        assert_eq!(
            vec![1, 2, 3],
            "1 2 3".unsigned_numbers().collect::<Vec<u32>>()
        );
        assert_eq!(
            vec![629551616, 310303897, 265998072, 58091853],
            "629551616 310303897 265998072 58091853"
                .unsigned_numbers()
                .collect::<Vec<u32>>()
        );
    }
}
