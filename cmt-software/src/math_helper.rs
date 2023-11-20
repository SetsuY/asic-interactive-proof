use std::{fmt, ops, cmp};
use rand;

pub const PRIME: u32 = !0 - 1;
pub const LOW_BIT_MASK: usize = !0 - 1;

pub fn interpolate(sample_pts: &[(Zp, Zp)], tgt_pt: Zp) -> Zp {
	let mut sum = Zp::new(0);
	for j in 0..sample_pts.len() {
		sum += sample_pts[j].1 * lagrange_poly(sample_pts, tgt_pt, j, sample_pts.len());
	}
	sum
}

fn lagrange_poly(sample_pts: &[(Zp, Zp)], x: Zp, j: usize, k: usize) -> Zp {
	let mut result = Zp::new(1);
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

#[derive(Debug, Copy, Clone)]
pub struct Zp {
	val: i64,
}

impl Zp {
	pub fn new(n: u32) -> Zp {
		Zp {
			val: (n as i64) % (PRIME as i64),
		}
	}
	pub fn new_rand() -> Zp {
		Zp {
			val: rand::random(),
		}
	}
	pub fn val(&self) -> u32 {
		self.val as u32
	}
}
impl fmt::Display for Zp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.val)
	}
}
impl cmp::PartialEq<Zp> for Zp {
	fn eq(&self, other: &Zp) -> bool {
		self.val == other.val
	}
}
impl cmp::PartialEq<u32> for Zp {
	fn eq(&self, other: &u32) -> bool {
		self.val == *other as i64
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
		let result: u128 = (self.val as u128 * other.val as u128) % (PRIME as u128);
		Zp {
			val: result as i64,
		}
	}
}
impl ops::MulAssign for Zp {
	fn mul_assign(&mut self, other: Self) {
		*self = *self * other;
	}
}
impl ops::Div for Zp {
	type Output = Self;
	fn div(self, other: Self) -> Self {
		Zp {
			val: self.val / other.val,
		}
	}
}
impl ops::Neg for Zp {
	type Output = Self;
	fn neg(self) -> Self {
		Zp {
			val: PRIME as i64 - self.val,
		}
	}
}
