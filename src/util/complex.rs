use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn conj(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> SubAssign for Complex<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> Mul for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<T> Div for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re.clone() * rhs.re.clone() + rhs.im.clone() * rhs.im.clone();
        Self {
            re: (self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone())
                / denom.clone(),
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl<T> DivAssign for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}
