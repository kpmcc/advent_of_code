#!/usr/bin/env python3
import sys


def calculate_depths(input_file):
    with open(input_file, "r") as f:
        t = f.read()
        lines = t.strip().split("\n")
        ints = [int(i) for i in lines]

    depth_sum = sum(ints[0:3])
    depth_increase_count = 0
    for n in range(3, len(ints), 1):
        depths = ints[n - 2 : n + 1]
        new_depth_sum = sum(depths)
        if new_depth_sum > depth_sum:
            depth_increase_count += 1

        depth_sum = new_depth_sum

    return depth_increase_count


if __name__ == "__main__":
    first_arg = sys.argv[1]
    if first_arg == "-h":
        print("Advent of Code Day 01 Problem 2")
        print("Usage: 01_02.py -i input.txt")
    elif first_arg == "-i":
        input_file = sys.argv[2]
        print(calculate_depths(input_file))
