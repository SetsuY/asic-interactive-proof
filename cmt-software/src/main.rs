use std::time::Instant;
use cmt_software::arith_circuit as circuit;
use cmt_software::runner;

fn main() {
	let now = Instant::now();
	env_logger::init();
	let circ = circuit::ArithCircuit::new("circ.txt");
	println!("Circuit\n------------------------------------------");
	circ.print_circuit();
	println!("------------------------------------------");
	if runner::run(&circ) {
		println!("Accept");
	} else {
		println!("Reject");
	}
	println!("Program Runtime: {}ns", now.elapsed().as_nanos());
}
