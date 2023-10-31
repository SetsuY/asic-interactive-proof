use cmt_software::arith_circuit as circuit;
use cmt_software::runner;
use cmt_software::math_helper;
fn main() {
	let mut circ = circuit::ArithCircuit::new("circ.txt");
	// println!("{:?}", math_helper::lagrange_interpolation(&vec!((0, 10), (1, 11)), 2));
	if runner::run(&mut circ) {
		println!("Accept");
	} else {
		println!("Decline");
	}
}
