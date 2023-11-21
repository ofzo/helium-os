    .section .text.entry
    .global _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .global boot_stack_lower_bound

boot_stack_lower_bound: # 栈能够增长到的下限位置(低地址)
    .space 4096 * 16 # 64kb
    .global boot_stack_top

boot_stack_top: # 来标识栈顶的位置
