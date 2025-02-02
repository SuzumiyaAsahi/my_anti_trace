#![no_std]
#![no_main]

use core::ffi::c_long;

use aya_ebpf::{helpers::bpf_get_current_task, macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;
mod vmlinux;
use vmlinux::task_struct;
mod my_core;
use my_anti_trace_common::target_pid;

mod my_error;
use my_error::BpfError;

mod my_ebpf_func;
use my_ebpf_func::send_signal;

#[tracepoint]
pub fn my_anti_trace(ctx: TracePointContext) -> i32 {
    bpf_dos(ctx).unwrap_or_else(|ret| ret as i32)
}

fn bpf_dos(ctx: TracePointContext) -> Result<i32, c_long> {
    // if target_pid is 0 then we target all pids
    if unsafe { target_pid != 0 } {
        let task = unsafe { bpf_get_current_task() } as *const task_struct;
        let ppid = BPF_CORE_READ!(task, real_parent, tgid)?;
        if unsafe { ppid as u64 != target_pid } {
            return Ok(0);
        }
    }

    if let Err(ret) = send_signal(9) {
        match ret {
            BpfError::Busy => {
                info!(&ctx, "EBUSY if work queue under nmi is full.");
            }
            BpfError::Again => {
                info!(&ctx, "EAGAIN if bpf program can try again.");
            }
            BpfError::Invalid => {
                info!(&ctx, "EINVAL if sig is invalid.");
            }
            BpfError::Permission => {
                info!(&ctx, "EPERM if no permission to send the sig.");
            }
            _ => {
                info!(&ctx, "Unknown error, it is impossible.");
            }
        }
    }

    info!(&ctx, "tiger! tiger! tiger!");

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
