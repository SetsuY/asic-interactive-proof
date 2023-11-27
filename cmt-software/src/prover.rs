use log::info;
use super::arith_circuit::{ArithCircuit};
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Prover<'a>{
	circuit: &'a ArithCircuit,
	num_bits: usize,
	curr_gate: Vec<Zp>,
	rand_lbls: Vec<Zp>,
}

impl<'a> Prover<'a> {
	pub fn new(circ: &'a ArithCircuit, start_lbl: Vec<Zp>) -> Prover {
		circ.next_layer();
		Prover {
			num_bits: circ.num_bits,
			curr_gate: start_lbl,
			rand_lbls: Vec::new(),
			circuit: circ,
		}
	}
	pub fn next_layer(&mut self, next_gate: Zp) {
		self.curr_gate = math::interpolate_next_gates(&self.rand_lbls, next_gate, self.num_bits);
		self.circuit.next_layer();
		self.rand_lbls.clear();
	}
	pub fn get_all_vals(&self) -> Vec<(Zp, Zp)> {
		let mut vals: Vec<(Zp, Zp)> = Vec::new();
		let (lbl_l, lbl_r) = self.rand_lbls.split_at(self.num_bits);
		vals.push((Zp::new(0), self.circuit.mle_gate_val(lbl_l)));
		vals.push((Zp::new(1), self.circuit.mle_gate_val(lbl_r)));
		for i in 2..(self.num_bits + 1) {
			let curr_gate = math::interpolate_next_gates(&self.rand_lbls, 
				Zp::new(i as u32), self.num_bits);
			vals.push((Zp::new(i as u32), self.circuit.mle_gate_val(&curr_gate)));
		}
		vals
	}
	pub fn sum_check(&mut self, round: usize, r: Zp) -> [Zp; 3] { 
		let mut poly: [Zp; 3] = [Zp::new(0), Zp::new(0), Zp::new(0)];
		for (i, &gate) in self.circuit.get_last_layer().into_iter().enumerate() {
			let conn_gates = gate.get_wiring();
			let s = self.assemble_s(i, conn_gates.0, conn_gates.1);
			for k in 0..3 {
				let u = self.assemble_u(k);
				let term_p = Self::calc_termp(&s, &u);
				let term_l;
				let term_r;
				if round < self.num_bits {
					let l_lbl = self.assemble_gate_label(
						&self.rand_lbls, Zp::new(k), conn_gates.0);
					let r_lbl = conn_gates.1;
					term_l = self.circuit.mle_gate_val(&l_lbl);
					term_r = self.circuit.get_gate_val(r_lbl)
				} else {
					let l_lbl = &self.rand_lbls[0..self.num_bits];
					let r_lbl = self.assemble_gate_label(
						&self.rand_lbls[self.num_bits..],
						Zp::new(k), conn_gates.1);
					term_l = self.circuit.mle_gate_val(l_lbl);
					term_r = self.circuit.mle_gate_val(&r_lbl);
				}
				if gate.is_add() {
					poly[k as usize] += term_p * (term_l + term_r);
				} else {
					poly[k as usize] += term_p * term_l * term_r;
				}
				info!("Gate {}, {} {} {}", i, term_p, term_l, term_r);
			}
		}
		self.rand_lbls.push(r);
		poly
	}
	// Little endian
	fn assemble_u(&self, k: u32) -> Vec<Zp> {
		let mut u = Vec::new();
		u.extend(&self.curr_gate);	
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
	fn assemble_gate_label(&self, r: &[Zp], k: Zp, g: usize) -> Vec<Zp> {
		let mut label_vec = Vec::new();
		label_vec.extend(r);
		label_vec.push(k);
		for i in (r.len() + 1)..self.num_bits {
			label_vec.push(Zp::new(math::get_bit(g, i) as u32));
		}
		label_vec
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
