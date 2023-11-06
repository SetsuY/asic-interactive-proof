use std::{fmt, ops};

pub const PRIME: u32 = !0 - 1;
pub const LOW_BIT_MASK: usize = !0 - 1;

pub fn interpolate(sample_pts: &Vec<(i32, i32)>, tgt_pt: i32) -> i32 {
	let mut sum = 0;
	for j in 0..sample_pts.len() {
		sum += sample_pts[j].1 * lagrange_poly(sample_pts, tgt_pt, j, sample_pts.len());
	}
	sum
}

fn lagrange_poly(sample_pts: &Vec<(i32, i32)>, x: i32, j: usize, k: usize) -> i32 {
	let mut result = 1;
	for i in 0..k {
		if i != j.try_into().unwrap() {
			result *= (x - sample_pts[i].0) / (sample_pts[j].0 - sample_pts[i].0);
		}
	}
	result
}

pub fn bic(n: usize, mask: usize) -> usize {
	n & !mask
}

pub fn get_bit(n: usize, bit: usize) -> usize {
	bic(n >> bit, LOW_BIT_MASK)
}

#[derive(Copy, Clone)]
pub struct Zp {
	val: i64,
}

impl Zp {
	pub fn new(n: u32) -> Zp {
		Zp {
			val: n as i64,
		}
	}
}
impl fmt::Display for Zp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.val)
	}
}
impl ops::Add for Zp {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			val: (self.val + other.val) % PRIME as i64,
		}
	}
}
impl ops::AddAssign for Zp {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}
impl ops::Sub for Zp {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Zp {
			val: if self.val - other.val < 0 {
				let mut sum: i64 = self.val - other.val;
				while sum < 0 {
					sum += PRIME as i64;
				}
				sum
			} else {
				self.val - other.val
			},
		}
	}
}
impl ops::Mul for Zp {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		Zp {
			val: (self.val * other.val) % PRIME as i64,
		}
	}
}
impl ops::MulAssign for Zp {
	fn mul_assign(&mut self, other: Self) {
		*self = *self * other;
	}
}
