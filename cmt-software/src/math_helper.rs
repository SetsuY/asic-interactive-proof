use std::{fmt, ops, cmp};
use rand;

// PRIME = M31
const PRIME: u32 = 2147483647; 
const PRIME_EXP: u32 = 3;
const MOD_BASE: u64 = 2; //change these arguments later
const LOW_BIT_MASK: usize = !0 - 1;

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

pub fn mle_interpolate(w: &[Zp], x: &[Zp]) -> Zp {
	assert_eq!(w.len(), x.len());
	let mut result = Zp::new(1);
	for i in 0..w.len() {
		result *= x[i] * w[i] + (Zp::new(1) - x[i]) * (Zp::new(1) - w[i]);
	}
	result
}

pub fn into_bit_arr(n: usize, num_bits: usize) -> Vec<Zp> {
	let mut result: Vec<Zp> = Vec::new();
	for i in 0..num_bits {
		result.push(Zp::new(get_bit(n, i) as u32));
	}
	result
}

pub fn interpolate_next_gates(rand_lbls: &[Zp], rand_next: Zp, num_bits: usize) -> Vec<Zp> {
	assert_eq!(rand_lbls.len(), num_bits * 2);
	let mut next_lbl: Vec<Zp> = Vec::new();
	let (lbl_l, lbl_r) = rand_lbls.split_at(num_bits);
	for i in 0..lbl_l.len() {
		next_lbl.push((lbl_r[i] - lbl_l[i]) * rand_next + lbl_l[i]);
	}
	next_lbl
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
		Zp::new(rand::random())
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
		let l = self.val as u32;
		let r = other.val as u32;
		let result = ((l as u128) * (r as u128) % (PRIME as u128)) as u32;
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
		// First use Extended Eucidean to find other^-1
		let mul_inv = Zp::new(ext_eucidean(other.val()));
		self * mul_inv
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
// Returns x in ax + PRIME*y=1
fn ext_eucidean(a: u32) -> u32 {
	let mut r_last = a as i64;
	let mut r_this = PRIME as i64;
	let mut r_next;
	let mut q: i64;

	let mut s_last: i64 = 1;
	let mut s_this: i64 = 0;
	let mut s_next;
	loop {
		q = r_last / r_this;
		r_next = r_last % r_this;
		s_next = s_last - s_this * q;
		if r_next == 0 {
			return if s_this >= 0 {s_this as u32} else {(PRIME as i64 + s_this) as u32};
		}

		r_last = r_this;
		r_this = r_next;
		s_last = s_this;
		s_this = s_next;
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zpe(u64);
impl Zpe {
	pub fn new(n: u64) -> Zpe {
		Zpe(n % MOD_BASE)
	}
	pub fn new_rand() -> Zpe {
		Zpe::new(rand::random::<u64>() % PRIME as u64)
	}
}
impl fmt::Display for Zpe {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl ops::Add for Zpe {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self::new(self.0 + other.0)
	}
}
impl ops::AddAssign for Zpe {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}
impl ops::Sub for Zpe {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		let lval = self.0 as i128;
		let rval = other.0 as i128;
		if lval - rval < 0 {
			Self::new((lval - rval + MOD_BASE as i128) as u64)
		} else {
			Self::new((lval - rval) as u64)
		}
	}
}
impl ops::Mul for Zpe {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		let result = (self.0 as u128 * other.0 as u128) % MOD_BASE as u128;
		Zpe(result as u64)
	}
}
impl ops::MulAssign for Zpe {
	fn mul_assign(&mut self, other: Self) {
		*self = *self * other;
	}
}
impl ops::Neg for Zpe {
	type Output = Self;
	fn neg(self) -> Self {
		Zpe(MOD_BASE - self.0)
	}
}
