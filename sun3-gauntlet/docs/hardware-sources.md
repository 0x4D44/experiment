# Hardware sources

Implementation behavior is cross-checked against:

- *Sun-3 Architecture Manual*, Version 2.0 preliminary, 16 May 1985
- Motorola MC68020 and MC68881 programming architecture
- Zilog Z8530 programming model
- MAME `src/mame/sun/sun3.cpp` as non-authoritative prior art
- `54weasels/sun3_60`, including schematics/PAL work and a minimal physical board
- OldSilicon Sun Boot ROM archive, Sun 3/60 PROM v3.0.1

The exact CPU core commit and firmware hashes are emitted by bootstrap and CI.
Disagreements are resolved in favor of primary documentation and observed PROM
behavior rather than copied implementation quirks.
