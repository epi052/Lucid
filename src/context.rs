/// This file contains the logic necessary to perform context-switching between
/// our fuzzer and Bochs

use std::arch::asm;
use crate::loader::Bochs;

pub fn start_bochs(bochs: Bochs) {
    // Set RAX to our jump destination which is the program entry, clear RDX,
    // and set RSP to the correct value
    unsafe {
        asm!(
            "mov rax, {0}",
            "mov rsp, {1}",
            "xor rdx, rdx",
            "jmp rax",
            in(reg) bochs.entry,
            in(reg) bochs.rsp,
        );
    }
}
