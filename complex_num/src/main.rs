// In this code sample we will implement complex number.
// Complex number is of the form a + bi
// where a is the real part and bi is the imaginary part
// features to be implemented
// - instantiate/create complex numbers
// - display value of complex numbers
// - basic arithmetic operations on complex numbers
// using +, -, *, / operators

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Sub};

// Define Complex struct with two fields
// one for real part and other for the imaginary part
#[derive(Debug, PartialEq)]
struct Complex {
    real: i32,
    imag: i32,
}

// implement new function
// so that new Complex number can be created
impl Complex {
    fn new(real: i32, imag: i32) -> Self {
        Self { real, imag }
    }

    // conjugate of any complex number is the number
    // where the sign of the imaginary part is reversed
    // 7 + 4i will have conjugate 7 - 4i
    fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: self.imag * -1,
        }
    }

    // copy the number into a new Complex
    fn copy(&self) -> Self {
        Self {
            real: self.real,
            imag: self.imag,
        }
    }
}

// implement Display trait so that Complex number can be printed
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match (self.real, self.imag) {
            (0, 0) => write!(f, "0"),
            (0, _) => write!(f, "{}i", self.imag),
            (_, 0) => write!(f, "{}", self.real),
            (_, i) if i < 0 => write!(f, "{} - {}i", self.real, self.imag * -1),
            _ => write!(f, "{} + {}i", self.real, self.imag),
        }
    }
}

// implement Add trait so that addition operation can be done on Complex
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // (a+bi) + (x+yi) = (a+x)+(b+y)i
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// implement Sub trait so that subtraction operation can be done on Complex
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        // (a+bi) - (x+yi) = (a-x)+(b-y)i
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// implement Mul trait so that multiplication operation can be done on Complex
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // (a+bi)(x+yi) = ax + ayi + bxi - by = (ax-by) + (ay+ bx)i
        let real = self.real * rhs.real - self.imag * rhs.imag;
        let imag = self.real * rhs.imag + self.imag * rhs.real;
        Self { real, imag }
    }
}

// implement Div trait so that division operation can be done on Complex
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // (a+bi)/(x+yi)
        // = ((a+bi)(x-yi))/((x+yi)(x-yi))
        // = ax-ayi+bxi+by/(x^2+y^2)
        let c = rhs.conjugate();
        let numerator = self * c;
        let denominator = rhs.real * rhs.real + rhs.imag * rhs.imag;
        // Note: Since we have taken i32 type as the fields for Complex struct
        // division operation will always round off the result to whole number
        // for both real and imaginary part
        // for more accurate result real and imag fields should be represented
        // as f32 or f64 type
        Self {
            real: numerator.real / denominator,
            imag: numerator.imag / denominator,
        }
    }
}

fn main() {
    println!("Complex number arithmetic implementation");
    let num1 = Complex::new(5, 12);
    let num2 = Complex::new(4, 7);
    println!("num1 is: {num1}, num2 is: {num2}");
    println!("num1 + num2 = {}", num1.copy() + num2.copy());
    println!("num1 - num2 = {}", num1.copy() - num2.copy());
    println!("num1 * num2 = {}", num1.copy() * num2.copy());
    println!("num1 / num2 = {}", num1.copy() / num2.copy());
}

#[cfg(test)]
mod tests {
    use super::*;

    // test Complex number display using to_string
    // both real and imaginary parts are positive
    #[test]
    fn test_display_positive_num() {
        let num = Complex::new(2, 3);
        let s = num.to_string();
        assert_eq!("2 + 3i".to_string(), s);
    }
    // real part is negative but imaginary part is positive
    #[test]
    fn test_display_negative_real() {
        let num = Complex::new(-2, 3);
        let s = num.to_string();
        assert_eq!("-2 + 3i".to_string(), s);
    }
    // real part is positive but imaginary part is negative
    #[test]
    fn test_display_negative_imag() {
        let num = Complex::new(2, -3);
        let s = num.to_string();
        assert_eq!("2 - 3i".to_string(), s);
    }
    // both real and imaginary parts are negative
    #[test]
    fn test_display_negative_num() {
        let num = Complex::new(-2, -3);
        let s = num.to_string();
        assert_eq!("-2 - 3i".to_string(), s);
    }
    // real part is non zero and imaginary part is zero
    #[test]
    fn test_display_imag_zero() {
        let num = Complex::new(2, 0);
        let s = num.to_string();
        assert_eq!("2".to_string(), s);
    }
    // real part is zero and imaginary part is non zero
    #[test]
    fn test_display_real_zero() {
        let num = Complex::new(0, 3);
        let s = num.to_string();
        assert_eq!("3i".to_string(), s);
    }
    // real part is zero and imaginary part is non zero and negative
    #[test]
    fn test_display_real_zero_neg() {
        let num = Complex::new(0, -3);
        let s = num.to_string();
        assert_eq!("-3i".to_string(), s);
    }
    // both real part and imaginary parts are zero
    #[test]
    fn test_display_zero() {
        let num = Complex::new(0, 0);
        let s = num.to_string();
        assert_eq!("0".to_string(), s);
    }

