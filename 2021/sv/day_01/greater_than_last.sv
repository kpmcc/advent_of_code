module greater_than_last
(
 input logic clk,
 input logic rst,
 input logic en,
 input logic[15:0] height,
 output logic[15:0] num_increases,
 output logic[15:0] num_sum_increases
);

   logic [15:0]     previous_heights[0:2] ;
   logic [15:0]     previous_sum;
   logic [15:0]     sum;

   logic [2:0]      delay;

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

   always_ff @ (posedge clk) begin
      previous_heights[0] <= height;
      previous_heights[1] <= previous_heights[0];
      previous_heights[2] <= previous_heights[1];
      end

   always_ff @ (posedge clk) begin
      if (en) begin
         if (height > previous_heights[0]) begin
            num_increases <= num_increases + 1;
         end
      end
   end

   always_ff @ (posedge clk) begin
      if (en) begin
         delay <= {delay[1:0], 1'b1};
      end
    end

   always_ff @ (posedge clk) begin
      if (rst) begin
         previous_sum <= 0;
      end else begin
            previous_sum <= sum;
      end
    end

   always_ff @ (posedge clk) begin
      if (rst) begin
         sum <= 0;
      end else begin
         sum <= sum + height - previous_heights[2];
         end
      end

   always_ff @ (posedge clk) begin
      if (delay[2] == 1'b1) begin
         if (sum > previous_sum) begin
            num_sum_increases <= num_sum_increases + 1;
            end
         end
      end

   initial begin
      $dumpvars(1, greater_than_last);
      end

  endmodule
