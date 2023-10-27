use std::fs;
use super::circuit_conf as conf;

#[derive(Copy, Clone)]
struct Gate {
	// Input gates
	w0: usize,
	w1: usize,
	is_add: bool,
	value: u32,
}

impl Gate {
	fn new(w0: usize, w1: usize, is_add: bool, value: u32) -> Gate {
		Gate {
			w0: w0,
			w1: w1,
			is_add: is_add,
			value: value,
		}
	}
}

pub struct ArithCircuit {
	circuit: Vec<Vec<Gate>>,
	curr_layer: usize,
}

enum FileReadState {
	Read0,
	Read1,
	ReadType,
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
					if vals[2] != "+" || vals[2] != "*" {
						panic!("Wrong format at layer {}", circ.curr_layer);
					}
					curr_layer.push(Gate::new(
						vals[0].parse().unwrap(),
						vals[1].parse().unwrap(),
						vals[2] == "+", 0));
				}
			} else {
				for input in l.split_whitespace() {
					curr_layer.push(Gate::new(0, 0, false, input.parse().unwrap()));
				}
			}
			circ.circuit.push(curr_layer);
			circ.curr_layer += 1;
		}
		circ.evaluate_circuit();
		return circ;
	}
	fn get_curr_layer(&mut self) -> &mut Vec<Gate> {
		&mut self.circuit[self.curr_layer]
	}
	fn evaluate_circuit(&mut self) {
		for i in (0..self.circuit.len() - 1).rev() {
			for j in 0..self.circuit[i].len() {
				let w0 = self.circuit[i+1][self.circuit[i][j].w0].value;
				let w1 = self.circuit[i+1][self.circuit[i][j].w1].value;
				if self.circuit[i][j].is_add {
					self.circuit[i][j].value = (w0 + w1) % conf::PRIME;
				} else {
					self.circuit[i][j].value = (w0 * w1) % conf::PRIME;
				}
			}
		}
	}
}

