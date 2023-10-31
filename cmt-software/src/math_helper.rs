pub const PRIME: u32 = !0 - 1;

pub fn lagrange_interpolation(sample_pts: &Vec<(i32, i32)>, tgt_pt: i32) -> i32 {
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
