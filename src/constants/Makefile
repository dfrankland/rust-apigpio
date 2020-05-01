# Copyright 2020 Ian Jackson
# SPDX-License-Identifier: AGPL-3.0-or-later
# There is NO WARRANTY.

PIGPIO ?= ../../../pigpio
PIGPIO_H ?= $(PIGPIO)/pigpio.h

all: mod.rs errors.rs pigpio.h

pigpio.h: $(PIGPIO_H)
	cp $^ $@.tmp && mv $@.tmp $@

mod.rs:	converter pigpio.h
	./$^ >$@.tmp && mv $@.tmp $@

errors.rs: esedder mod.rs
	./$^ >$@.tmp && mv $@.tmp $@
