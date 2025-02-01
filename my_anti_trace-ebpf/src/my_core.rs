/// Recursive unrolled BPF_CORE_READ macro
#[macro_export]
macro_rules! BPF_CORE_READ {
    // access to single field
    ($ptr:expr, $field:ident) => {{
        unsafe { ::aya_ebpf::helpers::bpf_probe_read_kernel(&(*$ptr).$field) }
    }};

    // Recursive access to multi-level fields
    ($ptr:expr, $field:ident, $($rest:ident),+) => {{
        let inner_ptr = unsafe { ::aya_ebpf::helpers::bpf_probe_read_kernel(&(*$ptr).$field) };
        if let Ok(p) = inner_ptr {
            BPF_CORE_READ!(p, $($rest),+)
        } else {
            Err(-1)
        }
    }};
}
