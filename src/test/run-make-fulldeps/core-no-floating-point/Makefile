-include ../tools.mk

all:
	$(RUSTC) --edition=2018 --crate-type=rlib ../../../../library/core/src/lib.rs --cfg no_floating_point
