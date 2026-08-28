# Debugging

Use independent trace classes to keep logs bounded:

```text
--trace-cpu --trace-mmu --trace-bus --trace-io
--trace-irq --trace-scc --trace-prom --trace-file FILE
```

The ring retains the final operations before failure and appends the latest
fault with PC, virtual/physical address, function code, width, direction, PTE,
and bus-error reason. `--break HEX` stops at a firmware address without changing
guest state.
