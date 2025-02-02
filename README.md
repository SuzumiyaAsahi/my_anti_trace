# my_anti_trace

## Build & Run

Use `cargo build`, `cargo check`, etc. as normal. Run your program with:

```shell
cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

Cargo build scripts are used to automatically build the eBPF correctly and include it in the
program.

## print log to txt

```shell
cargo build > a.txt 2>&1
```

## get EBUSY error code value

```shell
cat /usr/include/asm-generic/errno-base.h | grep EBUSY
```
