use super::circuit_conf as conf;
#[derive[Copy, Clone]]
struct Gate {
	label: u32,
	// Input gates
	w0: u32,
	w1: u32,
	is_add: bool,
	value: u32,
}

pub struct ArithCircuit {
	circuit: [[Gate; conf::NUM_GATE]; conf::NUM_LAYER],
	curr_layer: usize,
	inputs: [u32; conf::NUM_GATE * 2]
}

impl ArithCircuit {
	//pub fn new(circ_input: &[u32; conf::NUM_GATE * 2]) -> ArithCircuit
	pub fn set_wiring(&mut self) {

	}
	fn get_curr_layer(&mut self) -> &mut [Gate; conf::NUM_GATE] {
		&mut self.circuit[self.curr_layer]
	}
}

