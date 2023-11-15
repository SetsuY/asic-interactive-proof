module Wiring #(
	UINT_WIDTH = 32,
	NUM_LAYERS = 4,
	NUM_BITS = NUM_LAYERS - 1
	)(
	input logic [NUM_BITS-1:0] query_gate,
	input logic [NUM_LAYERS-1:0] query_layer,
	output logic [UINT_WIDTH-1:0] gate_result,
	output logic [NUM_BITS-1:0] conn_gate [2],
	output logic isAdd
	);
endmodule
