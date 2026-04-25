// serial.zig
// Este módulo implementa as funções de E/S rs232
// void rs_write(struct tty_struct * queue);
// void rs_init(void);
// e todas as interrupções relacionadas à E/S serial.

const std = @import("std");
const builtin = @import("builtin");

// Constantes
const WAKEUP_CHARS = 256 / 4; // TTY_BUF_SIZE = 256

// Estruturas externas (definidas em outro lugar)
const tty_struct = extern struct {
    read_q: Queue,
    write_q: Queue,
    // outros campos...
};

const Queue = extern struct {
    data: u16, // porta base
    // outros campos...
};

// Referências externas para as funções de interrupção
extern fn rs1_interrupt() void;
extern fn rs2_interrupt() void;

// Funções de E/S inline (simulando as macros outb_p, inb_p)
inline fn outb_p(value: u8, port: u16) void {
    asm volatile ("outb %al, %dx"
        :
        : [value] "{al}" (value),
          [port] "{dx}" (port)
        : "memory"
    );
    // Pequeno delay (simulando outb_p)
    asm volatile ("jmp 1f\n1:\tjmp 1f\n1:" ::: "memory");
}

inline fn inb_p(port: u16) u8 {
    var result: u8 = undefined;
    asm volatile ("inb %dx, %al"
        : [result] "={al}" (result)
        : [port] "{dx}" (port)
        : "memory"
    );
Relix-0.01v/Relix-0.01-mega/kernel/serial.zig                                                                                                                                                     47,1          Topo

