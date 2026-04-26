# Relix

A minimalist microkernel study inspired by the Linux kernel, built with modern systems programming languages.

## 📌 About the Project

Relix is an **experimental hobby** focused on operating system internals and microkernel architecture. It started as an exercise in rewriting critical kernel components from C to **Zig**, and has since evolved into a robust codebase dominated by **Rust**.

The goal of Relix is to explore how a "minimalist core" can manage hardware resources while maintaining strict memory safety and stability.

## 🛠 Tech Stack

Based on the current repository analysis:

* **Rust (93.8%)**: Handles the core kernel logic, safe memory management, and high-level abstractions.
* **Zig (5.5%)**: Used for the Hardware Abstraction Layer (HAL), boot protocols, and low-level "close-to-metal" efficiency.
* **Assembly**: Initial boot-strapping and CPU-specific instructions.

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
