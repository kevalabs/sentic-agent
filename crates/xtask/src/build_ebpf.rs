use anyhow::{Context, Result};
use std::process::Command;

pub fn build_ebpf(release: bool) -> Result<()> {
    let mut args = vec![
        "build",
        "--package",
        "sentic-ebpf",
        "--target",
        "bpfel-unknown-none",
        "-Z",
        "build-std=core",
    ];

    if release {
        args.push("--release");
    }

    let status = Command::new("cargo")
        .args(&args)
        .status()
        .context("Failed to run cargo build for sentic-ebpf")?;

    if !status.success() {
        anyhow::bail!("Failed to build eBPF component");
    }

    Ok(())
}
