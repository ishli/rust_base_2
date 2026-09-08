.section .rodata
fmt: .asciz "%d\n"          # строка формата для printf – целое число и перевод строки

.section .text
.globl main                 
main:
    subq    $40, %rsp       

    leaq    fmt(%rip), %rcx     # через rcx передаем первый аргумент в функцию

    movq    $123456789, %rax  # число в  rax (64‑bit), это наш аккумулятор
    addq    $987654321, %rax  # прибавляем второе в rax
    movq    %rax, %rdx        # через rdx передаем второй аргумент в функцию

    call    printf            

    xorq    %rax, %rax        
    addq    $40, %rsp         
    ret                       