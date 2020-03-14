
PIGPIO ?= ../../../pigpio
PIGPIO_H ?= $(PIGPIO)/pigpio.h

all: mod.rs

mod.rs:	converter $(PIGPIO_H)
	./$^ >$@.tmp && mv $@.tmp $@
