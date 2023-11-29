use std::time::Instant;
use super::arith_circuit;
use super::verifier;

pub fn run(circ: &arith_circuit::ArithCircuit) -> bool {
	let mut veri = verifier::Verifier::new(circ);
	let now = Instant::now();
	let result = veri.verify();
	println!("Proof Runtime: {}ns", now.elapsed().as_nanos());
	result
}