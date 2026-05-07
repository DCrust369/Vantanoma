; Bootloader em assembly ARM 64 bits

; Definindo as constantes
STACK_SIZE = 4096

; Definindo as rotinas de inicialização
section .data
align 16
stack:
  resb STACK_SIZE
; Fim da seção de dados

section .text
align 16
global _start
_start:
; Inicializar o stack
mov sp, stack + STACK_SIZE
; Fim da inicialização do stack

; Carregar o kernel
mov x0, 0x8000 ; endereço do kernel
ldr x1, =kernel
ldr x2, =mode_l
ldr x3, =cst_z
svc #0 ; Carregar o kernel

; Checar se o kernel foi carregado com sucesso
cmp x0, 0
bne erro_kernel

; Entrar no modo usuario
ldr x0, =mode_u
ldr x1, =cst_z
svc #0

; Fim da inicialização
loop:
  bkpt 0 ; Instrução BKPT para pausar o processador

; Rotina de tratamento de erro (kernel não foi carregado)
erro_kernel:
  mov x0, 1
  mov x1, 0
  mov x2, 0
  mov x3, 0
  svc #0

; Rotina de tratamento de erro (modo não foi definido)
erro_modo:
  mov x0, 1
  mov x1, 0
  mov x2, 0
  mov x3, 0
  svc #0

; Kernel
; Definindo as constantes
KERNEL_SIZE = 4096

; Definindo as rotinas de inicialização
section .data
align 16
kernel:
  resb KERNEL_SIZE
; Fim da seção de dados

section .text
align 16
global kernel_start
kernel_start:
; Inicializar o kernel
; Fim da inicialização do kernel

; Rotina de tratamento de erro (modo não foi definido)
erro_modo:
  mov x0, 1
  mov x1, 0
  mov x2, 0
  mov x3, 0
  svc #0

; Fim do kernel
; Fim do bootloader
