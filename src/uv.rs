use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct UV {
    e: [f64; 2],
}

impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        Self { e: [u, v] }
    }

    pub fn u(&self) -> f64 {
        self.e[0]
    }

    pub fn v(&self) -> f64 {
        self.e[1]
    }
}

impl Index<usize> for UV {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for UV {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for UV {
    type Output = Self;
    fn neg(self) -> Self::Output {
        UV::new(-self.e[0], -self.e[1])
    }
}

impl AddAssign for UV {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
    }
}

impl SubAssign for UV {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
    }
}

impl MulAssign<f64> for UV {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
    }
}

impl DivAssign<f64> for UV {
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
    }
}

impl Add for UV {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        UV::new(self.e[0] + other.e[0], self.e[1] + other.e[1])
    }
}

impl Sub for UV {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        UV::new(self.e[0] - other.e[0], self.e[1] - other.e[1])
    }
}

impl Mul<f64> for UV {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        UV::new(self.e[0] * t, self.e[1] * t)
    }
}

impl Mul<UV> for f64 {
    type Output = UV;
    fn mul(self, v: UV) -> Self::Output {
        v * self
    }
}

impl Div<f64> for UV {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        UV::new(self.e[0] / t, self.e[1] / t)
    }
}
