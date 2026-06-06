# ternary-temperament

Musical temperament and tuning in ternary — intervals, chords, just intonation, and the mathematics of the triad

## Why This Matters

Musical temperament and tuning in ternary — intervals, chords, just intonation, and the mathematics of the triad

## The Five-Layer Stack

This crate is part of the **Oxide Stack** — a distributed GPU runtime built on five layers:

```
┌─────────────────┐
│  cudaclaw        │  Persistent GPU kernels, warp consensus, SmartCRDT
├─────────────────┤
│  cuda-oxide      │  Flux → MIR → Pliron → NVVM → PTX compiler
├─────────────────┤
│  flux-core       │  Bytecode VM + A2A agent protocol
├─────────────────┤
│  pincher         │  "Vector DB as runtime, LLM as compiler"
├─────────────────┤
│  open-parallel   │  Async runtime (tokio fork)
└─────────────────┘
```

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Design

Every value in this crate follows **ternary algebra** (Z₃):

| Value | Meaning | GPU Analog |
|-------|---------|------------|
| +1 | Positive / Active / Healthy | Warp vote yes |
| 0 | Neutral / Pending / Balanced | Warp vote abstain |
| -1 | Negative / Failed / Overloaded | Warp vote no |

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary LLMs at 60% less power
2. **GPU warp voting** — hardware ballot returns ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity

## Key Types

```rust
pub struct Pitch
pub fn new
pub struct Interval
pub fn new
pub struct Chord
pub fn new
pub fn pitches
pub fn major_triad
pub fn minor_triad
pub fn transpose
pub fn invert
pub fn resolve
```

## Usage

```toml
[dependencies]
ternary-temperament = "0.1.0"
```

```rust
use ternary_temperament::*;
// See src/lib.rs tests for complete working examples
```

## Testing

```bash
git clone https://github.com/SuperInstance/ternary-temperament.git
cd ternary-temperament
cargo test    # 16 tests
```

## Stats

| Metric | Value |
|--------|-------|
| Tests | 16 |
| Lines of Rust | 247 |
| Public API | 15 items |

## License

Apache-2.0
