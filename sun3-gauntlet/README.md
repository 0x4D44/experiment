# Sun 3/60 Emulator (Rust)

A correctness-first Sun Microsystems Sun 3/60 emulator which runs the genuine
64 KiB Sun 3/60 Boot PROM v3.0.1. The machine core is headless and deterministic;
serial port A is the initial console.

This branch is an active gauntlet implementation. Firmware is **not**
redistributed. `scripts/bootstrap.sh` imports the public OldSilicon archive,
validates SHA-1 `6e48414ce2139282e69f57612b20f7d5c475e74c` and SHA-256
`b562aa5d7bc51eed732fbafde1fd6ea1340977d2b04fb826201c079f699212c6`, and
pins the reusable MC68020/MC68881 core to an exact commit.

## Build and run

```bash
./scripts/bootstrap.sh
cargo build --release
./target/release/sun3 \
  --rom firmware/sun3-60-3.0.1.bin \
  --ram 24M \
  --console stdio \
  --interactive
```

For a deterministic acceptance run:

```bash
./target/release/sun3 \
  --rom firmware/sun3-60-3.0.1.bin \
  --ram 24M \
  --console capture \
  --script-cr \
  --require-monitor \
  --console-log evidence/prom-console.bin \
  --trace-prom --trace-mmu --trace-io --trace-irq --trace-scc \
  --trace-file evidence/prom-trace.log
```

`sun3 --help` documents RAM, IDPROM, breakpoints, deterministic bounds, serial
input, and individual trace classes.

## Implemented architecture

- Motorola MC68020 interpreter and MC68881 state/operations
- Sun-3 function-code control space and reset boot-state mapping
- eight contexts, 16,384 segment entries, 256 PMEGs, and 4,096 PTEs
- permission faults, accessed/modified bits, physical address-space types
- 4–24 MiB RAM sizing, parity registers, and real 68020 bus-error delivery
- deterministic IDPROM, interrupt controller, 100 Hz virtual clock, and RTC IRQ
- two programmable Z8530 SCCs with serial console receive/transmit and IRQs
- bwtwo VRAM mapping, PROM mapping, NVRAM, and system registers
- probe-safe NCR5380, AM9516, LANCE, and SCSI-control register models
- bounded ring tracing and genuine PROM POST landmarks

No banner, POST result, or monitor prompt is hard-coded. Unknown or invalid accesses
fault rather than silently succeeding.
