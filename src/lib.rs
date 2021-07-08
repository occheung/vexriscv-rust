//! Low level access to VexRiscv RISC-V cores

#![no_std]
#![cfg_attr(feature = "inline-asm", feature(llvm_asm))]

pub mod register;
