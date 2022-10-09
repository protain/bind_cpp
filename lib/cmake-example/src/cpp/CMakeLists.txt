cmake_minimum_required(VERSION 3.10)
project("cmake exam for RUST" LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)

find_package(PkgConfig)
set(TARGET cmake-example)
add_library(${TARGET}
    STATIC
        test.cpp
)

install(TARGETS ${TARGET} DESTINATION .)
