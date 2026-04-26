// serial.zig
// Este módulo implementa as funções de E/S rs232
// void rs_write(struct tty_struct * queue);
// void rs_init(void);
// e todas as interrupções relacionadas à E/S serial.
// direitos autorais DCrust 16/04/2026

const std = @import("std");
const builtin = @import("builtin");

// Constantes
const WAKEUP_CHARS: u16 = 256 / 4; // TTY_BUF_SIZE = 256

// Portas seriais base
const COM1_PORT: u16 = 0x3F8;
const COM2_PORT: u16 = 0x2F8;

// Offsets dos registradores UART
const UART_DATA:    u16 = 0; // Data register
const UART_IER:     u16 = 1; // Interrupt Enable Register
const UART_FCR:     u16 = 2; // FIFO Control Register
const UART_LCR:     u16 = 3; // Line Control Register
const UART_MCR:     u16 = 4; // Modem Control Register
const UART_LSR:     u16 = 5; // Line Status Register
const UART_DLL:     u16 = 0; // Divisor Latch Low (com DLAB=1)
const UART_DLH:     u16 = 1; // Divisor Latch High (com DLAB=1)

// Bits do LSR
const UART_LSR_DR:   u8 = 0x01; // Data Ready
const UART_LSR_THRE: u8 = 0x20; // Transmitter Holding Register Empty

// Estruturas
const Queue = extern struct {
    data: u16, // porta base
};

const tty_struct = extern struct {
    read_q:  Queue,
    write_q: Queue,
};

// Referências externas para as funções de interrupção
extern fn rs1_interrupt() void;
extern fn rs2_interrupt() void;

// ─── E/S inline (simulando outb_p / inb_p do kernel original) ───────────────

inline fn outb_p(value: u8, port: u16) void {
    asm volatile ("outb %al, %dx"
        :
        : [value] "{al}" (value),
          [port]  "{dx}" (port)
        : "memory"
    );
    // Pequeno delay (simulando outb_p)
    asm volatile ("jmp 1f\n1:\tjmp 1f\n1:" ::: "memory");
}

inline fn inb_p(port: u16) u8 {
    var result: u8 = undefined;
    asm volatile ("inb %dx, %al"
        : [result] "={al}" (result)
        : [port]   "{dx}"  (port)
        : "memory"
    );
    // Pequeno delay (simulando inb_p)
    asm volatile ("jmp 1f\n1:\tjmp 1f\n1:" ::: "memory");
    return result;
}

// ─── Inicialização da porta serial ──────────────────────────────────────────

fn init_serial_port(port: u16, irq_enable: bool) void {
    // Desabilita interrupções
    outb_p(0x00, port + UART_IER);

    // Habilita DLAB para configurar baud rate (115200 bps)
    outb_p(0x80, port + UART_LCR);
    outb_p(0x01, port + UART_DLL); // divisor low  = 1 → 115200 bps
    outb_p(0x00, port + UART_DLH); // divisor high = 0

    // 8 bits, sem paridade, 1 stop bit — desabilita DLAB
    outb_p(0x03, port + UART_LCR);

    // Habilita FIFO, limpa, 14 bytes threshold
    outb_p(0xC7, port + UART_FCR);

    // Habilita RTS, DSR
    outb_p(0x0B, port + UART_MCR);

    // Habilita interrupções se pedido
    if (irq_enable) {
        outb_p(0x01, port + UART_IER);
    }
}

// ─── rs_init ────────────────────────────────────────────────────────────────

pub fn rs_init() void {
    init_serial_port(COM1_PORT, true);  // COM1 com IRQ
    init_serial_port(COM2_PORT, true);  // COM2 com IRQ
}

// ─── rs_write ────────────────────────────────────────────────────────────────

pub fn rs_write(tty: *const tty_struct) void {
    const port = tty.write_q.data;

    // Espera o transmissor estar pronto
    while ((inb_p(port + UART_LSR) & UART_LSR_THRE) == 0) {}

    // Envia byte fictício (0x00) — substitua pela lógica real da fila
    outb_p(0x00, port + UART_DATA);
}

// ─── Verificação de dados recebidos ─────────────────────────────────────────

pub fn rs_data_ready(port: u16) bool {
    return (inb_p(port + UART_LSR) & UART_LSR_DR) != 0;
}

pub fn rs_read_byte(port: u16) u8 {
    while (!rs_data_ready(port)) {}
    return inb_p(port + UART_DATA);
}

// ─── main (teste) ────────────────────────────────────────────────────────────

pub fn main() void {
    rs_init();
    std.debug.print("Serial inicializado: COM1={x}, COM2={x}\n", .{ COM1_PORT, COM2_PORT });
}
