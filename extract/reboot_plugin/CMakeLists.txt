project(RebootPlugin)
cmake_minimum_required(VERSION 4.0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(./external/aviutl2_sdk_mirror/include)

add_library(RebootPlugin SHARED src/RebootPlugin.cpp)
target_compile_definitions(RebootPlugin PRIVATE UNICODE _UNICODE)
target_compile_options(RebootPlugin PRIVATE /utf-8)
set_target_properties(RebootPlugin PROPERTIES OUTPUT_NAME "RebootPlugin")
