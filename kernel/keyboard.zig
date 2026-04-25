    );
}

fn outb(port: u16, value: u8) void {
    asm volatile ("outb %[value], %[port]"
        :
        : [value] "{al}" (value),
          [port] "N{dx}" (port),
    );
}

fn put_queue(queue: *Queue, value: u32) void {
    var ecx = queue.head;
    var eax = value;
    var ebx: u32 = 0;

    while (true) {
        queue.buf[ecx] = @truncate(eax);
        ecx = (ecx + 1) & (BUFFER_SIZE - 1);
        
        if (ecx == queue.tail) return; // Buffer cheio
        
        eax = (eax >> 8);
        if (eax == 0) break;
    }
    
    queue.head = ecx;
    if (queue.proc_list) |proc| {
        @as(*u32, @ptrFromInt(proc)).* = 0;
    }
}

// Manipulação de modos
fn set_ctrl() void {
    var al: u8 = 0x04;
    if (e0 != 0) al += al;
    mode |= al;
}

fn unset_ctrl() void {
    var al: u8 = 0x04;
    if (e0 != 0) al += al;
    al = ~al;
    mode &= al;
}

fn 1024set_alt() void {
Relix-0.01v/Relix-0.01-mega/kernel/keyboard.zig [+]                                                                                                                                               116,7          24%

