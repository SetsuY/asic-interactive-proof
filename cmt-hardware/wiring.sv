module WiringAdd #(
	parameter LN_LAYER = 1,
	parameter G = 3,
	parameter NUM_BITS = 3,
	parameter LN_G = 1
	)(
	input logic [LN_G:0] currGate,
	input logic [LN_LAYER:0] currLayer,
	output logic [NUM_BITS:0] connGate,
	output logic isAdd
	);
endmodule

module WiringMul #(
	parameter LN_LAYER = 1,
	parameter G = 3,
	parameter NUM_BITS = 3,
	parameter LN_G = 1
	)(
	input logic [LN_G:0] currGate,
	input logic [LN_LAYER:0] currLayer,
	output logic [NUM_BITS:0] connGate,
	output logic isMul
	);
endmodule