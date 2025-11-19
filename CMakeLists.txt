cmake_minimum_required(VERSION 4.0)
project(build_msp)

set(CMAKE_CXX_STANDARD 23)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    set(CMAKE_CXX_FLAGS "/W4 /EHsc")
    set(CMAKE_CXX_FLAGS_DEBUG "/Zi")
    set(CMAKE_CXX_FLAGS_RELEASE "/O3")
else()
    set(CMAKE_CXX_FLAGS "-Wall -Wextra")
    set(CMAKE_CXX_FLAGS_DEBUG "-g")
    set(CMAKE_CXX_FLAGS_RELEASE "-O3")
endif()

add_executable(build_msp src/main.cpp
    switch-tools/src/filepath.c
    switch-tools/src/pfs0.c
    switch-tools/src/romfs.c
)
target_include_directories(build_msp PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/switch-tools/src)
