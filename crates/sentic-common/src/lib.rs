#![no_std]

/// The maximum length of a process name or path we will capture.
/// Keeping this small (16-64 bytes) is critical for eBPF stack limits (512 bytes).
pub const MAX_COMM_LEN: usize = 16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessEvent {
    pub pid: u32,                  // Process ID
    pub ppid: u32,                 // Parent Process ID
    pub uid: u32,                  // User ID
    pub comm: [u8; MAX_COMM_LEN],  // Process name (truncated)
    pub exit_code: i32,            // Only used for exit events
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum EventType {
    ProcessStart,
    ProcessExit,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SenticEvent {
    pub event_type: EventType,
    pub data: ProcessEvent,
}