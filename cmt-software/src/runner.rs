use super::arith_circuit;
use super::verifier;

pub fn run(circ: &mut arith_circuit::ArithCircuit) -> bool {
	let mut veri = verifier::Verifier::new(circ);
	veri.verify();
	true
}