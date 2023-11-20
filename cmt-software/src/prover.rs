use log::info;
use super::arith_circuit::{ArithCircuit, GateLbl};
use super::math_helper as math;
use super::math_helper::Zp;

pub struct Prover<'a>{
	circuit: &'a ArithCircuit,
	num_bits: usize,
	curr_gate: GateLbl,
	rand_lbls: Vec<Zp>,
}

impl<'a> Prover<'a> {
	pub fn new(circ: &'a ArithCircuit, start_lbl: GateLbl) -> Prover {
		let p = Prover {
			num_bits: circ.num_bits,
			curr_gate: start_lbl,
			rand_lbls: Vec::new(),
			circuit: circ,
		};
		p.circuit.next_layer();
		p
	}
	pub fn next_layer(&mut self, next_gate: usize) {
		let rand_lbls = self.get_rand_gate();
		self.curr_gate = (rand_lbls.1 - rand_lbls.0) * next_gate + rand_lbls.0;
		self.curr_gate %= self.num_gate_at_layer();
		self.curr_wiring = self.circuit.get_gate_wiring(self.curr_gate);
		self.circuit.next_layer();
		self.rand_lbls.clear();
	}
	pub fn get_curr_wiring(&self) -> (usize, usize) {
		self.curr_wiring
	}
	pub fn get_gate_value(&self) -> (Zp, Zp) {
		(self.circuit.get_gate_val(self.curr_wiring.0), 
		self.circuit.get_gate_val(self.curr_wiring.1))
	}
	pub fn get_gate_is_add(&self) -> bool {
		self.circuit.is_gate_add(self.curr_gate)
	}
	pub fn query_rand_gate(&self) -> Result<(Zp, Zp), bool> {
		let lbl_l = self.get_rand_gate().0;
		let lbl_r = self.get_rand_gate().1;
		if lbl_l >= self.circuit.num_gate_at_layer() || 
			lbl_l >= self.circuit.num_gate_at_layer() {
			return Err(false);
		}
		if lbl_l == self.curr_wiring.0 && lbl_r == self.curr_wiring.1 {
			return Ok(self.get_gate_value());
		}
		if lbl_l == self.curr_wiring.1 && lbl_r == self.curr_wiring.0 {
			return Ok(self.get_gate_value());
		}
		Err(false)
	}
	pub fn get_all_vals(&self) -> Vec<(Zp, Zp)> {
		let mut vals: Vec<(Zp, Zp)> = Vec::new();
		let (lbl_l, lbl_r) = self.rand_lbls.split_at(self.num_bits);
		vals.push((0, self.circuit.mle_gate_val(lbl_l)));
		vals.push((1, self.circuit.mle_gate_val(lbl_r)));
		for i in 2..(self.num_bits + 1) {
			let curr_gate = math::interpolate_next_gates(self.rand_lbls, 
				Zp::new(i), self.num_bits);
			vals.push((i, self.circuit.mle_gate_val(curr_gate)));
		}
		vals
	}
	pub fn num_gate_at_layer(&self) -> usize {
		self.circuit.num_gate_at_layer()
	}
	pub fn sum_check(&mut self, round: usize, r: Zp) -> [Zp; 3] { 
		let mut poly: [Zp; 3] = [Zp::new(0), Zp::new(0), Zp::new(0)];
		for (i, gate) in self.circuit.get_last_layer().into_iter().enumerate() {
			let conn_gates = gate.get_wiring();
			let s = self.assemble_s(i, conn_gates.0, conn_gates.1);
			// TODO: Figure out how to deal with k = 2. Bit assemble won't work.
			for k in 0..2 {
				let u = self.assemble_u(k);
				let term_p = Self::calc_termp(&s, &u);
				let l_lbl;
				let r_lbl;
				if round < self.num_bits {
					l_lbl = self.assemble_gate_label(
						&self.rand_lbls, Zp::new(k), conn_gates.0);
					r_lbl = conn_gates.1;
				} else {
					l_lbl = self.assemble_rand_label(
						&self.rand_lbls[0..self.num_bits]);
					r_lbl = self.assemble_gate_label(
						&self.rand_lbls[self.num_bits..],
						Zp::new(k), conn_gates.1);
				}
				let term_l = self.circuit.get_gate_val(l_lbl);
				let term_r = self.circuit.get_gate_val(r_lbl);
				if gate.is_add() {
					poly[k as usize] += term_p * (term_l + term_r);
				} else {
					poly[k as usize] += term_p * term_l * term_r;
				}
				info!("Gate {}, {} {} / {} {} {}", i, l_lbl, r_lbl, term_p, term_l, term_r);
			}
		}
		self.rand_lbls.push(r);
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
	fn assemble_gate_label(&self, r: &[Zp], k: Zp, g: usize) -> usize {
		let mut label_vec = Vec::new();
		label_vec.extend(r);
		label_vec.push(k);
		for i in (r.len() + 1)..self.num_bits {
			label_vec.push(Zp::new(math::get_bit(g, i) as u32));
		}
		self.assemble_rand_label(&label_vec)
	}
	fn assemble_rand_label(&self, r: &[Zp]) -> usize {
		assert!(self.num_bits == r.len());
		let mut val: usize = 0;
		for (i, n) in r.into_iter().enumerate() {
			val |= (n.val() as usize) << i;
		}
		val
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
