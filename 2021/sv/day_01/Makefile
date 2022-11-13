# Makefile

# defaults
SIM ?= icarus
TOPLEVEL_LANG ?= verilog

VERILOG_SOURCES += $(PWD)/../rtl_tmp/greater_than_last.v

# TOPLEVEL is the name of the toplevel module in your Verilog or VHDL files
TOPLEVEL = greater_than_last

# Module is the basename of the Python test file
MODULE = main

# include cocotb's make rules to take care of the simulator setup
include $(shell cocotb-config --makefiles)/Makefile.sim
