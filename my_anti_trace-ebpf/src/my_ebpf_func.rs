#![no_std]
use aya_ebpf::helpers::bpf_send_signal;

use crate::my_error::BpfError;

pub fn send_signal(sig: u32) -> Result<(), BpfError> {
    let ret = unsafe { bpf_send_signal(sig) } as i32;
    if ret == 0 {
        Ok(())
    } else {
        Err(ret.into())
    }
}
