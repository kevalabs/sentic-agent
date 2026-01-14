# Engineering Deep-Dives ⚙️

This directory serves as the technical "Source of Truth" for the Sentic Agent. Unlike user manuals, these documents explain the internal architecture, core constraints, and the complex trade-offs made during development.

## 🗺️ Implementation Map

| Document | Reason for Existence | Target Audience |
| :--- | :--- | :--- |
| **[high-level-overview.md](./high-level-overview.md)** | **High-Level Overview.** Explains the high level overview of the Sentic Agent. | All Contributors |
| **[cgroup-v2-identity.md](./cgroup-v2-identity.md)** | **Identity Foundation.** Explains why we require Kernel 5.4+ and how we use the Unified Hierarchy for stable process attribution. | SREs / Core Devs |
| **[ebpf-pipeline.md](./ebpf-pipeline.md)** | **Data Flow.** Details the high-performance path from BPF maps in the kernel to the Rust userspace agent. | Rust Developers |
| **[debugging-internals.md](./debugging-internals.md)** | **Troubleshooting the Agent.** How to debug BPF verifier errors and map corruption without crashing the host kernel. | Core Maintainers |

---

## 🏗️ Architectural Pillars

1. **Kernel-First Identity:** We do not trust PIDs. Every event is anchored to a 64-bit Cgroup ID to ensure visibility persists across process restarts.
2. **Rust Safety:** By using the `aya` library, we ensure that our userspace logic is memory-safe while maintaining zero-cost abstractions for kernel interaction.
3. **Non-Intrusive Monitoring:** We prioritize `fentry` and `LSM` hooks over legacy `kprobes` to minimize performance overhead in high-stakes environments (e.g., Banking).

## ✍️ Contribution Guidelines
- **Draft First:** Major changes should start as an [ADR](../adr/) before being formalized here.
- **Link to Research:** If a decision was based on a lab experiment, link to the corresponding entry in [docs/research/](../research/).
- **Update on Refactor:** If the internal logic of a subsystem changes, this documentation must be updated in the same Pull Request.
