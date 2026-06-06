# ternary-temperament

**Tuning systems for ternary weights.**

In music, temperament determines how notes are tuned relative to each other. Equal
temperament divides the octave into equal steps; just intonation uses pure mathematical
ratios; meantone is a historical compromise. This crate brings those concepts to ternary
(−1, 0, +1) weight systems, offering different "tunings" for how trit values map to
real-valued weights.

## Overview

A ternary system uses three values: `Neg` (−1), `Zero` (0), and `Pos` (+1). When mapping
these abstract values to concrete weights (e.g., for neural network connections, agent
parameters, or resource allocations), the choice of mapping matters. This crate provides
several "temperaments" — systematic approaches to tuning the mapping from trits to weights.

## Tuning Systems

### EqualTemperament

The standard tuning: equal spacing between all trit values. Maps `{Neg, Zero, Pos}` to
equally spaced values in a configurable range.

```
Default: Neg → -1.0, Zero → 0.0, Pos → +1.0
Custom:  Neg → 0.0,  Zero → 0.5, Pos → +1.0  (range [0, 1])
```

**When to use**: When you need balanced, predictable behavior. The default choice for
most applications.

### JustIntonation

Uses pure mathematical ratios derived from small integers. Theoretically "perfect" but
not equally spaced — just like in music, where just intonation produces pure intervals
but some keys sound better than others.

```
Standard: Neg → 0.8, Zero → 1.0, Pos → 1.25  (ratios 4/5, 1/1, 5/4)
Pythagorean: Neg → 8/9, Zero → 1.0, Pos → 9/8
```

**When to use**: When mathematical purity matters more than uniformity. Good for
systems where the ratios between weights are more important than their absolute values.

### Meantone

A compromise between equal and just intonation, parameterized by alpha ∈ [0.0, 1.0]:

- `alpha = 0.0` → Equal temperament
- `alpha = 1.0` → Just intonation
- `alpha = 0.25` → Quarter-comma meantone (classic historical tuning)

**When to use**: When you want some of the purity of just intonation but need smoother
transitions. The alpha parameter lets you dial in the right balance.

### Microtonal

Subdivides each trit step into finer divisions for sub-trit precision. With `n` divisions,
you get `2n + 1` possible micro-trit values instead of just 3.

```
divisions=4: 9 values from -1.0 to +1.0 in steps of 0.25
divisions=2: 5 values from -1.0 to +1.0 in steps of 0.5
```

**When to use**: When you need fine-grained control beyond the standard three values.
Useful for continuous control systems or gradual transitions.

## Comparison & Conversion

### TuningComparison

Compare all tuning systems side-by-side on the same ternary sequence:

```rust
let comp = TuningComparison::compare(&seq);
let max_dev = comp.max_deviation();
let mad = comp.equal_vs_just_mad();
```

### TemperamentAdapter

Convert between tuning systems while preserving sequence structure (trit identity):

```rust
let equal_weights = eq.tune_sequence(&seq);
let just_weights = TemperamentAdapter::equal_to_just(&equal_weights);
assert!(TemperamentAdapter::verify_structure_preserved(&equal_weights, &just_weights));
```

## Usage

```rust
use ternary_temperament::*;

// Equal temperament (standard)
let eq = EqualTemperament::standard();
let w = eq.tune(Trit::Pos);
assert_eq!(w.value, 1.0);

// Just intonation
let just = JustIntonation::standard();
let w = just.tune(Trit::Pos);
assert_eq!(w.value, 1.25);

// Meantone compromise
let mt = Meantone::new(0.5); // halfway between equal and just
let w = mt.tune(Trit::Pos);

// Microtonal (fine-grained)
let micro = Microtonal::new(4);
let w = micro.tune_micro(2); // two sub-steps above center
assert_eq!(w.value, 0.5);

// Compare systems
let comp = TuningComparison::compare(&[Trit::Neg, Trit::Zero, Trit::Pos]);
println!("Max deviation: {}", comp.max_deviation());

// Convert between systems
let weights = eq.tune_sequence(&seq);
let converted = TemperamentAdapter::convert(&weights, TuningSystem::Just);
```

## Design Philosophy

The temperament metaphor is surprisingly apt for ternary weight systems:

- **Equal temperament** = democratic, no value is privileged
- **Just intonation** = mathematically elegant, but with uneven gaps
- **Meantone** = practical compromise for real-world use
- **Microtonal** = when three values aren't enough

Just as musicians choose temperaments based on the music they're playing, agent
designers can choose tunings based on the behavior they want. A conversational agent
might prefer equal temperament for balanced responses, while a creative agent might
benefit from the asymmetric ratios of just intonation.

## Testing

```bash
cargo test
```

All 38 tests pass, covering equal temperament spacing, just intonation ratios, meantone
interpolation, microtonal subdivisions, comparison metrics, and adapter correctness.

## License

MIT
