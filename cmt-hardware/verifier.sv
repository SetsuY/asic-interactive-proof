module Verifier #(
	UINT_WIDTH = 32,
	NUM_LAYERS = 4,
	NUM_BITS = NUM_LAYERS - 1
	)(
	input logic clk, nrst, random,
	output logic accept,

	input logic [UINT_WIDTH-1:0] sample_pts [3],

	output logic [NUM_LAYERS-1:0] curr_layer,
	output logic [NUM_BITS-1:0] curr_gate,


	);
	logic [NUM_BITS-1:0] counter;
	logic [UINT_WIDTH-1:0] curr_result;

	logic [UINT_WIDTH-1:0] query_result;
	always_ff @(posedge clk, negedge nrst) begin
		if(!nrst) begin
			curr_layer <= 0;
			counter <= 0;
			curr_gate <= 0;
			accept <= 1;
		end else begin
			// First round
			if(curr_layer == 0) begin
				// Accumulate random bits here, save some pins at the cost of clock cycle
				if(counter < NUM_BITS) begin
					curr_gate <= {curr_gate[NUM_BITS-2:0], random};
					counter <= counter + 1;
				end else begin
					curr_result <= query_result;
					curr_layer <= curr_layer + 1;
				end
			// Middle rounds
			end else if(curr_layer < NUM_LAYERS) begin
				
			// Last round
			end else if(curr_layer == NUM_LAYERS) begin
				if(curr_result != query_result) begin
					accept <= 0;
				end
			end
		end
	end
endmodule

module SumcheckV #(
	UINT_WIDTH = 32,
	NUM_LAYERS = 4,
	NUM_BITS = NUM_LAYERS - 1
	)(
	input logic clk, nrst, random,
	output logic accept, done,

	input logic [UINT_WIDTH-1:0] sample_pts [3],

	input logic [NUM_LAYERS-1:0] curr_layer,
	input logic [NUM_BITS-1:0] curr_gate,
	input logic [UINT_WIDTH-1:0] curr_result,
	);
endmodule