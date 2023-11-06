use super::arith_circuit::{ArithCircuit, Gate};
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Prover<'a>{
	circuit: &'a mut ArithCircuit,
	num_bits: usize,
	curr_gate: usize,
	curr_wiring: (usize, usize),
	rand_lbls: Vec<Zp>,
}

impl<'a> Prover<'a> {
	pub fn new(circ: &'a mut ArithCircuit, start_lbl: usize) -> Prover {
		Prover {
			num_bits: circ.num_bits,
			curr_gate: start_lbl,
			curr_wiring: circ.get_gate_wiring(start_lbl),
			rand_lbls: Vec::new(),
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
			let s = self.assemble_s(i, conn_gates.0, conn_gates.1);
			for k in 0..3 {
				let mut u = self.assemble_u(k);
				let mut term_p = Self::calc_termp(&s, &u);
			}
		}
		poly
	}
	// Little endian
	fn assemble_u(&self, k: u32) -> Vec<Zp> {
		let mut u = Vec::new();
		for i in 0..self.num_bits {
			u.push(Zp::new(math::get_bit(self.curr_gate, i) as u32));
		}
		u.extend(&self.rand_lbls);
		u.push(Zp::new(k));
		u
	}
	fn assemble_s(&self, g: usize, gl: usize, gr: usize) -> Vec<Zp> {
		let mut s = Vec::new();
		for i in 0..self.num_bits {
			s.push(Zp::new(math::get_bit(g, i) as u32));
		}
		for i in 0..self.num_bits {
			s.push(Zp::new(math::get_bit(gl, i) as u32));
		}
		for i in 0..self.num_bits {
			s.push(Zp::new(math::get_bit(gr, i) as u32));
		}
		s
	}
	fn calc_termp(s: &[Zp], u: &[Zp]) -> Zp {
		let mut term_p = Zp::new(1);
		for i in 0..u.len() {
			term_p *= Self::x(s[i], u[i]);
		}
		term_p
	}
	fn x(s: Zp, u: Zp) -> Zp {
		if s == 1 {
			u
		} else {
			-u + Zp::new(1)
		}
	}
}
