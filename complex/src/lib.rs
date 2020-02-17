use std::convert::From;
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex {
            re: value.0,
            im: value.1,
        }
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complex_add() {
        let a = Complex::new(0.3, 0.4);
        let b = Complex::new(0.3, 0.4);
        let c = a + b;
        assert_eq!(c.re, 0.6);
        assert_eq!(c.im, 0.8);
    }
    #[test]
    fn complex_default_value() {
        let a: Complex<f32> = Complex::default();
        assert_eq!(a.re, 0.0);
        assert_eq!(a.im, 0.0);
    }
    #[test]
    fn from_tuple() {
        let a = (0., 1.);
        let a = Complex::from(a);
        assert_eq!(a.re, 0.);
        assert_eq!(a.im, 1.);
    }
    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(2345, 456);
        println!("{}", my_imaginary);
    }
}
