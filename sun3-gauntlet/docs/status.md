# Status

The implementation is driven directly by Sun 3/60 Boot PROM v3.0.1. Every CI
run performs format, Clippy, unit/architecture tests, a release build, and a
bounded execution of the genuine firmware. The boot transcript and bounded
trace are uploaded even when firmware exposes the next missing hardware detail.

The decisive milestone is an authentic interactive PROM monitor; intermediate
PCs and diagnostics are evidence for iteration, not completion.
