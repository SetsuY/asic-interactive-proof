use cmt_software::arith_circuit as circuit;
use cmt_software::runner;

fn main() {
	env_logger::init();
	let mut circ = circuit::ArithCircuit::new("circ.txt");
	println!("Circuit\n------------------------------------------");
	circ.print_circuit();
	println!("------------------------------------------");
	if runner::run(&mut circ) {
		println!("Accept");
	} else {
		println!("Reject");
	}
}
