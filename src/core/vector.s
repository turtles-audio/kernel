.macro vector name
.section .vector.\name, "a"
    stack_\name: .word _stack_start
    reset_\name: .word boot
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    .word fault
    exceptions_\name: .skip 4
    checksum_\name: .word 0
    kernel_\name: .word 0
.global vector.\name
.endm

vector flash
vector root