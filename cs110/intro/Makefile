CC ?= cc
CFLAGS ?= -Wall -Wextra -std=c11

SOURCES := $(wildcard *.c)
PROGRAMS := $(SOURCES:.c=)

.PHONY: all clean

# Source-name goals are aliases, so force them only when requested directly.
ifneq ($(filter $(SOURCES),$(MAKECMDGOALS)),)
.PHONY: $(filter $(SOURCES),$(MAKECMDGOALS))
endif

all: $(PROGRAMS)

# Build one executable for each C source file.
$(PROGRAMS): %: %.c
	$(CC) $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $< $(LDLIBS) -o $@

# Allow `make program.c` while keeping the executable named `program`.
$(SOURCES):
	$(CC) $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $@ $(LDLIBS) -o $(@:.c=)

clean:
	rm -f $(PROGRAMS)
