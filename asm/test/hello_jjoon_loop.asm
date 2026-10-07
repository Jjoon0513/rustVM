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
    db "HELLO, JJOON!", 0x0A
