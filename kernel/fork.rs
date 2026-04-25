// fork.rs — Rotinas auxiliares para a chamada de sistema 'fork'

use core::ptr;

// Constantes do sistema
const NR_TASKS: usize = 64;
const NR_OPEN: usize = 20;
const PAGE_SIZE: usize = 4096;
const EAGAIN: i32 = 11;
const ENOMEM: i32 = 12;

// ─── Tipos auxiliares ────────────────────────────────────────────────────────

#[repr(C)]
pub struct Tss {
    pub back_link:    u32,
    pub esp0:         u32,
    pub ss0:          u16,
    _pad0:            u16,
    pub eip:          u32,
    pub eflags:       u32,
    pub eax:          u32,
    pub ecx:          u32,
    pub edx:          u32,
    pub ebx:          u32,
    pub esp:          u32,
    pub ebp:          u32,
    pub esi:          u32,
    pub edi:          u32,
    pub es:           u16,
    _pad1:            u16,
    pub cs:           u16,
    _pad2:            u16,
    pub ss:           u16,
    _pad3:            u16,
    pub ds:           u16,
    _pad4:            u16,
    pub fs:           u16,
    _pad5:            u16,
    pub gs:           u16,
    _pad6:            u16,
    pub ldt:          u32,
    pub trace_bitmap: u32,
    pub i387:         [u8; 108],
}

#[repr(C)]
pub struct LdtEntry {
    pub base:  u32,
    pub limit: u32,
}

#[repr(C)]
pub struct File {
    pub f_count: u32,
    // outros campos omitidos
}

#[repr(C)]
pub struct Inode {
    pub i_count: u32,
    // outros campos omitidos
}

#[repr(C)]
pub struct TaskStruct {
    pub state:      i32,
    pub pid:        i32,
    pub father:     i32,
    pub counter:    i32,
    pub priority:   i32,
    pub signal:     u32,
    pub alarm:      u32,
    pub leader:     i32,
    pub utime:      u64,
    pub stime:      u64,
    pub cutime:     u64,
    pub cstime:     u64,
    pub start_time: u64,
    pub tss:        Tss,
    pub ldt:        [LdtEntry; 3],
    pub filp:       [*mut File; NR_OPEN],
    pub pwd:        *mut Inode,
    pub root:       *mut Inode,
}

// ─── Estado global (unsafe — espelhando o comportamento do kernel) ────────────

static mut LAST_PID: i32 = 0;
static mut TASK: [*mut TaskStruct; NR_TASKS] = [ptr::null_mut(); NR_TASKS];
static mut JIFFIES: u64 = 0;

// Funções externas (implementadas em assembly ou em outros módulos)
extern "C" {
    fn write_verify(address: u32);
    fn get_free_page() -> *mut TaskStruct;
    fn free_page(addr: u64);
    fn copy_page_tables(old_base: u64, new_base: u64, limit: u64) -> i32;
    fn free_page_tables(base: u64, limit: u64) -> i32;
    fn get_base(ldt: &LdtEntry) -> u64;
    fn get_limit(selector: u16) -> u64;
    fn set_base(ldt: &mut LdtEntry, base: u64);
    fn set_tss_desc(gdt_entry: *mut u8, tss: &Tss);
    fn set_ldt_desc(gdt_entry: *mut u8, ldt: &[LdtEntry; 3]);
    fn panic(msg: &str) -> !;
    static mut last_task_used_math: *mut TaskStruct;
    static mut current: *mut TaskStruct;
    static mut gdt: *mut u8;
}

// Constantes de descritores GDT
const FIRST_TSS_ENTRY: usize = 4;
const FIRST_LDT_ENTRY: usize = 5;

// ─── verify_area ─────────────────────────────────────────────────────────────

/// Verifica e garante que uma região de memória é gravável,
/// expandindo o endereço para o início da página correspondente.
pub unsafe fn verify_area(addr: *mut u8, size: usize) {
    let mut start = addr as u64;
    let size = size as u64 + (start & 0xfff);
    start &= 0xffff_f000;
    start += get_base(&(*current).ldt[2]);

    let mut remaining = size as i64;
    while remaining > 0 {
        write_verify(start as u32);
        start += 4096;
        remaining -= 4096;
    }
}

// ─── copy_mem ────────────────────────────────────────────────────────────────

/// Copia o espaço de memória do processo atual para o novo processo filho,
/// configurando os descritores LDT e as tabelas de páginas.
pub unsafe fn copy_mem(nr: i32, p: *mut TaskStruct) -> Result<(), i32> {
    let code_limit = get_limit(0x0f);
    let data_limit = get_limit(0x17);
    let old_code_base = get_base(&(*current).ldt[1]);
    let old_data_base = get_base(&(*current).ldt[2]);

    if old_data_base != old_code_base {
        panic("Segmentos de código e dados separados não são suportados");
    }
    if data_limit < code_limit {
        panic("Limite de dados inválido");
    }

    // Cada processo ocupa um slot de 64 MB (0x4000000 bytes)
    let new_base = (nr as u64) * 0x0400_0000;

    set_base(&mut (*p).ldt[1], new_base); // segmento de código
    set_base(&mut (*p).ldt[2], new_base); // segmento de dados

    if copy_page_tables(old_data_base, new_base, data_limit) != 0 {
        free_page_tables(new_base, data_limit);
        return Err(-ENOMEM);
    }

    Ok(())
}

