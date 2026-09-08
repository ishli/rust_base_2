.section .rodata
input:  .asciz "%d"          # input
output: .asciz "%d\n"        # output

.section .text
.globl main
main:
    subq    $56, %rsp         # 56 = 32 shadow + 8 выравнивания + 16 локальных (типа так правильно выравнивать для Windows x64)

    # scanf("%d", &n)  – адрес n хранится по смещению 40(%rsp)
    leaq    input(%rip), %rcx   # в 1 аргумент функции загруаем адрес строки входного intput формата
    leaq    40(%rsp), %rdx       # во 2 аргумент адрес переменной x (40 = 32 + 8)
    call    scanf               # запись введенное число в адрес 40(%rsp)

    movl    40(%rsp), %r8d      # загружаем ч в регистр r8d

    # случай x == 0
    cmpl    $0, %r8d
    je      case_zero

    # случай x == 1
    cmpl    $1, %r8d
    je      case_one

    # общий случай: x >= 2
    movl    $0, %r9d          # a = 0 в r9d
    movl    $1, %r10d         # b = 1 в r10d
    movl    $2, %r11d         # i = 2 в r11d

loop:
    cmpl    %r11d, %r8d       # сравниваем x и i
    jl      end_loop          # если i > x, выходим из цикла

    # c = a + b
    movl    %r9d, %eax        # копируем a в eax
    addl    %r10d, %eax       # eax = a + b

    # a = b, b = c
    movl    %r10d, %r9d       # a = b
    movl    %eax, %r10d       # b = c

    # i++
    addl    $1, %r11d
    jmp     loop

end_loop:
    movl    %r10d, %edx       # помещаем результат в edx  (2 аргумент для printf)
    jmp     print

case_zero:
    movl    $0, %edx
    jmp     print

case_one:
    movl    $1, %edx

print:
    leaq    output(%rip), %rcx   # 1‑й аргумент: строка формата
    call    printf                # выводим число

    xorq    %rax, %rax        # возвращаем 0
    addq    $56, %rsp         # восстанавливаем стек
    ret