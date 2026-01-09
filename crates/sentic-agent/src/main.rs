use anyhow::Context;
use aya::{
    maps::PerfEventArray,
    programs::TracePoint,
    util::online_cpus,
    Ebpf,
};
use bytes::BytesMut;
use sentic_common::SenticEvent;
use std::{path::Path, convert::TryFrom, convert::TryInto, time::Duration};
use tokio::{signal, task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let possible_paths = [
        "../../target/bpfel-unknown-none/debug/sentic",
        "target/bpfel-unknown-none/debug/sentic",
        "../target/bpfel-unknown-none/debug/sentic",
        "../../target/bpfel-unknown-none/release/sentic",
        "target/bpfel-unknown-none/release/sentic",
        "../target/bpfel-unknown-none/release/sentic",
    ];
    
    let path = possible_paths.iter()
        .map(Path::new)
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("eBPF binary 'sentic' not found. Did you run 'cargo xtask build-ebpf'?"))?;

    println!("Loading eBPF program from: {:?}", path);

    let mut bpf = Ebpf::load_file(path)?;

    if let Err(e) = aya_log::EbpfLogger::init(&mut bpf) {
        println!("Logger init failed (might not be enabled in eBPF): {}", e);
    }

    let program: &mut TracePoint = bpf.program_mut("sched_process_exec")
        .context("program sched_process_exec not found")?
        .try_into()?;
    
    program.load()?;
    program.attach("sched", "sched_process_exec")?;
    
    println!("Attached to tracepoint: sched:sched_process_exec");

    let mut events = PerfEventArray::try_from(bpf.take_map("EVENTS").context("map EVENTS not found")?)?;
    
    // Spawn a blocking task to handle event reading loop
    task::spawn_blocking(move || {
        let cpus = match online_cpus() {
            Ok(cpus) => cpus,
            Err(e) => {
                println!("Failed to get online cpus: {:?}", e);
                return;
            }
        };

        let mut buffers = Vec::new();
        for cpu_id in cpus {
            match events.open(cpu_id, None) {
                Ok(buf) => buffers.push(buf),
                Err(e) => println!("Failed to open buffer for CPU {}: {}", cpu_id, e),
            }
        }

        let mut byte_buffers = (0..buffers.len())
            .map(|_| (0..10).map(|_| BytesMut::with_capacity(1024)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        loop {
            let mut got_events = false;
            for (i, buf) in buffers.iter_mut().enumerate() {
                let byte_bufs = &mut byte_buffers[i];
                
                // read_events returns Result<Buffers> with count
                match buf.read_events(byte_bufs) {
                    Ok(events) => {
                        if events.read > 0 {
                            got_events = true;
                            for buf in byte_bufs.iter().take(events.read) {
                                if buf.len() < std::mem::size_of::<SenticEvent>() {
                                    continue;
                                }
                                let ptr = buf.as_ptr() as *const SenticEvent;
                                let event = unsafe { ptr.read_unaligned() };
                                
                                let comm = match std::str::from_utf8(&event.data.comm) {
                                    Ok(s) => s.trim_matches(char::from(0)),
                                    Err(_) => "<invalid_utf8>",
                                };
                                
                                println!("New Process: PID: {}, UID: {}, Comm: {}", event.data.pid, event.data.uid, comm);
                            }
                        }
                    }
                    Err(_e) => {
                         // println!("Error reading events: {}", e); 
                    }
                }
            }
            
            if !got_events {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    });

    println!("Waiting for events... (Press Ctrl+C to exit)");
    signal::ctrl_c().await?;
    println!("Exiting...");

    Ok(())
}