// ─── copy_process ────────────────────────────────────────────────────────────

/// Rotina principal do fork. Copia as informações do processo do sistema
/// (task[nr]) e configura os registradores necessários.
/// Também copia integralmente o segmento de dados.
#[allow(clippy::too_many_arguments)]
pub unsafe fn copy_process(
    nr: i32,
    ebp: u32, edi: u32, esi: u32, gs: u32,
    _none: u32,
    ebx: u32, ecx: u32, edx: u32,
    fs: u32, es: u32, ds: u32,
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32,
) -> i32 {
    // Aloca uma página para a estrutura do novo processo
    let p = get_free_page();
    if p.is_null() {
        return -EAGAIN;
    }

    // Copia todos os campos do processo atual para o filho
    // NOTA: não copia a pilha do supervisor
    ptr::write(p, ptr::read(current));

    let task = &mut *p;

    task.state      = 0; // TASK_RUNNING
    task.pid        = LAST_PID;
    task.father     = (*current).pid;
    task.counter    = task.priority;
    task.signal     = 0;
    task.alarm      = 0;
    task.leader     = 0; // liderança de processo não é herdada
    task.utime      = 0;
    task.stime      = 0;
    task.cutime     = 0;
    task.cstime     = 0;
    task.start_time = JIFFIES;

    // Configura o TSS (Task State Segment) do processo filho
    task.tss.back_link    = 0;
    task.tss.esp0         = (PAGE_SIZE + p as usize) as u32; // topo da pilha do kernel
    task.tss.ss0          = 0x10;
    task.tss.eip          = eip;
    task.tss.eflags       = eflags;
    task.tss.eax          = 0; // fork() retorna 0 no filho
    task.tss.ecx          = ecx;
    task.tss.edx          = edx;
    task.tss.ebx          = ebx;
    task.tss.esp          = esp;
    task.tss.ebp          = ebp;
    task.tss.esi          = esi;
    task.tss.edi          = edi;
    task.tss.es           = (es  & 0xffff) as u16;
    task.tss.cs           = (cs  & 0xffff) as u16;
    task.tss.ss           = (ss  & 0xffff) as u16;
    task.tss.ds           = (ds  & 0xffff) as u16;
    task.tss.fs           = (fs  & 0xffff) as u16;
    task.tss.gs           = (gs  & 0xffff) as u16;
    task.tss.ldt          = ldt_selector(nr);
    task.tss.trace_bitmap = 0x8000_0000;

    // Salva o estado da FPU se o processo atual estava usando
    if last_task_used_math == current {
        core::arch::asm!("fnsave {0}", out(mem) task.tss.i387);
    }

    // Copia as tabelas de páginas de memória
    if copy_mem(nr, p).is_err() {
        free_page(p as u64);
        return -EAGAIN;
    }

    // Incrementa o contador de referências dos arquivos abertos
    for i in 0..NR_OPEN {
        let f = task.filp[i];
        if !f.is_null() {
            (*f).f_count += 1;
        }
    }

    // Incrementa o contador de referências dos inodes de pwd e root
    if !(*current).pwd.is_null() {
        (*(*current).pwd).i_count += 1;
    }
    if !(*current).root.is_null() {
        (*(*current).root).i_count += 1;
    }

    // Registra o TSS e LDT na GDT global
    set_tss_desc(gdt.add((nr as usize) * 2 + FIRST_TSS_ENTRY), &task.tss);
    set_ldt_desc(gdt.add((nr as usize) * 2 + FIRST_LDT_ENTRY), &task.ldt);

    // Por último, registra o processo na tabela de tarefas
    TASK[nr as usize] = p;

    LAST_PID
}

// ─── find_empty_process ──────────────────────────────────────────────────────

/// Encontra um slot vazio na tabela de processos e gera um novo PID único.
/// Retorna o índice do slot ou -EAGAIN se não houver espaço.
pub unsafe fn find_empty_process() -> i32 {
    loop {
        LAST_PID += 1;
        if LAST_PID < 0 {
            LAST_PID = 1;
        }

        // Verifica se o PID gerado já está em uso
        let pid_in_use = (0..NR_TASKS).any(|i| {
            !TASK[i].is_null() && (*TASK[i]).pid == LAST_PID
        });

        if !pid_in_use {
            break;
        }
    }

    // Encontra o primeiro slot vazio (índice 0 é reservado para o processo idle)
    for i in 1..NR_TASKS {
        if TASK[i].is_null() {
            return i as i32;
        }
    }

    -EAGAIN
}

// ─── Helpers internos ────────────────────────────────────────────────────────

/// Calcula o seletor LDT para o processo de índice `nr` na GDT.
#[inline]
fn ldt_selector(nr: i32) -> u32 {
    // _LDT(nr) = (nr << 4) | 0b100 | 0  →  entrada de LDT no nível 0
    ((nr as u32) << 4) | 0x4
}
