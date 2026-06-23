; r0 = 코드
syscall:
    push r4
    movi r4, 1
    cmp r0, r4
    je sys_write

    movi r4, 2
    cmp r0, r4
    je sys_read


    pop r4
    sysret
    
; r1: 문자열 메모리 주소
; r2: len
; r3: (다른 인자인데 write엔 필요없으므로...) 실제 내보낼 문자 하나?
; r5: 목표 메모리
; r6: 문자열 메모리 주소 복사 (working)
sys_write:
    push r5
    push r6
    movr r5, r1
    addr r5, r2
    movr r6, r1

sys_write_loop:
    cmp r6, r5
    je sys_write_exit
    loadr r3, r6
    storei 49152, r3 ; 0xC000 = 49152, ㅠㅠ
    addi r6, 1
    jmp sys_write_loop

sys_write_exit:
    pop r6
    pop r5
    pop r4
    sysret

sys_read:
    pop r4
    sysret


timer_interrupt:
    sysret