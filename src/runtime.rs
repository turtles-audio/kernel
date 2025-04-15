use core::arch::global_asm;

global_asm!(include_str!("core/boot.s"));
global_asm!(include_str!("core/vector.s"));
