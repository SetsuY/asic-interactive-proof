use std::fs::File;
use std::io::Write;
use rand;

fn main() {
	let num_layer: usize = 50;
	let num_gate: usize = 100;
	let mut f = File::create("circ.txt").unwrap();
	for _i in 0..num_layer {
		for _j in 0..num_gate {
			let w0 = rand::random::<usize>() % num_gate;
			let w1 = rand::random::<usize>() % num_gate;
			let gate_type = if rand::random() {"+"} else {"*"};
			f.write(&format!("{},{},{} ", w0, w1, gate_type).as_bytes()).unwrap();
		}
		f.write(b"\n").unwrap();
	}
	for _i in 0..(num_gate * 2) {
		let input = rand::random::<usize>() % 255;
		f.write(&format!("{} ", input).as_bytes()).unwrap();
	}
}
