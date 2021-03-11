## 内容总结

1. 编写了 SBI 调用和系统调用
2. 用rust宏实现了print相关函数
3. 用 ld 文件指定了主存中段的分布
4. 用 SBI 调用 实现了字符打印和关机
5. 通过在 ld 文件中的标号在 rust 中获取内存分布

## 效果
```
[rustsbi] RustSBI version 0.1.1
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Platform: QEMU (Version 0.1.0)
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: 0x222
[rustsbi] medeleg: 0xb1ab
[rustsbi-dtb] Hart count: cluster0 with 1 cores
[rustsbi] Kernel entry: 0x80200000
[INFO] .text [0x80200000, 0x80203000)
[INFO] .rodata [0x80203000, 0x80204000)
[INFO] .data [0x80204000, 0x80204000)
[INFO] .bss [0x80214000, 0x80214000)
Hello World!
```

## 问答题

1. 略
2. 机器上电后，进入0x1000执行，即 bios 所在地址，然后跳转到 0x80200000
```ASM
0x1000:	auipc	t0,0x0
0x1004:	addi	a2,t0,40
0x1008:	csrr	a0,mhartid
0x100c:	ld	a1,32(t0)
0x1010:	ld	t0,24(t0)
0x1014:	jr	t0
```
3.进入到 .text 段执行代码