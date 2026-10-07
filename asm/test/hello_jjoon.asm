
movi r0, 1
movi r1, msg
movi r2, 14
syscall ; sys_write

movi r0, 5
syscall ; sys_halt

msg:
    db "HELLO, JJOON!", 0x0A
