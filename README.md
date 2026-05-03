# VortexShield Kernel

> *"It's not yet surprising, but it is functional."*

A minimalist microkernel built with modern systems programming languages, focused on **security**, **privacy**, and **memory safety**.

---

## 📌 About

VortexShield started as a personal study project to understand operating system internals from the ground up. What began as an exercise in rewriting kernel components evolved into a full microkernel with its own security subsystem, anti-malware layer, and scripting environment.

The project went through three naming stages before its current form:

```
Relix → Stax → VortexShield
```

The kernel was developed on **Fedora Linux** (migrated from Fedora 43 to 44) and is inspired by the philosophy that security should be built into the core — not added on top.

---

## 🛠 Tech Stack

| Language | Role |
|----------|------|
| **Rust** | Core kernel logic, memory safety, scheduling, security modules |
| **Zig** | Hardware Abstraction Layer (HAL), boot protocols, drivers |
| **C** | Low-level kernel base, hardware interfacing |
| **Assembly** | Boot-strapping, CPU-specific instructions, context switching |
| **Lua** | Shell and terminal scripting layer |
| **Leyernet** | Native kernel language — low-level control with beginner-friendly syntax |

---

## 🏗 Architecture

VortexShield follows the **Microkernel** philosophy:

- **Minimalist Core** — only essential services run in supervisor mode
- **Isolation** — drivers and system services run in user space, preventing total system failure on crash
- **Multi-arch** — active porting to `x86_64`, `ARM`, and `RISC-V`

### Directory Structure

```
VortexShield/
├── kernel/
│   ├── boot.C                  # Boot stage
│   ├── boot_face.rs            # Boot interface
│   ├── fork.rs                 # Process management
│   ├── keyboard.zig            # Keyboard driver
│   ├── serial.zig              # Serial communication
│   ├── security.rs             # Kernel-level security
│   ├── time.rs                 # Time management
│   ├── wake_up_C_arm.asm       # ARM wake-up routine
│   └── kitDEV/                 # Dev toolkit
├── NoMalware/
│   ├── ResistMalware.zig       # Active malware resistance
│   ├── buffer_over_malware.C   # Buffer overflow counter-attack
│   └── scamMALWARE.config.rs  # Scam/malware detection config
├── drives_PC/
│   ├── driver_hardware.zig     # Hardware drivers
│   ├── drives_library.C        # Driver library
│   └── offline_library.C       # Offline support
├── tools/
│   ├── main.rs                 # Entry point
│   ├── security_asm.zig        # Security assembly tools
│   ├── detector.rs             # Threat detection
│   └── ...
└── leyernet_programming_language/
    └── ...                     # Leyernet compiler and docs
```

---

## 🔒 Security Model

VortexShield treats security as architecture, not an afterthought:

- **`ResistDDoS.zig`** — DDoS resistance at kernel level, before packets reach userspace
- **`buffer_over_malware.C`** — detects malware and uses buffer overflow against it
- **`ResistMalware.zig`** — active malware resistance layer
- **`security.rs`** — core kernel security enforcement
- **`scamMALWARE.config.rs`** — behavioral scam and malware detection

---

## 🌐 Leyernet Language

Leyernet is a programming language developed alongside VortexShield. Its goal is to give beginners the same low-level control as Assembly, with a more approachable syntax.

```
Assembly  →  total control, hostile syntax
C         →  close to metal, abstracts too much
Leyernet  →  control of Assembly, readable by beginners
```

---

## 🚀 Status: Work in Progress

| Component | Status |
|-----------|--------|
| Kernel core | ✅ Functional |
| NoMalware subsystem | ✅ Active |
| x86_64 support | ✅ Primary target |
| ARM port | 🔧 In progress |
| RISC-V port | 🔧 In progress |
| Memory allocator | 🔧 In progress |
| Task scheduling | 🔧 In progress |
| IPC mechanism | 📋 Planned |
| Leyernet compiler | 🔧 In progress |

---

## 📜 License

This project is licensed under the **MIT License** — see the `LICENSE` file for details.

---

*Developed by **DCrust369** — distributed on the Onion network.*  
*No warranties provided. Just pure engineering curiosity.*
