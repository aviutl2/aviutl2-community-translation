project(ExtractPlugin)
cmake_minimum_required(VERSION 4.0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(./external/aviutl2_sdk_mirror/include)

add_library(ExtractPlugin SHARED src/ExtractPlugin.cpp)
target_compile_definitions(ExtractPlugin PRIVATE UNICODE _UNICODE)
target_compile_options(ExtractPlugin PRIVATE /utf-8)
set_target_properties(ExtractPlugin PROPERTIES OUTPUT_NAME "ExtractPlugin")
