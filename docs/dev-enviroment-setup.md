# Developer Environment Setup Guide

## A. macOS: Lima (Ubuntu) + VS Code Remote SSH

Lima provides a lightweight Linux VM on macOS that feels like a native container.

### 1. install lima

```bash
brew install lima
```
### 2. Create a Sentic VM: Use the default Ubuntu template, ensuring the kernel is 5.15+ for optimal eBPF/LSM support.
```bash
limactl start --name=sentic template://ubuntu # any distro can be selected from https://lima-vm.io/docs/templates

```
### 3. Environment Initialization (Lima/Fedora)
After the initial OS installation, you must synchronize the package manager and ensure essential network utilities are present. This prevents VS Code Remote-SSH installation failures.
```bash

# 1. Update package metadata and upgrade all system packages
# This ensures you are running the latest kernel patches.
sudo dnf upgrade --refresh -y

# 2. Install essential utilities if missing
# 'which' and 'wget' are required for VS Code Server to self-install.
sudo dnf install -y wget which tar

```

### 4. VS Code Integration:
- Install the "Remote - SSH" extension in VS Code.
- Add your Lima VM to ~/.ssh/config:
```text
Host sentic-dev
  HostName 127.0.0.1
  User your-user
  Port 60022 # Check 'limactl list' for the port
  IdentityFile ~/.lima/_config/user

```
- Connect VS Code to sentic-dev

## B. Windows: WSL2 (Ubuntu 22.04+)
WSL2 runs a genuine Linux kernel, which is necessary for eBPF development.
### 1. Install WSL:
```powershell
wsl --install -d Ubuntu-22.04
```
### 2. Kernel Update: Ensure you are running the latest WSL kernel to support BPF Type Format (BTF)
```powershell
wsl --update
```
### 3. VS Code Integration:
- Install the "**WSL**" extension.
- Click the green bottom-left corner and select "**Connect to WSL**".

---
## 3. Shared Linux Toolchain Installation
Once inside your Linux environment (Lima or WSL2), run these commands to prepare the Rust eBPF stack:
```bash
# 1. Install Rust and the BPF target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add bpfel-unknown-none

# 2. Install bpf-linker (Required by Aya)
cargo install bpf-linker

# 3. Install aya-tool (For generating kernel bindings) [cite: 207, 328]
cargo install aya-tool

# 4. Install LLVM/Clang (Required for some eBPF compilation) [cite: 317, 327]
sudo apt update && sudo apt install -y clang llvm libelf-dev
```
