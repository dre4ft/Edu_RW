# Compiler and flags
CXX = g++
CXXFLAGS = -std=c++17 -Wall -O2

# Project name
TARGET = crypter

# Source files
SRC = main.cpp \
      crypter.cpp \
      AES_256.cpp

# Header files
HEADERS = crypter.h AES_256.hpp

# Object files
OBJ = $(SRC:.cpp=.o)

# Build target
$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJ)

# Compile source files into object files
%.o: %.cpp $(HEADERS)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean build artifacts
clean:
	rm -f $(OBJ) $(TARGET)

# Phony targets
.PHONY: clean
