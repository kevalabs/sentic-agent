# Operational Debugging

This guide focuses on "how to inspect" the system using high-leverage diagnostic commands.

## Common Diagnostic Commands

### Verify eBPF Probes
Verify that the `sentic-ebpf` probes are correctly loaded into the kernel:
```bash
sudo bpftool prog show
```

### Inspect BPF Maps
Directly inspect raw telemetry in the BPF maps before it reaches the `sentic-agent` user-space daemon:
```bash
sudo bpftool map dump id <ID>
```

### Verify BTF Alignment
Export the kernel's BTF data to verify memory alignment with the `SenticEvent` struct defined in `sentic-common`:
```bash
sudo bpftool btf dump file /sys/kernel/btf/vmlinux format c
```
