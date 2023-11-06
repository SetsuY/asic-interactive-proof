use rand::prelude::*;
use super::arith_circuit;
use super::prover;

pub struct Verifier<'a> {
	prov: prover::Prover<'a>,
	inputs: Vec<arith_circuit::Gate>,
	num_bits: usize,
}

impl<'a> Verifier<'a> {
	pub fn new(circ: &'a mut arith_circuit::ArithCircuit) -> Verifier {
		Verifier {
			num_bits: circ.num_bits,
			prov: prover::Prover::new(circ, rand::random()),
			inputs: Vec::new(),
		}
	}
	pub fn layer_verify(&mut self, gate: usize, result: u32) -> bool {
		true
	}
	fn sum_check(&mut self, gate: usize, result: u32) -> bool {
		true
	}
}
