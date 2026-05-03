# VoidShield-Kernel

A minimalist microkernel study inspired by the Linux kernel, built with modern systems programming languages.

## 📌 About the Project

Relix is an **experimental hobby** focused on operating system internals and microkernel architecture. It started as an exercise in rewriting critical kernel components from C to **Zig**, and has since evolved into a robust codebase dominated by **Rust**.

The goal of VoidShield-Kernel is to explore how a "minimalist core" can manage hardware resources while maintaining strict memory safety and stability.

## 🛠 Tech Stack

Based on the current repository analysis:

* **Rust (93.8%)**: Handles the core kernel logic, safe memory management, and high-level abstractions.
* **Zig (25.3%)**: Used for the Hardware Abstraction Layer (HAL), boot protocols, and low-level "close-to-metal" efficiency.
* **Assembly**: Initial boot-strapping and CPU-specific instructions.

## 💻 operational system
* I developed this kernel like this, I wanted to create a linux distro based on mint linux on a windows in 2025 but we have a problem the organization on my windows 11 NT so I was testing distros and * **exploring my first distro was parrot security but I didn't like it so I went to arch, nobara to void and pop!_OS and I liked fedora - Native SELinux - Super updated kernel - pure GNOME so I used * * * **fedora 43 and migrated to fedora 44 the first version of this kernel was called Relix then it went to stax then VortexShield Kernel

## 🏗 Architecture

Relix follows the **Microkernel** philosophy:
* **Minimalist Core**: Only essential services run in supervisor mode.
* **Isolation**: Drivers and system services are designed to run in user space to prevent total system failure in case of a crash.

## 🚀 Status: Work in Progress

This is currently a **personal study project**. 
- **Current focus**: Memory allocation and Task Scheduling.
- **Next steps**: Implementing a stable Inter-Process Communication (IPC) mechanism.

> "It's not yet surprising, but it is functional."

## 📜 License

This project is licensed under the **MIT License** - see the LICENSE file for details.

---
*Developed as a hobby by **DCrust369**. No warranties provided, just pure engineering curiosity.*
