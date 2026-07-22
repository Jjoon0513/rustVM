movi r5, 0
movi r4, 5
movi r0, 1
movi r2, 14
movi r1, msg

write_loop:
    syscall ; sys_write
    cmp r5, r4
    je write_exit
    subi r4, 1
    jmp write_loop
write_exit:
    movi r0, 5
    syscall ; sys_halt

msg:
    db 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x4A, 0x6A, 0x6F, 0x6F, 0x6E, 0x21, 0x0A
