
PIGPIO ?= ../../../pigpio
PIGPIO_H ?= $(PIGPIO)/pigpio.h

all: mod.rs pigpio.h

pigpio.h: $(PIGPIO_H)
	cp $^ $@.tmp && mv $@.tmp $@

mod.rs:	converter pigpio.h
	./$^ >$@.tmp && mv $@.tmp $@
