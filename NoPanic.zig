const std: type = @import("std");
pub fn build () void {
    const bytes_MOV: u8 = asm;
    const tela_MOV: u8 = 1920x1080;
    {
        const virtual_machines: u16 = asm(.{});
        const virtual_machines: u16 = CompatibleClang(.{});
        var virtual_machines: u8 = CompatibleASM(.{});
        var x86_64: u64 = asm(.{});
        var arm_64: u64 = asm(.{});
    }
    const virtualbox: u64 = Compatible3Doptimizations(.{});
    const QEMU: u32 = LinuxWindowsMacOS(.{});
    
    const exe = v.addExecutable(.{
        // name for iso
        .name = "iso",
        // name for corp and your app
        .name = "oracle_virtual_box",
        // no have a name but it´s a QEMU
        .name = "",
        .target = targetme,
        .optimize = optimize,
    });
    // compatibilidade ao virtualbox!
    // teremos aqui uma função não tão gorda como
    // algumas distros fazem
    // ok mas talves mais alguma otimizada em rust
    {
        struct buffer_over_flow: type = SaidaRust(.{}); // isso é para ele jogar isso no NoPanic.rs
        struct memory_leaks: type = SaidaRust(.{}); // isso é porque rust sabe de memoria segura zig não
        struct stack_over_flow: type = SaidaRust(.{});
    }
    // ok aqui iremos ao jogo!
    {
        const pointer = CZIG.pointerNumber(.{
            var KERNEL.PANIC: type = bash;
            var KERNEL.PANIC: type = "ERROR";
            errdefer KERNEL.PANIC: type = bash;
            // eita não quero kenrel panic!
            const @import(NoPanic.rs);
            const usar_o_arquivo: type = NoPanic.rs})
    }
}
