use super::arith_circuit::{ArithCircuit, Gate};
use super::math_helper as math;

pub struct Prover<'a>{
	circuit: &'a mut ArithCircuit,
	num_bits: usize,
	curr_gate: usize,
	curr_wiring: (usize, usize),
	rand_lbls: usize,
}

impl<'a> Prover<'a> {
	pub fn new(circ: &'a mut ArithCircuit, start_lbl: usize) -> Prover {
		Prover {
			num_bits: circ.num_bits,
			curr_gate: start_lbl,
			curr_wiring: circ.get_gate_wiring(start_lbl),
			rand_lbls: 0,
			circuit: circ,
		}
	}
	pub fn next_layer(&mut self, next_gate: bool) {
		self.circuit.next_layer();
		self.curr_gate = if next_gate {self.curr_wiring.1} else {self.curr_wiring.0};
		self.curr_wiring = self.circuit.get_gate_wiring(self.curr_gate);
	}
	pub fn get_curr_wiring(&self) -> (usize, usize) {
		self.curr_wiring
	}
	pub fn sum_check(&self, round: usize, r: u32) -> [u32; 3] { 
		let mut poly: [u32; 3] = [0, 0, 0];
		for (i, gate) in self.circuit.get_last_layer().into_iter().enumerate() {
			let conn_gates = gate.get_wiring();
			let b = self.num_bits;
			for k in 0..3 {
				
				
			}
		}
		poly
	}
	fn calc_termp(&self, u: usize, b: usize, j: usize) -> u32 {
		0
	}
	fn x(&self, s: u32, u: u32) -> u32 {
		if s == 1 {
			u
		} else {
			1 - u
		}
	}
}
