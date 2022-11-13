#!/usr/bin/env python3


import cocotb
from cocotb.clock import Clock
from cocotb.triggers import Timer, RisingEdge
from cocotb.result import TestFailure, TestSuccess


@cocotb.test()
async def part_one(dut):

    dut.rst.value = 0
    dut.height.value = 0
    dut.clk.value = 0
    dut.en.value = 0
    cocotb.fork(Clock(dut.clk, 4, units="ns").start())
    for i in range(5):
        await RisingEdge(dut.clk)

    dut.rst.value = 1
    await RisingEdge(dut.clk)
    dut.rst.value = 0
    await RisingEdge(dut.clk)

    for i in range(5):
        await RisingEdge(dut.clk)

    expected_height_increases = 0
    with open("../../input/01.txt", "r") as f:
        s = f.read().split("\n")[:-1]
        s = [int(i) for i in s]
        prev_height = s[0]
        for i, h in enumerate(s):
            if i > 1:
                dut.en.value = 1
            if h > prev_height:
                expected_height_increases += 1
            prev_height = h
            await RisingEdge(dut.clk)
            dut.height.value = h

    await RisingEdge(dut.clk)
    dut.en.value = 0
    await RisingEdge(dut.clk)
    await RisingEdge(dut.clk)

    height_increases = int(dut.num_increases.value)

    if height_increases != expected_height_increases:
        raise TestFailure(
            "Expected %d height increases - got %d"
            % (expected_height_increases, height_increases)
        )
    else:
        print(int(dut.num_sum_increases.value))
        raise TestSuccess()
