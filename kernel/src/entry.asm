    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top #init stack point
    call rust_main        #main function
    
#distribute stack space
    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: