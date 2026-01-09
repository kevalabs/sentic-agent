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
    BuildEbpf,
    /// Build the userspace agent (dummy command for now)
    Build,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        Command::BuildEbpf => {
            build_ebpf::build_ebpf()?;
        }
        Command::Build => {
            println!("✅ Build agent logic to be implemented");
        }
    }

    Ok(())
}
