// 3-layer for now, proof of concept
module Verifier #(
	parameter NUM_LAYER = 3,
	parameter LN_LAYER = 1,
	parameter G = 3,
	parameter NUM_BITS = 3,
	parameter LN_G = 1,
	parameter INT_WIDTH = 31
	)(
	input logic clk, nrst, random,
	input logic [LN_G:0] startGate,
	output logic [LN_G:0] currGate,
	output logic [LN_LAYER:0] currLayer,
	input logic [INT_WIDTH:0] result, // result := V_currLayer(currGate)
	input logic [INT_WIDTH:0] d_input[G+1],
	input logic [INT_WIDTH:0] poly[G+1],

	// output logic [LN_LAYER:0] queryLayer, // will always be currLayer - 1
	// gateSel := {r0, r1}
	output logic [NUM_BITS:0] gateSel,
	input logic [INT_WIDTH:0] gateRslt[2],

	output logic accept
	);
	// May need some sync to tell prover which H we are talking about
	wire [INT_WIDTH:0] v0 = poly[0];
	wire [INT_WIDTH:0] v1 = poly[0] + poly[1] + poly[2] + poly[3];

	logic sumCheckEn, sumAccept, sumDone, isAdd;
	logic [NUM_BITS:0] nextGates;
	SumcheckV #(.LN_LAYER(LN_LAYER), .G(G), .NUM_BITS(NUM_BITS), .LN_G(LN_G), .INT_WIDTH(INT_WIDTH)) sumV
		(.clk(clk), .nrst(nrst & sumCheckEn), .random(random), .currGate(currGate), .currLayer(currLayer - 1),
		.result(result), .v0(v0), .v1(v1), .gateSel(gateSel), .gateRslt(gateRslt),
		.accept(sumAccept), .done(sumDone), .connGate(nextGates), .isAdd(isAdd));

	logic [LN_LAYER+1:0] counter;
	assign currLayer = counter[LN_LAYER:0];
	logic [INT_WIDTH:0] a;
	always_ff @ (posedge clk, negedge nrst) begin
		if(!nrst) begin
			currGate <= 0;
			counter <= 0;
			sumCheckEn <= 0;
			accept <= 1;
		end else if(accept && counter <= NUM_LAYER + 1) begin
			// First round
			if(counter == 0) begin
				counter <= counter + 1;
				sumCheckEn <= 1;
				currGate <= startGate;
				a <= result; // Should be V_0(startGate) at this round
			/*
			Middle rounds
			1. Wait for sumDone
			2. De-assert sumCheckEn. This will reset sumcheck. At the same time read out stuff and do verification
			3. Assert sumCheckEn again for next layer. 
			*/
			end else if(counter < NUM_LAYER + 1) begin
				if(sumDone) begin
					sumCheckEn <= 0;
					if(!sumAccept)
						accept <= 0;
					if(a != (isAdd ? v0 + v1 : v0 * v1))
						accept <= 0;
					a <= random ? v1 : v0;
					currGate <= random ? nextGates[NUM_BITS:NUM_BITS/2+1] : nextGates[NUM_BITS/2:0];
				end else if(!sumCheckEn) begin
					sumCheckEn <= 1;
					counter <= counter + 1;
				end
			// Last round, just check input
			end else begin
				counter <= counter + 1;
				if(d_input[currGate] != a)
					accept <= 0;
			end
		end
	end
endmodule

// Sum check on each layer
module SumcheckV #(
	parameter LN_LAYER = 1,
	parameter G = 3,
	parameter NUM_BITS = 3,
	parameter LN_G = 1,
	parameter INT_WIDTH = 31
	)(
	input logic clk, nrst, random,
	input logic [LN_G:0] currGate,
	input logic [LN_LAYER:0] currLayer,

	input logic [INT_WIDTH:0] result,
	input logic [INT_WIDTH:0] v0, v1,

	output logic [NUM_BITS:0] gateSel,
	input logic [INT_WIDTH:0] gateRslt[2],

	output logic accept, done, isAdd,
	output logic [NUM_BITS:0] connGate
	);
	logic [LN_G+1:0] counter;
	logic [INT_WIDTH:0] a;
	logic [NUM_BITS:0] r_save;

	logic [INT_WIDTH:0] fRslt;
	// At final round, top bit won't be ready in r_save. Don't need this at other rounds either way
	assign gateSel = {random, r_save[NUM_BITS-1:0]};
	F #(.LN_LAYER(LN_LAYER), .G(G), .NUM_BITS(NUM_BITS), .LN_G(LN_G), .INT_WIDTH(INT_WIDTH)) fEval
		(.v0(gateRslt[0]), .v1(gateRslt[1]), .gateSel(gateSel), .currLayer(currLayer),
		.currGate(currGate), .result(fRslt), .connGate(connGate), .gateType(isAdd));

	always_ff @(posedge clk, negedge nrst) begin
		if(!nrst) begin
			accept <= 1;
			counter <= 0;
			a <= 0;
			r_save <= 0;
			done <= 0;
		end else if(accept && counter <= NUM_BITS) begin
			counter <= counter + 1;
			r_save[counter[LN_G:0]] <= random;
			if(counter == 0) begin
				if(v0 + v1 != result)
					accept <= 0;
				a <= random ? v1 : v0;
			end else if(counter < NUM_BITS) begin
				if(v0 + v1 != a)
					accept <= 0;
				a <= random ? v1 : v0;
			end else begin
				if(fRslt != (random ? v1 : v0)) 
					accept <= 0;
				done <= 1;
			end
		end
	end
endmodule

module F #(
	parameter LN_LAYER = 1,
	parameter G = 3,
	parameter NUM_BITS = 3,
	parameter INT_WIDTH = 31,
	parameter LN_G = 1
	)(
	input logic [LN_LAYER:0] currLayer,
	input logic [LN_G:0] currGate,
	input logic [INT_WIDTH:0] v0, v1,
	input logic [NUM_BITS:0] gateSel,
	output logic [INT_WIDTH:0] result,
	output logic [NUM_BITS:0] connGate,
	output logic gateType
	);
	logic [NUM_BITS:0] addGates, mulGates;
	logic isAdd, isMul;
	wire connValid = isAdd | isMul; // Debug use
	WiringAdd #(.LN_LAYER(LN_LAYER), .G(G), .NUM_BITS(NUM_BITS), .LN_G(LN_G)) add
		(.currGate(currGate), .currLayer(currLayer), .connGate(addGates), .isAdd(isAdd));
	WiringMul #(.LN_LAYER(LN_LAYER),.G(G), .NUM_BITS(NUM_BITS), .LN_G(LN_G)) mul
		(.currGate(currGate), .currLayer(currLayer), .connGate(mulGates), .isMul(isMul));
	always_comb begin
		result = (isAdd && (gateSel == addGates || 
			{gateSel[NUM_BITS/2:0], gateSel[NUM_BITS : NUM_BITS/2+1]} == addGates) ? v0 + v1 : 0)
			+ (isMul && (gateSel == mulGates ||
				{gateSel[NUM_BITS/2:0], gateSel[NUM_BITS : NUM_BITS/2+1]} == mulGates) ? v0 * v1 : 0);
		connGate = isAdd ? addGates : mulGates;
		gateType = isAdd;
	end
endmodule
