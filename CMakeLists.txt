cmake_minimum_required(VERSION 3.25)
project(WB15CC)

add_custom_target(cargo_build
        COMMAND RUSTC_LOG=rustc_codegen_ssa::back::link=info cargo build)
