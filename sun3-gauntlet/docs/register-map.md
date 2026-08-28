# Register map

## Control space (MC68020 FC=3)

| Address top nibble | Device | Width |
|---:|---|---|
| `0x0` | 32-byte IDPROM | byte |
| `0x1` | page map | long |
| `0x2` | segment map | byte |
| `0x3` | context | byte |
| `0x4` | system enable | byte |
| `0x5` | user DVMA enable | byte |
| `0x6` | bus-error cause | byte |
| `0x7` | diagnostic LEDs | byte |
| `0x8` | cache tags | long |
| `0x9` | cache data | long |
| `0xa` | cache flush | operation |
| `0xb` | block copy | operation |
| `0xf` | UART bypass | byte |

## Type-1 physical I/O

| Address | Device |
|---:|---|
| `0x000000` | keyboard/mouse Z8530 |
| `0x020000` | serial Z8530 |
| `0x040000` | 2 KiB NVRAM |
| `0x060000` | clock/RTC |
| `0x080000` | parity registers |
| `0x0a0000` | interrupt control |
| `0x100000` | boot PROM |
| `0x120000` | LANCE registers |
| `0x140000` | NCR5380 |
| `0x140010` | AM9516 UDC |
| `0x140018` | SCSI control |
| `0x1e0000` | ECC registers (absent on parity-memory 3/60) |
