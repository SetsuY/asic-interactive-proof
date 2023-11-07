use super::arith_circuit::ArithCircuit;
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
	pub fn get_gate_value(&self) -> (Zp, Zp) {
		(self.circuit.get_gate_val(self.curr_wiring.0), 
		self.circuit.get_gate_val(self.curr_wiring.1))
	}
	pub fn get_gate_is_add(&self) -> bool {
		self.circuit.is_gate_add(self.curr_gate)
	}
	pub fn query_rand_gate(&self) -> Result<(Zp, Zp), bool> {
		let lbl_l = self.assemble_rand_label(&self.rand_lbls[0..self.num_bits]);
		let lbl_r = self.assemble_rand_label(&self.rand_lbls[self.num_bits..]);
		if lbl_l >= self.circuit.num_gate_at_layer() || 
			lbl_l >= self.circuit.num_gate_at_layer() ||
			!self.circuit.is_gate_add(self.curr_gate) {
			return Err(false);
		}
		if lbl_l == self.curr_wiring.0 && lbl_r == self.curr_wiring.1 {
			return Ok(self.get_gate_value());
		}
		if lbl_l == self.curr_wiring.1 && lbl_r == self.curr_wiring.0 {
			return Ok(self.get_gate_value());
		}
		return Err(false);
	}
	pub fn sum_check(&mut self, round: usize, r: Zp) -> [Zp; 3] { 
		let mut poly: [Zp; 3] = [Zp::new(0), Zp::new(0), Zp::new(0)];
		for (i, gate) in self.circuit.get_this_layer().into_iter().enumerate() {
			let conn_gates = gate.get_wiring();
			let s = self.assemble_s(i, conn_gates.0, conn_gates.1);
			// TODO: Figure out how to deal with k = 2. Bit assemble won't work.
			for k in 0..2 {
				let u = self.assemble_u(k);
				let term_p = Self::calc_termp(&s, &u);
				let term_l;
				let term_r;
				if round <= self.num_bits {
					term_l = self.circuit.get_gate_val(
						self.assemble_gate_label(
						&self.rand_lbls, Zp::new(k), conn_gates.0));
					term_r = self.circuit.get_gate_val(conn_gates.1);
				} else {
					term_l = self.circuit.get_gate_val(
						self.assemble_rand_label(
						&self.rand_lbls[0..self.num_bits]));
					term_r = self.circuit.get_gate_val(
						self.assemble_gate_label(
						&self.rand_lbls[self.num_bits..],
						Zp::new(k), conn_gates.1));
				}
				if gate.is_add() {
					poly[k as usize] += term_p * (term_l + term_r);
				} else {
					poly[k as usize] += term_p * term_l * term_r;
				}
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
		let mut g_vec = Vec::new();
		for i in (r.len() + 1)..self.num_bits {
			g_vec.push(Zp::new(math::get_bit(g, i) as u32));
		}
		let mut label_vec = Vec::new();
		label_vec.extend(r);
		label_vec.push(k);
		label_vec.extend(g_vec);
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
