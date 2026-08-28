# Memory map

The CPU presents a 256 MiB virtual address per context. Sun-3 MMU PTE type 0
selects main-memory space and type 1 selects onboard I/O. Type 2 and 3 select
16-bit and 32-bit VME spaces and currently produce documented timeout bus
errors because no VME board is installed.

Type-0 RAM begins at physical zero and ends at the configured installed size.
The onboard monochrome framebuffer is at `0xff000000` with 256 KiB of VRAM.
Type-1 PROM is at `0x00100000`; supervisor-program fetches are also overlaid
from the same 64 KiB image while system-enable `EN.BOOT-` is clear.
