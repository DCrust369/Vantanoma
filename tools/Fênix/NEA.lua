-- ==========================================================
-- VortexShield-kernel: Sistema de Scripts de Segurança
-- Copyright (c) 2026 DCrust
-- Licenses: GPL-3.0 / Apache-2.0 / MIT
-- ==========================================================
-- with IA ok
-- Tabela Global de Ferramentas
Vortex = {
    version = "1.0.0-alpha",
    author = "DCrust"
}

-- 1. UTILITÁRIOS DE MEMÓRIA (Pentesting)
Vortex.Mem = {
    scan = function(start, size, pattern)
        print("[*] Scanning memory for pattern: " .. pattern)
        -- Aqui chama a função que você exportou do Rust/C
        local result = vortex_core.mem_search(start, size, pattern)
        return result
    end,
    
    dump = function(addr, len)
        print("[*] Dumping " .. len .. " bytes from " .. string.format("0x%X", addr))
        vortex_core.raw_dump(addr, len)
    end
}

-- 2. FERRAMENTAS DE REDE (Injeção)
Vortex.Net = {
    flood = function(ip, port)
        print("[!] Warning: Starting SYN Flood test on " .. ip .. ":" .. port)
        for i=1, 1000 do
            vortex_core.send_syn(ip, port)
        end
    end
}

-- 3. SISTEMA E STEALTH (Rootkit Analysis)
Vortex.Sys = {
    hide_me = function()
        print("[+] Enabling Stealth Mode...")
        vortex_core.set_visibility(false)
    end,
    
    list_procs = function()
        local procs = vortex_core.get_process_list()
        for _, p in ipairs(procs) do
            print(string.format("PID: %d | Name: %s", p.id, p.name))
        end
    end
}
