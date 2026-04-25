.section .data
    msg:    .ascii "Olá, Kernel!\n"
    len = . - msg

.section .text
.globl _start

_start:
    # syscall write(1, msg, len)
    movl $4, %eax        # número da syscall (write)
    movl $1, %ebx        # fd (stdout)
    leal msg, %ecx       # ponteiro para mensagem
    movl $len, %edx      # tamanho
    int $0x80            # interrupção do kernel

    # syscall exit(0)
    movl $1, %eax        # número da syscall (exit)
    xorl %ebx, %ebx      # código de retorno 0
    int $0x80
