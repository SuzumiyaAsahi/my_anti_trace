#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::bpf_get_current_pid_tgid, macros::tracepoint, programs::TracePointContext,
};
use aya_log_ebpf::info;
use my_anti_trace_common::target_pid;
mod vmlinux;
use vmlinux::task_struct;

#[tracepoint]
pub fn my_anti_trace(ctx: TracePointContext) -> u32 {
    match try_my_anti_trace(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_my_anti_trace(ctx: TracePointContext) -> Result<u32, u32> {
    let mut ret = 0;
    // let pid_tgid =
    info!(&ctx, "tracepoint sys_enter_ptrace called");
    unsafe {
        info!(&ctx, "target_pid is {}", target_pid);
    };
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
