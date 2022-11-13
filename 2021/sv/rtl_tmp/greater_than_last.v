module greater_than_last (
	clk,
	rst,
	en,
	height,
	num_increases,
	num_sum_increases
);
	input wire clk;
	input wire rst;
	input wire en;
	input wire [15:0] height;
	output reg [15:0] num_increases;
	output reg [15:0] num_sum_increases;
	reg [15:0] previous_heights [0:2];
	reg [15:0] previous_sum;
	reg [15:0] sum;
	reg [2:0] delay;
	initial begin
		num_increases = 0;
		num_sum_increases = 0;
		previous_heights[0] = 0;
		previous_heights[1] = 0;
		previous_heights[2] = 0;
		delay[0] = 0;
		delay[1] = 0;
		delay[2] = 0;
		previous_sum = 0;
		sum = 0;
	end
	always @(posedge clk) begin
		previous_heights[0] <= height;
		previous_heights[1] <= previous_heights[0];
		previous_heights[2] <= previous_heights[1];
	end
	always @(posedge clk)
		if (en)
			if (height > previous_heights[0])
				num_increases <= num_increases + 1;
	always @(posedge clk)
		if (en)
			delay <= {delay[1:0], 1'b1};
	always @(posedge clk)
		if (rst)
			previous_sum <= 0;
		else
			previous_sum <= sum;
	always @(posedge clk)
		if (rst)
			sum <= 0;
		else
			sum <= (sum + height) - previous_heights[2];
	always @(posedge clk)
		if (delay[2] == 1'b1)
			if (sum > previous_sum)
				num_sum_increases <= num_sum_increases + 1;
	initial $dumpvars(1, greater_than_last);
endmodule
