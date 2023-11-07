use rand::prelude::*;
use super::arith_circuit;
use super::prover;
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
			
		}
		true
	}
	fn sum_check(&mut self, gate: usize, result: u32) -> bool {
		true
	}
}
