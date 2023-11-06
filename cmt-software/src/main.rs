use cmt_software::arith_circuit as circuit;
use cmt_software::runner;

fn main() {
	let mut circ = circuit::ArithCircuit::new("circ.txt");
	if runner::run(&mut circ) {
		println!("Accept");
	} else {
		println!("Decline");
	}
}
