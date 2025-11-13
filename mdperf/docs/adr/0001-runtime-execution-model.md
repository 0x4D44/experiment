# ADR 0001: Runtime Execution Model
- **Date:** 2025-11-13
- **Status:** Accepted

## Context
The benchmarking tool must coordinate CPU-bound loops, async network I/O, disk workloads, and a TUI without oversubscribing host resources. The HLD called out a mix of std threads for CPU work and tokio for async tasks, but never codified how modules plug into a shared executor or how we degrade when async runtimes are unavailable (e.g., minimal container images).

## Decision
1. Adopt a tokio multi-thread runtime as the default executor for asynchronous workloads (network, async disk, telemetry fetchers). The runtime is initialized once during startup and handed to the orchestrator.
2. Provide a "blocking" fallback mode that skips initializing tokio and limits the orchestrator to synchronous modules—used for constrained hosts or debugging.
3. Expose the runtime choice through both configuration (`general.runtime`) and CLI (`--runtime`).
4. Require modules to declare whether they need async support; the orchestrator will reject async modules when running in blocking mode.

## Rationale
- tokio offers proven async performance, timer facilities, and ecosystem crates already referenced in later stages (reqwest, hyper, etc.).
- Isolating runtime initialization lets us validate the executor health during Stage 0 and catch misconfiguration before workloads run.
- Explicit fallback supports air-gapped environments where threading policies are tightly controlled.

## Consequences
- Stage 1+ modules can assume an async executor exists when `runtime == tokio`, reducing boilerplate per module.
- Blocking mode imposes feature constraints that must be surfaced as warnings (already emitted by the Stage 0 orchestrator).
- Future CI plans must test both runtime strategies to prevent regressions.

## Follow-Up Actions
- Extend the orchestrator API to tag modules as `Async`/`Blocking` and validate combinations.
- Document operational guidance for when to choose each runtime.
- Capture runtime metrics (thread count, queue depth) once modules execute meaningful workloads.
