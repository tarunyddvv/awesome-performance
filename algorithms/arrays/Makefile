CC ?= cc
CFLAGS ?= -Wall -Wextra -std=c11
CXX ?= c++
CXXFLAGS ?= -Wall -Wextra -std=c++17
FORMATTER ?= clang-format

C_SOURCES := $(wildcard *.c)
CXX_SOURCES := $(wildcard *.cpp)
C_PROGRAMS := $(C_SOURCES:.c=)
CXX_PROGRAMS := $(CXX_SOURCES:.cpp=)
SOURCES := $(C_SOURCES) $(CXX_SOURCES)
PROGRAMS := $(C_PROGRAMS) $(CXX_PROGRAMS)

.PHONY: all clean format

# Source-name goals are aliases, so force them only when requested directly.
ifneq ($(filter $(SOURCES),$(MAKECMDGOALS)),)
.PHONY: $(filter $(SOURCES),$(MAKECMDGOALS))
endif

all: $(PROGRAMS)

format:
	$(FORMATTER) -i $(SOURCES)

# Build one executable for each C source file.
$(C_PROGRAMS): %: %.c
	$(CC) $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $< $(LDLIBS) -o $@

# Build one executable for each C++ source file with the C++ linker.
$(CXX_PROGRAMS): %: %.cpp
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(LDFLAGS) $< $(LDLIBS) -o $@

# Allow `make program.c` or `make program.cpp` while keeping the executable name.
$(C_SOURCES):
	$(CC) $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $@ $(LDLIBS) -o $(@:.c=)

$(CXX_SOURCES):
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(LDFLAGS) $@ $(LDLIBS) -o $(@:.cpp=)

clean:
	rm -f $(PROGRAMS)
