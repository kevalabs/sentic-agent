#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{map, tracepoint},
    maps::PerfEventArray,
    programs::TracePointContext,
    helpers::{bpf_get_current_pid_tgid, bpf_get_current_comm, bpf_get_current_uid_gid},
};
use sentic_common::{SenticEvent, ProcessEvent, EventType};

#[map]
static EVENTS: PerfEventArray<SenticEvent> = PerfEventArray::new(0);

#[tracepoint]
pub fn sched_process_exec(ctx: TracePointContext) -> u32 {
    match try_sched_process_exec(ctx) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

fn try_sched_process_exec(ctx: TracePointContext) -> Result<u32, i64> {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;
    // Lower 32 bits are kernel TID (which is userspace PID in Linux threads view), 
    // Upper 32 bits are TGID (which is userspace Process ID).
    
    let uid_gid = bpf_get_current_uid_gid();
    let uid = (uid_gid & 0xFFFFFFFF) as u32;

    // aya-ebpf now returns the buffer directly
    let comm = bpf_get_current_comm().map_err(|e| e as i64)?;

    let event = SenticEvent {
        event_type: EventType::ProcessStart,
        data: ProcessEvent {
            pid,
            ppid: 0, 
            uid,
            comm,
            exit_code: 0,
        },
    };

    EVENTS.output(&ctx, &event, 0);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
