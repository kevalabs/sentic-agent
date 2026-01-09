mod build_ebpf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    BuildEbpf {
        #[clap(long)]
        release: bool,
        #[clap(long, default_value = "bpfel-unknown-none")]
        target: String,
    },
    /// Build the userspace agent (dummy command for now)
    Build,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        Command::BuildEbpf { release, .. } => {
            build_ebpf::build_ebpf(release)?;
        }
        Command::Build => {
            println!("Build agent logic to be implemented");
        }
    }

    Ok(())
}
