# ternary-temperament

Tuning systems and harmonic temperament in ternary pitch space — chords, intervals, voice leading, and the circle of thirds for three-pitch-class music.

## Background

Temperament — the art of dividing the octave into usable intervals — has shaped Western music for centuries. From Pythagorean just intonation to equal temperament, each system represents a different compromise between mathematical purity and musical practicality. Meantone temperament flattened fifths to sweeten thirds. Well temperament gave each key its own color. Equal temperament made all keys identical.

Ternary temperament takes this to its logical extreme: the octave is divided into exactly three pitch classes. There is no "circle of fifths" because there are only three intervals: unison (0), third (1), and tritone/fifth (2). The entire harmonic universe exists within a single augmented triad.

This crate implements chord construction, inversion, transposition, and voice leading in this minimal pitch space — and discovers that even with only three pitch classes, the fundamental operations of tonal harmony remain meaningful.

## How It Works

### Pitch and Interval

Pitches are ternary pitch classes: 0, 1, or 2. Intervals are computed modulo 3, yielding exactly three possibilities:

| Interval | Mod-3 Value | Traditional Analogue |
|----------|-------------|----------------------|
| Unison   | 0           | Unison / octave      |
| Third    | 1           | Major/minor third    |
| Fifth    | 2           | Tritone / diminished fifth |

### Chord Construction

Chords are built from a root pitch and a vector of intervals. The crate provides:

- **`major_triad(root)`** — intervals [0, 1, 1], the "bright" triad
- **`minor_triad(root)`** — intervals [0, 1, 2], the "dark" triad
- **Transpose** — shift root by a ternary step, wrapping mod 3
- **Invert** — rotate intervals to move bottom notes to top (or vice versa)

The distinction between major and minor is subtle but real in ternary: the major triad [0, 1, 1] emphasizes the "third" interval, while the minor triad [0, 1, 2] includes the "tritone" quality.

### Voice Leading

`voice_lead(from, to)` computes the minimal mapping between two chords — pairing pitches position-by-position to show how each voice moves. In ternary, voice leading distances are always small (at most 2 steps), reflecting the compressed pitch space.

### Circle of Thirds

The ternary equivalent of the circle of fifths: 0 → 1 → 2 → 0. In traditional theory, the circle of fifths connects all 12 keys. In ternary, the "circle of thirds" cycles through all three pitch classes in a single step — every key is adjacent to every other key.

### Temperament Error

`temperament_error(just, tempered)` measures the discrepancy between a target interval and a tempered approximation, clamped to ternary range {-1, 0, +1}. In a 3-pitch-class system, temperament error is always dramatic: you're either exactly right (0) or off by at least one third (±1).

## Experimental Results

- **Only two distinct triads exist.** Major and minor are the only non-degenerate three-note chords in ternary pitch space. Every other combination either duplicates a pitch or reduces to one of these two.
- **Transposition wraps in 3 steps.** Transposing any chord three times returns to the original — the entire key space cycles in three steps, compared to 12 in chromatic music.
- **Inversion preserves chord quality.** Unlike traditional harmony where inversions change the bass note and thus the chord character, ternary inversion merely rotates which voice has the root. All inversions are equivalent.
- **Temperament error is binary.** With only three pitch classes, any "out-of-tune" interval is off by exactly one step. There is no such thing as a "slightly" out-of-tune ternary interval.
- **Voice leading is always smooth.** The maximum voice leading distance between any two chords is 2 steps (one voice moves by a tritone). Most transitions are 0 or 1 step.

## Impact

Ternary temperament reveals that the essential operations of tonal harmony — chord construction, transposition, inversion, voice leading — are not dependent on the richness of the pitch space. They work with as few as three pitch classes, though the resulting musical vocabulary is severely constrained.

The crate provides a formal demonstration that temperament is a relative concept: the "errors" in any tuning system are meaningful only in relation to the number of available pitch classes.

## Use Cases

1. **Microtonal music research** — Explore the minimum viable tuning system for functional harmony. Ternary temperament is the simplest non-trivial system.
2. **Generative music** — Build chord progressions and voice leading algorithms with mathematically guaranteed minimal distance between all chords.
3. **Music theory education** — Demonstrate chord construction, transposition, and inversion concepts using a pitch space small enough to enumerate completely.
4. **Algorithmic composition constraints** — Use ternary temperament as a constraint system where every valid progression can be listed exhaustively.

## Open Questions

1. **Extended ternary harmony.** Can seventh chords, ninth chords, or extended harmony be meaningfully defined in a three-pitch-class system, or do they collapse into triads?
2. **Functional harmony.** Does tonic/dominant/subdominant function exist in ternary? The three pitch classes map naturally to these functions, but does a I-IV-V progression make sense when IV and V are adjacent?
3. **Just intonation for ternary.** If ternary intervals were tuned to just ratios rather than equal steps, would the 1:5:3 ratio (a "just" ternary triad) produce perceptibly different sonorities?

## Connection to Oxide Stack

`ternary-temperament` provides the harmonic foundation for `ternary-music` (which maps ternary chords to 12-tone equivalents), `ternary-counterpoint` (which uses interval classification for consonance/dissonance rules), and `ternary-color` (which applies analogous classification principles to visual temperature). The circle of thirds conceptually mirrors `ternary-compass`'s navigational bearings.