    // add two positive numbers
    #[test]
    fn test_add_positives() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(6, 12);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = a+bi, rhs = x-yi
    #[test]
    fn test_add_pos_neg1() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(4, -7);
        let expected = Complex::new(6, -2);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = a+bi, rhs = -x+yi
    #[test]
    fn test_add_pos_neg2() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(-4, 7);
        let expected = Complex::new(-2, 12);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = a+bi, rhs = -x-yi
    #[test]
    fn test_add_pos_neg3() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(-4, -7);
        let expected = Complex::new(-2, -2);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = a-bi, rhs = x+yi
    #[test]
    fn test_add_neg_pos1() {
        let lhs = Complex::new(2, -5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(6, 2);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = -a+bi, rhs = x+yi
    #[test]
    fn test_add_neg_pos2() {
        let lhs = Complex::new(-2, 5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(2, 12);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = -a-bi, rhs = x+yi
    #[test]
    fn test_add_neg_pos3() {
        let lhs = Complex::new(-2, -5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(2, 2);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = a-bi, rhs = x-yi
    #[test]
    fn test_add_neg_neg1() {
        let lhs = Complex::new(2, -5);
        let rhs = Complex::new(4, -7);
        let expected = Complex::new(6, -12);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add two lhs = -a-bi, rhs = -x-yi
    #[test]
    fn test_add_neg_neg2() {
        let lhs = Complex::new(-2, -5);
        let rhs = Complex::new(-4, -7);
        let expected = Complex::new(-6, -12);
        let result = lhs + rhs;
        assert_eq!(expected, result);
    }

    // add conjugate
    #[test]
    fn test_add_conjugate() {
        let n = Complex::new(-2, -5);
        let expected = Complex::new(n.real * 2, 0);
        let result = n.conjugate() + n;
        assert_eq!(expected, result);
    }

    // sub same number
    #[test]
    fn test_sub_same() {
        let n = Complex::new(-2, -5);
        let n1 = Complex::new(n.real, n.imag);
        let expected = Complex::new(n.real * 2, n.imag * 2);
        let result = n1 + n;
        assert_eq!(expected, result);
    }

    // sub conjugate
    #[test]
    fn test_sub_conjugate() {
        let n = Complex::new(-2, -5);
        let c = n.conjugate();
        let expected = Complex::new(0, n.imag * 2);
        let result = n - c;
        assert_eq!(expected, result);
    }

    // sub (a+bi) - (x+yi)
    #[test]
    fn test_sub_pos1() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(-2, -2);
        let result = lhs - rhs;
        assert_eq!(expected, result);
    }

    // sub (a+bi) - (x+bi)
    #[test]
    fn test_sub_same_imag() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(4, 5);
        let expected = Complex::new(-2, 0);
        let result = lhs - rhs;
        assert_eq!(expected, result);
    }

    // sub (a+bi) - (x+bi)
    #[test]
    fn test_sub_same_real() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(2, 7);
        let expected = Complex::new(0, -2);
        let result = lhs - rhs;
        assert_eq!(expected, result);
    }

    // mul (a+bi) * (x+yi)
    #[test]
    fn test_mul_pos() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(4, 7);
        let expected = Complex::new(-27, 34);
        let result = lhs * rhs;
        assert_eq!(expected, result);
    }

    // mul (a+bi) * 0
    #[test]
    fn test_mul_zero() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(0, 0);
        let expected = Complex::new(0, 0);
        let result = lhs * rhs;
        assert_eq!(expected, result);
    }

    // multiply with conjugate
    #[test]
    fn test_mul_conjugate() {
        let n = Complex::new(2, 5);
        let expected = Complex::new(n.real * n.real + n.imag * n.imag, 0);
        let result = n.conjugate() * n;
        assert_eq!(expected, result);
    }

    // divide by same
    #[test]
    fn test_div_same() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(lhs.real, lhs.imag);
        let expected = Complex::new(1, 0);
        let result = lhs / rhs;
        assert_eq!(expected, result);
    }

    // divide by imag zero
    #[test]
    fn test_div_pos() {
        let lhs = Complex::new(20, 25);
        let rhs = Complex::new(5, 0);
        let expected = Complex::new(4, 5);
        let result = lhs / rhs;
        assert_eq!(expected, result);
    }

    // divide by real zero
    #[test]
    fn test_div_real_zero() {
        let lhs = Complex::new(20, 25);
        let rhs = Complex::new(0, 5);
        let expected = Complex::new(5, -4);
        let result = lhs / rhs;
        assert_eq!(expected, result);
    }

    // divide by zero
    #[test]
    #[should_panic]
    fn test_div_zero() {
        let lhs = Complex::new(2, 5);
        let rhs = Complex::new(0, 0);
        let _ = lhs / rhs;
    }
}
