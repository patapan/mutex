cmake_minimum_required(VERSION 3.10)

# Set the project name
project(Mutex)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable
add_executable(mutex src/main.cc src/orderbook.cc)

# Include directories
target_include_directories(mutex PRIVATE src)

# Add warnings for better debugging
if (CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    target_compile_options(mutex PRIVATE -Wall -Wextra -Wpedantic)
elseif (MSVC)
    target_compile_options(mutex PRIVATE /W4)
endif()
