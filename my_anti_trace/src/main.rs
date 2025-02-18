use aya::programs::TracePoint;
#[rustfmt::skip]
use log::{debug, warn};
use clap::Parser;
use tokio::signal;

#[derive(Debug, Parser)]
struct TargetPid {
    #[clap(short, long, default_value = "0")]
    pid: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = TargetPid::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.

    let mut ebpf = aya::EbpfLoader::new()
        .set_global("target_pid", &opt.pid, true)
        .load(aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/my_anti_trace"
        )))?;

    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut TracePoint = ebpf.program_mut("my_anti_trace").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_ptrace")?;

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
