# The file is generated by linker.sh

    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 2
    .quad app_0_start
    .quad app_1_start
    .quad app_1_end 

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "/run/media/chipen/data/Pro_OS/rCoreOs/user/target/riscv64gc-unknown-none-elf/release/01HelloWorld.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "/run/media/chipen/data/Pro_OS/rCoreOs/user/target/riscv64gc-unknown-none-elf/release/02test.bin"
app_1_end:
