use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[0].abs() < s) && (self.e[0].abs() < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            // in the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl Neg for Vec3 {
    // -vec3
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    // vec3[i]
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Add for Vec3 {
    // vec3 + vec3
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl AddAssign for Vec3 {
    // vec3 += vec3
    fn add_assign(&mut self, v: Vec3) {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
    }
}

impl Sub for Vec3 {
    // vec3 - vec3
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

impl SubAssign for Vec3 {
    // vec3 -= vec3
    fn sub_assign(&mut self, v: Vec3) {
        self.e[0] -= v.e[0];
        self.e[1] -= v.e[1];
        self.e[2] -= v.e[2];
    }
}

impl Mul<Vec3> for Vec3 {
    // vec3 * vec3
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}

impl Mul<f64> for Vec3 {
    // vec3 * f64
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(t * self.e[0], t * self.e[1], t * self.e[2])
    }
}

impl Mul<Vec3> for f64 {
    // f64 * vec3
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl Mul<i32> for Vec3 {
    // vec3 * i32
    type Output = Vec3;
    fn mul(self, t: i32) -> Vec3 {
        self * (t as f64)
    }
}

impl Mul<Vec3> for i32 {
    // i32 * vec3
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        (self as f64) * v
    }
}

impl MulAssign<f64> for Vec3 {
    // vec3 *= f64
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Div<f64> for Vec3 {
    // vec3 / f64
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

impl Div<i32> for Vec3 {
    // vec3 / i32
    type Output = Vec3;
    fn div(self, t: i32) -> Vec3 {
        self / (t as f64)
    }
}

impl DivAssign<f64> for Vec3 {
    // vec3 /= f64
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub type Point3 = Vec3;
