# Sun-3 MMU

Virtual address bits 27:17 select one of 2,048 segments in the current 3-bit
context. The 8-bit segment entry selects a PMEG; bits 16:13 select one of its
16 PTEs. PTE bits implement valid, write, supervisor-only, cache-control,
physical-space type, accessed, modified, and a 19-bit physical page number.
Pages are 8 KiB and segments are 128 KiB.

MC68020 function code 3 addresses control space. Top address nibbles 0–7 select
IDPROM, page map, segment map, context, system enable, DVMA enable, bus-error,
and diagnostic registers. During reset boot state, supervisor-program cycles
are redirected to PROM independently of MMU contents; data cycles still pass
through the MMU.
