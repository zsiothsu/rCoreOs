# rCoreOs

This project is used as the learning and use of operating system courses,

**rCoreOs** is an operating system based on the RISC-V architecture, written in rust, and can run on qemu and K210. The reference tutorial is the operating system textbook [rCore-Tutorial-Book Third Edition](https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html) written by Professor Chen Yu of Tsinghua University and his student Wu Yifan.

This project modified some structure and code on  origial one, and there are reports for each chapter in the `report` folder.

## Directory composition

>./bootloader/ Program running on the hardware layer, providing SBI of rust os
>./kernel/ Operating system kernel running on the S layer, and provides API for the U layer, using qemu-system-riscv64 to run
>./user/ Runs on the U layer, runs on the os layer or the system layer provided by qemu-riscv64
>./reports/ Reports
>./Makefile & ./linker.sh project build script