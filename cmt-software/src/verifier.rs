use rand::prelude::*;
use super::arith_circuit;
use super::prover;
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Verifier<'a> {
	prov: prover::Prover<'a>,
	inputs: Vec<arith_circuit::Gate>,
	num_bits: usize,
	num_layers: usize,
	curr_gate: usize,
	curr_result: Zp,
}

impl<'a> Verifier<'a> {
	pub fn new(circ: &'a mut arith_circuit::ArithCircuit) -> Verifier {
		let gate = rand::random::<usize>() % circ.num_gate_at_layer();
		Verifier {
			num_bits: circ.num_bits,
			num_layers: circ.num_layers(),
			inputs: circ.get_inputs(),
			curr_gate: gate,
			curr_result: circ.get_gate_val(gate),
			prov: prover::Prover::new(circ, gate),
		}
	}
	pub fn verify(&mut self) -> bool {
		for i in 1..(self.num_layers + 1) {
			if !self.sum_check() {
				return false;
			}
			let tau: bool = 
		}
		true
	}
	fn sum_check(&mut self) -> bool {
		let mut result = self.curr_result;
		for i in 0..(2 * self.num_bits) {
			// TODO: resolve the case where r exceeds # of gates at layer
			let r = Zp::new(rand::random::<u32>() % 2);
			let poly: [Zp; 3] = self.prov.sum_check(i, r);
			if result != poly[0] + poly[1] {
				return false;
			}
			if r == 0 {
				result = poly[0];
			} else if r == 1 {
				result = poly[1];
			} else {
				result = math::interpolate(
					&[(Zp::new(0), poly[0]),
					  (Zp::new(1), poly[1]),
					  (Zp::new(2), poly[2])], r);
			}
		}
		let mut a: Zp;
		match self.prov.query_rand_gate() {
			Ok(val) => {
				if self.prov.get_gate_is_add() {
					a = val.0 + val.1;
				} else {
					a = val.0 * val.1;
				}
			},
			Err(_) => a = Zp::new(0),
		}
		a == result
	}
}
