use rand;
use log::info;
use super::arith_circuit::{ArithCircuit, GateLbl};
use super::prover;
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Verifier<'a> {
	prov: prover::Prover<'a>,
	circuit: &'a ArithCircuit,
	num_bits: usize,
	num_layers: usize,
	curr_gate: GateLbl,
	curr_result: Zp,
	rand_lbls: Vec<Zp>,
}

impl<'a> Verifier<'a> {
	pub fn new(circ: &'a ArithCircuit) -> Verifier {
		let gate = GateLbl::new_rand();
		Verifier {
			num_bits: circ.num_bits,
			num_layers: circ.num_layers(),
			curr_result: circ.mle_gate_val(gate),
			curr_gate: gate,
			prov: prover::Prover::new(circ, gate),
			circuit: circ,
		}
	}
	pub fn verify(&mut self) -> bool {
		for i in 1..(self.num_layers + 1) {
			if !self.sum_check() {
				return false;
			}
			info!("Layer {} Done\n", i);

			let all_gate_vals: Vec<(Zp, Zp)> = self.prov.get_all_vals();
			let rand_next = Zp::new_rand();
			self.curr_gate = (rand_lbls.1 - rand_lbls.0) * rand_next + rand_lbls.0;
			self.curr_gate %= self.prov.num_gate_at_layer();
			self.curr_result = all_gate_vals[rand_next];
			self.prov.next_layer(rand_next);
			info!("Update on rand {}, gate {}, value {}", rand_next, self.curr_gate, self.curr_result);
		}
		let inputs = self.circuit.get_inputs();
		self.curr_result == inputs[self.curr_gate % inputs.len()].val()
	}
	fn sum_check(&mut self) -> bool {
		let mut result = self.curr_result;
		for i in 0..(2 * self.num_bits) {
			let r = Zp::new_rand();
			let poly: [Zp; 3] = self.prov.sum_check(i, r);
			if result != poly[0] + poly[1] {
				info!("Reject on poly {:?}", poly);
				return false;
			}
			result = math::interpolate(
				&[(Zp::new(0), poly[0]),
				  (Zp::new(1), poly[1]),
				  (Zp::new(2), poly[2])], r);
			self.rand_lbls.push(r);
			info!("Round {} pass, result {}", i, result);
		}
		let a: Zp = self.sum_calc_next_result();
		info!("Matching rand gate {:?} = {}", self.rand_lbls, a);
		a == result
	}
	fn sum_calc_next_result(&self) -> Zp {
		assert_eq!(self.rand_lbls.len(), self.num_bits * 2);
		let (lbl_l, lbl_r) = self.rand_lbls.split_at(self.num_bits);
		let h0 = self.circuit.mle_gate_val(lbl_l);
		let h1 = self.circuit.mle_gate_val(lbl_r);
		self.circuit.mle_wiring(self.curr_gate, lbl_l, lbl_r, true) * (h0 + h1) +
		self.circuit.mle_wiring(self.curr_gate, lbl_l, lbl_r, false) * h0 * h1
	}
}
