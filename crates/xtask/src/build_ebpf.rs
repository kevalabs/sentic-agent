use anyhow::{Context, Result};
use std::process::Command;

pub fn build_ebpf() -> Result<()> {
    println!("🚀 Building Sentic eBPF probes...");

    // We use the 'cargo build' command but with specific eBPF flags
    let status = Command::new("cargo")
        .args([
            "+nightly",
            "build",
            "--package",
            "sentic-ebpf",
            "--target",
            "bpfel-unknown-none",
            "-Z",
            "build-std=core",
            "--release",
        ])
        .status()
        .context("Failed to run cargo build for eBPF")?;

    if !status.success() {
        anyhow::bail!("eBPF build failed with status: {}", status);
    }

    println!("✅ eBPF bytecode generated successfully.");
    Ok(())
}
