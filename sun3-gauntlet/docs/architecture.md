# Architecture

The reusable MC68020 core reaches the machine solely through `AddressBus`. A
small pinned patch adds a function-code callback so Sun control-space MOVES,
program fetches, data cycles, exception stacking, and vector fetches retain
their architectural FC values. The machine bus then applies boot-state mapping,
Sun-3 MMU translation, protection/statistics, physical-space selection, and
device dispatch.

The CPU core contains no Sun address constants. Devices do not index RAM
behind the MMU; future DVMA uses the same translation path. Timers advance from
guest cycles, making headless POST runs deterministic.
