# PROM POST landmarks

The trace records these observed firmware addresses as progress landmarks:

| PROM PC | Test |
|---|---|
| `0x0fefb104` | bus error |
| `0x0fefb18e` | interrupt |
| `0x0fefb1da` | clock interrupt |
| `0x0fefb344` | MMU valid bit |
| `0x0fefb3c4` | MMU read-only protection |
| `0x0fefb45e` | no spurious parity NMI |
| `0x0fefb50c` | parity NMI |
| `0x0fefb5c8` | RAM sizing |
| `0x0fef581c` | EPROM remapping |
| `0x0fef02b2` | framebuffer enable / main display path |

These are observation points only. The emulator contains no PC-specific bypasses.
