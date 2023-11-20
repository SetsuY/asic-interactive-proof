use rand;
use log::info;
use super::arith_circuit::ArithCircuit;
use super::prover;
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Verifier<'a> {
	prov: prover::Prover<'a>,
	circuit: &'a ArithCircuit,
	num_bits: usize,
	num_layers: usize,
	curr_gate: usize,
	curr_result: Zp,
}

impl<'a> Verifier<'a> {
	pub fn new(circ: &'a ArithCircuit) -> Verifier {
		let gate = rand::random::<usize>() % circ.num_gate_at_layer();
		Verifier {
			num_bits: circ.num_bits,
			num_layers: circ.num_layers(),
			curr_gate: gate,
			curr_result: circ.get_gate_val(gate),
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

			let all_gate_vals: Vec<Zp> = self.prov.get_all_vals();
			let rand_lbls: (usize, usize) = self.prov.get_rand_gate();
			// We have num_bits + 1 values.
			let rand_next = rand::random::<usize>() % (self.num_bits + 1);
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
			// let r = Zp::new_rand();
			let r = Zp::new(rand::random::<u32>() % 2);
			let poly: [Zp; 3] = self.prov.sum_check(i, r);
			if result != poly[0] + poly[1] {
				info!("Reject on poly {:?}", poly);
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
			info!("Round {} pass, result {}", i, result);
		}
		let a: Zp;
		info!("Matching rand gate {:?}", self.prov.get_rand_gate());
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
