use std::{fs, io::{stdout, Write}};
use super::math_helper::Zp;

#[derive(Copy, Clone)]
pub struct Gate {
	// Input gates
	w0: usize,
	w1: usize,
	is_add: bool,
	value: Zp,
}

impl Gate {
	fn new(w0: usize, w1: usize, is_add: bool, value: u32) -> Gate {
		Gate {
			w0: w0,
			w1: w1,
			is_add: is_add,
			value: Zp::new(value),
		}
	}
	pub fn get_wiring(&self) -> (usize, usize) {
		(self.w0, self.w1)
	}
	pub fn is_add(&self) -> bool {
		self.is_add
	}
	pub fn val(&self) -> Zp {
		self.value
	}
}

pub struct ArithCircuit {
	circuit: Vec<Vec<Gate>>,
	pub curr_layer: usize,
	pub num_bits: usize,
}

impl ArithCircuit {
	pub fn new(fname: &str) -> ArithCircuit {
		// Format:
		// each layer is one line
		// w0,w1,+ w0,w1,* ...
		// First line output gate; Last line input values only
		let mut circ = ArithCircuit {
			circuit: Vec::new(),
			curr_layer: 0,
			num_bits: 0,
		};
		let file: String = fs::read_to_string(fname).unwrap();
		// Parse file
		for l in file.lines() {
			let mut curr_layer: Vec<Gate> = Vec::new();
			if l.contains(",") {
				for gates in l.split_whitespace() {
					let vals: Vec<&str> = gates.split(",").collect();
					if vals.len() != 3 {
						panic!("Wrong format at layer {}", circ.curr_layer);
					}
					if !(vals[2].eq("+") || vals[2].eq("*")) {
						panic!("Wrong format at layer {}", circ.curr_layer);
					}
					curr_layer.push(Gate::new(
						vals[0].parse().unwrap(),
						vals[1].parse().unwrap(),
						vals[2].eq("+"), 0));
				}
			} else {
				for input in l.split_whitespace() {
					curr_layer.push(Gate::new(0, 0, false, input.parse().unwrap()));
				}
				circ.circuit.push(curr_layer);
				break;
			}
			circ.circuit.push(curr_layer);
			circ.curr_layer += 1;
		}
		circ.evaluate_circuit();
		circ.num_bits = circ.circuit.len() - 1;
		circ.curr_layer = 0;
		return circ;
	}
	fn evaluate_circuit(&mut self) {
		let circ = &mut self.circuit;
		for i in (0..circ.len() - 1).rev() {
			for j in 0..circ[i].len() {
				let w0 = circ[i+1][circ[i][j].w0].value;
				let w1 = circ[i+1][circ[i][j].w1].value;
				if circ[i][j].is_add {
					circ[i][j].value = w0 + w1;
				} else {
					circ[i][j].value = w0 * w1;
				}
			}
		}
	}
	pub fn print_circuit(&self) {
		let circ = &self.circuit;
		let mut stdout_lock = stdout().lock();
		for layer in circ {
			for gate in layer {
				write!(stdout_lock, "{},{},{} ", gate.w0, gate.w1, gate.value).unwrap();
			}
			write!(stdout_lock, "\n").unwrap();
		}
	}
	pub fn get_inputs(&self) -> Vec<Gate> {
		// Expensive, but just a one-off thing
		self.circuit[self.circuit.len() - 1].clone()
	}
	pub fn set_curr_layer(&mut self, layer: usize) {
		self.curr_layer = layer % (self.circuit.len());
	}
	pub fn get_last_layer(&self) -> &Vec<Gate> {
		&self.circuit[self.curr_layer - 1]
	}
	pub fn get_this_layer(&self) -> &Vec<Gate> {
		&self.circuit[self.curr_layer]
	}
	pub fn num_gate_at_layer(&self) -> usize {
		self.circuit[self.curr_layer].len()
	}
	pub fn num_gate_at_last_layer(&self) -> usize {
		self.circuit[self.curr_layer - 1].len()
	}
	pub fn num_layers(&self) -> usize {
		self.circuit.len() - 1
	}
	pub fn next_layer(&mut self) {
		self.set_curr_layer(self.curr_layer + 1);
	}
	pub fn get_gate_val(&self, gate_lbl: usize) -> Zp {
		if gate_lbl < self.circuit[self.curr_layer].len() {
			self.circuit[self.curr_layer][gate_lbl].value
		} else {
			Zp::new(0)
			// panic!("Gate Lbl Overflow");
		}
		// self.circuit[self.curr_layer][gate_lbl % self.num_gate_at_layer()].value
	}
	pub fn get_gate_wiring(&self, gate_lbl: usize) -> (usize, usize) {
		/*if gate_lbl < self.circuit[self.curr_layer].len() {
			let gate = &self.circuit[self.curr_layer][gate_lbl];
			(gate.w0, gate.w1)
		} else {
			// panic!("Gate Lbl Overflow");
		}*/
		let gate = &self.circuit[self.curr_layer][gate_lbl % self.num_gate_at_layer()];
		(gate.w0, gate.w1)
	}
	pub fn is_gate_add(&self, gate_lbl: usize) -> bool {
		self.circuit[self.curr_layer - 1][gate_lbl % self.num_gate_at_last_layer()].is_add
	}
}

