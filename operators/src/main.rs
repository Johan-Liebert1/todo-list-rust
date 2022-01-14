use core::fmt;
use std::ops;

#[derive(Clone, Copy)]
struct Imaginary {
    x: i64,
    y: i64,
}

impl ops::Add<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn add(self, rhs: Imaginary) -> Imaginary {
        Imaginary {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl fmt::Display for Imaginary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

fn main() {
    let imag = Imaginary { x: 5, y: 2 };
    let imag2 = Imaginary { x: 8, y: 10 };

    let imag3 = imag + imag2;

    println!("({}) + ({}) = {}", imag, imag2, imag3);
}
