use std::time::{Duration, Instant};
use log::{info, trace};
use super::arith_circuit::{ArithCircuit};
use super::prover;
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Verifier<'a> {
	prov: prover::Prover<'a>,
	circuit: &'a ArithCircuit,
	num_bits: usize,
	num_layers: usize,
	curr_gate: Vec<Zp>,
	curr_result: Zp,
	rand_lbls: Vec<Zp>,
	sumcheck_runtime: Duration,
}

impl<'a> Verifier<'a> {
	pub fn new(circ: &'a ArithCircuit) -> Verifier {
		let mut start_lbl = Vec::new();
		for _i in 0..circ.num_bits {
			start_lbl.push(Zp::new_rand());
		}
		trace!("Start gate {:?}", start_lbl);
		Verifier {
			num_bits: circ.num_bits,
			num_layers: circ.num_layers(),
			curr_result: circ.mle_gate_val(&start_lbl),
			prov: prover::Prover::new(circ, start_lbl.clone()),
			curr_gate: start_lbl,
			circuit: circ,
			rand_lbls: Vec::new(),
			sumcheck_runtime: Duration::ZERO,
		}
	}
	pub fn verify(&mut self) -> bool {
		for i in 1..(self.num_layers + 1) {
			if !self.sum_check() {
				return false;
			}
			info!("Layer {} Done\n", i);

			let (h0, h1) = self.prov.get_rand_val();
			let mut all_gate_vals = Vec::new();
			all_gate_vals.push((Zp::new(0), h0));
			all_gate_vals.push((Zp::new(1), h1));
			all_gate_vals.extend(self.prov.get_all_vals());
			trace!("All hi {:?}", all_gate_vals);
			let rand_next = Zp::new_rand();
			self.curr_gate = math::interpolate_next_gates(&self.rand_lbls, rand_next, self.num_bits);
			self.curr_result = math::interpolate(&all_gate_vals, rand_next);
			if i != self.num_layers {
				self.prov.next_layer(rand_next);
			}
			self.rand_lbls.clear();
			trace!("Update on rand {:?}, gate {:?}, value {}", rand_next, self.curr_gate, self.curr_result);
		}
		println!("Prover Sumcheck Time: {}ns", self.prov.sumcheck_time.as_nanos());
		println!("Verifier Sumcheck Time: {}ns", self.sumcheck_runtime.as_nanos() - 
			self.prov.sumcheck_time.as_nanos());
		self.curr_result == self.circuit.mle_gate_val(&self.curr_gate)
	}
	fn sum_check(&mut self) -> bool {
		let now = Instant::now();
		let mut result = self.curr_result;
		for i in 0..(2 * self.num_bits) {
			let r = Zp::new_rand();
			let poly: [Zp; 3] = self.prov.sum_check(i, r);
			if result != poly[0] + poly[1] {
				info!("Reject on poly {:?}, expecting {}", poly, result);
				return false;
			}
			trace!("Got poly {:?}, interpolate on {}", poly, r);
			result = math::interpolate(
				&[(Zp::new(0), poly[0]),
				  (Zp::new(1), poly[1]),
				  (Zp::new(2), poly[2])], r);
			self.rand_lbls.push(r);
			trace!("Round {} pass, result {}", i, result);
		}
		let a: Zp = self.sum_calc_next_result();
		trace!("Matching rand gate {:?} = {}", self.rand_lbls, a);
		self.sumcheck_runtime += now.elapsed();
		a == result
	}
	fn sum_calc_next_result(&self) -> Zp {
		let (lbl_l, lbl_r) = self.rand_lbls.split_at(self.num_bits);
		let (h0, h1) = self.prov.get_rand_val();
		self.circuit.mle_wiring(&self.curr_gate, lbl_l, lbl_r, true) * (h0 + h1) +
		self.circuit.mle_wiring(&self.curr_gate, lbl_l, lbl_r, false) * h0 * h1
	}
}
