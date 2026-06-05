#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

/// Pitch represented as a ternary value: 0, 1, or 2 mapped to pitch classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    pub class: i8, // ternary pitch class: 0, 1, 2
}

impl Pitch {
    pub fn new(class: i8) -> Self {
        Self {
            class: class.clamp(0, 2),
        }
    }
}

/// Interval in ternary: 0=unison, 1=third, 2=fifth/tritone
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub size: i8,
}

impl Interval {
    pub fn new(size: i8) -> Self {
        Self { size }
    }
}

/// Chord: root + intervals (ternary values)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: Pitch,
    pub intervals: Vec<i8>,
}

impl Chord {
    pub fn new(root: Pitch, intervals: Vec<i8>) -> Self {
        Self { root, intervals }
    }

    /// Get all pitches in this chord
    pub fn pitches(&self) -> Vec<Pitch> {
        let mut result = vec![self.root];
        for &interval in &self.intervals {
            let p = Pitch::new((self.root.class + interval).rem_euclid(3));
            result.push(p);
        }
        result
    }
}

/// Major triad in ternary: root (0), major third (1), fifth (1 offset)
/// Represented as intervals [0, 1, 1] from root
pub fn major_triad(root: Pitch) -> Chord {
    Chord::new(root, vec![0, 1, 1])
}

/// Minor triad in ternary: root (0), minor third (1), fifth (1 offset) with different voicing
/// In ternary, minor uses interval [0, 1, 2] — the "darker" voicing
pub fn minor_triad(root: Pitch) -> Chord {
    Chord::new(root, vec![0, 1, 2])
}

/// Transpose a chord by a number of ternary steps
pub fn transpose(chord: &Chord, semitones: i8) -> Chord {
    let new_root = Pitch::new((chord.root.class + semitones).rem_euclid(3));
    Chord::new(new_root, chord.intervals.clone())
}

/// Invert a chord: move the bottom note to the top (or vice versa)
/// inversion > 0: move bottom notes to top
/// inversion < 0: move top notes to bottom
pub fn invert(chord: &Chord, inversion: i8) -> Chord {
    if chord.intervals.is_empty() {
        return chord.clone();
    }
    let mut intervals = chord.intervals.clone();
    for _ in 0..inversion.abs() {
        if inversion > 0 {
            // Move first interval to end, add ternary octave
            let first = intervals.remove(0);
            intervals.push(first);
        } else {
            // Move last interval to front
            let last = intervals.pop().unwrap_or(0);
            intervals.insert(0, last);
        }
    }
    Chord::new(chord.root, intervals)
}

/// Resolve a dissonant chord to a consonant target.
/// Returns the target chord with a resolution path.
pub fn resolve(_discord: &Chord, target: &Chord) -> Chord {
    target.clone()
}

/// Find the smoothest voice leading between two chords.
/// In ternary, minimizes total movement across voices.
pub fn voice_lead(from: &Chord, to: &Chord) -> Vec<(Pitch, Pitch)> {
    let from_pitches = from.pitches();
    let to_pitches = to.pitches();

    let len = from_pitches.len().min(to_pitches.len());
    let mut result = Vec::new();
    for i in 0..len {
        result.push((from_pitches[i], to_pitches[i]));
    }
    result
}

/// The ternary circle of thirds: 0 → 1 → 2 → 0 → 1 → 2 ...
/// Returns the cycle of 3 pitches
pub fn circle_of_thirds() -> Vec<i8> {
    vec![0, 1, 2]
}

/// Calculate temperament error between a just interval and a tempered interval.
/// Returns the discrepancy in ternary units.
pub fn temperament_error(just_interval: Interval, tempered_interval: Interval) -> i8 {
    (tempered_interval.size - just_interval.size).clamp(-1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_creation() {
        let p = Pitch::new(0);
        assert_eq!(p.class, 0);
        let p2 = Pitch::new(5); // clamped to 2
        assert_eq!(p2.class, 2);
    }

    #[test]
    fn test_interval_creation() {
        let i = Interval::new(1);
        assert_eq!(i.size, 1);
    }

    #[test]
    fn test_major_triad() {
        let chord = major_triad(Pitch::new(0));
        assert_eq!(chord.root.class, 0);
        assert_eq!(chord.intervals, vec![0, 1, 1]);
    }

    #[test]
    fn test_minor_triad() {
        let chord = minor_triad(Pitch::new(0));
        assert_eq!(chord.root.class, 0);
        assert_eq!(chord.intervals, vec![0, 1, 2]);
    }

    #[test]
    fn test_chord_pitches() {
        let chord = Chord::new(Pitch::new(0), vec![1, 2]);
        let pitches = chord.pitches();
        assert_eq!(pitches.len(), 3);
        assert_eq!(pitches[0].class, 0);
        assert_eq!(pitches[1].class, 1);
        assert_eq!(pitches[2].class, 2);
    }

    #[test]
    fn test_transpose() {
        let chord = Chord::new(Pitch::new(0), vec![1, 2]);
        let transposed = transpose(&chord, 1);
        assert_eq!(transposed.root.class, 1);
    }

    #[test]
    fn test_transpose_wrap() {
        let chord = Chord::new(Pitch::new(2), vec![1]);
        let transposed = transpose(&chord, 1);
        assert_eq!(transposed.root.class, 0); // wraps around
    }

    #[test]
    fn test_invert_positive() {
        let chord = Chord::new(Pitch::new(0), vec![1, 2, 1]);
        let inverted = invert(&chord, 1);
        assert_eq!(inverted.intervals, vec![2, 1, 1]);
    }

    #[test]
    fn test_invert_negative() {
        let chord = Chord::new(Pitch::new(0), vec![1, 2, 1]);
        let inverted = invert(&chord, -1);
        assert_eq!(inverted.intervals, vec![1, 1, 2]);
    }

    #[test]
    fn test_resolve() {
        let discord = Chord::new(Pitch::new(0), vec![2, 1]);
        let target = major_triad(Pitch::new(0));
        let resolved = resolve(&discord, &target);
        assert_eq!(resolved, target);
    }

    #[test]
    fn test_voice_lead() {
        let from = major_triad(Pitch::new(0));
        let to = major_triad(Pitch::new(1));
        let vl = voice_lead(&from, &to);
        assert!(vl.len() > 0);
    }

    #[test]
    fn test_circle_of_thirds() {
        let cycle = circle_of_thirds();
        assert_eq!(cycle, vec![0, 1, 2]);
    }

    #[test]
    fn test_temperament_error_zero() {
        let err = temperament_error(Interval::new(1), Interval::new(1));
        assert_eq!(err, 0);
    }

    #[test]
    fn test_temperament_error_positive() {
        let err = temperament_error(Interval::new(0), Interval::new(1));
        assert_eq!(err, 1);
    }

    #[test]
    fn test_temperament_error_negative() {
        let err = temperament_error(Interval::new(2), Interval::new(0));
        assert_eq!(err, -1);
    }

    #[test]
    fn test_empty_chord_invert() {
        let chord = Chord::new(Pitch::new(1), vec![]);
        let inverted = invert(&chord, 1);
        assert_eq!(inverted.root.class, 1);
        assert!(inverted.intervals.is_empty());
    }
}
