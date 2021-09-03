use std::ops::{Add, Sub};

pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self { Complex { re, im } }
}

impl<T> Add for Complex<T>
    where
        T: Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Sub for Complex<T>
    where
        T: Sub<Output = T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;

    #[test]
    fn test_complex_creation() {
        let c = Complex::new(1, 2);

        assert_eq!(c.re, 1);
        assert_eq!(c.im, 2);
    }

    #[test]
    fn test_add_complex_numbers() {
        let c1 = Complex::new(1, 2);
        let c2 = Complex::new(-1, 3);

        let result = c1 + c2;

        assert_eq!(result.re, 0);
        assert_eq!(result.im, 5);
    }

    #[test]
    fn test_subtract_complex_numbers_of_different_types() {
        let c1 = Complex::new(1.0, 1.0);
        let c2 = Complex::new(1.0, 1.0);

        let result = c1 - c2;

        assert_eq!(result.re, 0.0);
        assert_eq!(result.im, 0.0);
    }
}