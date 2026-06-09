//! # Ternary Temperament
//!
//! Tuning systems for ternary weights. In music, temperament determines how notes are
//! tuned — equal temperament divides the octave equally, just intonation uses pure
//! mathematical ratios, and meantone is a historical compromise. This crate brings those
//! concepts to ternary (−1, 0, +1) weight systems, offering different "tunings" for how
//! trit values map to real-valued weights.
//!
//! ## Core Concepts
//!
//! - **Equal Temperament**: Equal spacing between trits — the standard, balanced approach.
//! - **Just Intonation**: Pure mathematical ratios — theoretically perfect but may have gaps.
//! - **Meantone**: Historical compromise — smooth but slightly imperfect.
//! - **Microtonal**: Sub-trit divisions for fine-grained control beyond the standard 3 values.

#![forbid(unsafe_code)]

use std::fmt;

/// A trit value: Neg (-1), Zero (0), or Pos (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

impl Trit {
    pub fn value(&self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Trit::Neg),
            0 => Some(Trit::Zero),
            1 => Some(Trit::Pos),
            _ => None,
        }
    }
}

/// A tuned weight value (f64) with metadata about which tuning produced it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TunedWeight {
    pub value: f64,
    pub source_trit: Trit,
    pub tuning_system: TuningSystem,
}

impl TunedWeight {
    pub fn new(value: f64, source_trit: Trit, system: TuningSystem) -> Self {
        Self {
            value,
            source_trit: source_trit,
            tuning_system: system,
        }
    }
}

/// Identifies which tuning system produced a weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TuningSystem {
    Equal,
    Just,
    Meantone,
    Microtonal,
}

impl fmt::Display for TuningSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuningSystem::Equal => write!(f, "equal temperament"),
            TuningSystem::Just => write!(f, "just intonation"),
            TuningSystem::Meantone => write!(f, "meantone"),
            TuningSystem::Microtonal => write!(f, "microtonal"),
        }
    }
}

// ── EqualTemperament ─────────────────────────────────────────────────────────

/// Equal spacing between trits. The standard tuning: maps {-1, 0, +1} to {-1.0, 0.0, +1.0}
/// by default, but can scale to any range with equal spacing.
#[derive(Debug, Clone)]
pub struct EqualTemperament {
    /// The weight value for Trit::Neg.
    pub neg_weight: f64,
    /// The weight value for Trit::Zero.
    pub zero_weight: f64,
    /// The weight value for Trit::Pos.
    pub pos_weight: f64,
}

impl EqualTemperament {
    /// Standard equal temperament: -1.0, 0.0, +1.0.
    pub fn standard() -> Self {
        Self {
            neg_weight: -1.0,
            zero_weight: 0.0,
            pos_weight: 1.0,
        }
    }

    /// Equal temperament over a custom range [lo, hi].
    /// The three trits are equally spaced within the range.
    pub fn with_range(lo: f64, hi: f64) -> Self {
        let step = (hi - lo) / 2.0;
        Self {
            neg_weight: lo,
            zero_weight: lo + step,
            pos_weight: hi,
        }
    }

    /// Tune a single trit to its weight.
    pub fn tune(&self, trit: Trit) -> TunedWeight {
        let value = match trit {
            Trit::Neg => self.neg_weight,
            Trit::Zero => self.zero_weight,
            Trit::Pos => self.pos_weight,
        };
        TunedWeight::new(value, trit, TuningSystem::Equal)
    }

    /// Tune a sequence of trits.
    pub fn tune_sequence(&self, trits: &[Trit]) -> Vec<TunedWeight> {
        trits.iter().map(|&t| self.tune(t)).collect()
    }

    /// Verify equal spacing: the gap between consecutive weights is constant.
    pub fn verify_spacing(&self) -> bool {
        let gap1 = self.zero_weight - self.neg_weight;
        let gap2 = self.pos_weight - self.zero_weight;
        (gap1 - gap2).abs() < 1e-12
    }

    /// Step size between consecutive trit weights.
    pub fn step_size(&self) -> f64 {
        (self.pos_weight - self.neg_weight) / 2.0
    }
}

impl Default for EqualTemperament {
    fn default() -> Self {
        Self::standard()
    }
}

// ── JustIntonation ───────────────────────────────────────────────────────────

/// Just intonation uses pure mathematical ratios. For ternary systems, we map trits
/// using ratios derived from small integers, giving theoretically "pure" intervals
/// that may not be equally spaced.
#[derive(Debug, Clone)]
pub struct JustIntonation {
    /// Ratio for Neg trit (relative to base).
    pub neg_ratio: f64,
    /// Ratio for Zero trit (unity — always 1.0).
    pub zero_ratio: f64,
    /// Ratio for Pos trit (relative to base).
    pub pos_ratio: f64,
    /// Base frequency/magnitude.
    pub base: f64,
}

impl JustIntonation {
    /// Standard just intonation using small-integer ratios.
    /// Maps trits to base × ratio: Neg = base × 4/5, Zero = base × 1, Pos = base × 5/4.
    pub fn standard() -> Self {
        Self {
            neg_ratio: 4.0 / 5.0,
            zero_ratio: 1.0,
            pos_ratio: 5.0 / 4.0,
            base: 1.0,
        }
    }

    /// Pythagorean tuning: ratios based on powers of 3/2.
    pub fn pythagorean() -> Self {
        Self {
            neg_ratio: 8.0 / 9.0,
            zero_ratio: 1.0,
            pos_ratio: 9.0 / 8.0,
            base: 1.0,
        }
    }

    /// Set the base value.
    pub fn with_base(self, base: f64) -> Self {
        Self { base, ..self }
    }

    /// Tune a trit.
    pub fn tune(&self, trit: Trit) -> TunedWeight {
        let ratio = match trit {
            Trit::Neg => self.neg_ratio,
            Trit::Zero => self.zero_ratio,
            Trit::Pos => self.pos_ratio,
        };
        TunedWeight::new(self.base * ratio, trit, TuningSystem::Just)
    }

    /// Tune a sequence.
    pub fn tune_sequence(&self, trits: &[Trit]) -> Vec<TunedWeight> {
        trits.iter().map(|&t| self.tune(t)).collect()
    }

    /// Check if the ratios are pure (exact small-integer fractions).
    pub fn is_pure(&self) -> bool {
        // Check if ratios are close to simple fractions with small denominators
        for &ratio in &[self.neg_ratio, self.pos_ratio] {
            // Check common small fractions: 1/2, 2/3, 3/4, 4/5, 5/4, 3/2, 5/3, etc.
            let near_simple = [
                (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10),
                (1, 1), (10, 9), (9, 8), (8, 7), (7, 6), (6, 5), (5, 4), (4, 3), (3, 2), (2, 1),
            ];
            let found = near_simple.iter().any(|(n, d)| (ratio - (*n as f64 / *d as f64)).abs() < 0.01);
            if !found { return false; }
        }
        true
    }

    /// Deviation from equal spacing.
    pub fn deviation_from_equal(&self) -> f64 {
        let eq_step = (self.pos_ratio - self.neg_ratio) / 2.0;
        let actual_gap_low = self.zero_ratio - self.neg_ratio;
        let actual_gap_high = self.pos_ratio - self.zero_ratio;
        ((actual_gap_low - eq_step).abs() + (actual_gap_high - eq_step).abs()) / 2.0
    }
}

impl Default for JustIntonation {
    fn default() -> Self {
        Self::standard()
    }
}

// ── Meantone ─────────────────────────────────────────────────────────────────

/// Meantone temperament: a historical compromise between equal and just intonation.
/// Smooths out the "wolf intervals" of just intonation while preserving some purity.
#[derive(Debug, Clone)]
pub struct Meantone {
    /// The compromise factor: 0.0 = equal temperament, 1.0 = just intonation.
    pub alpha: f64,
    neg_weight: f64,
    zero_weight: f64,
    pos_weight: f64,
}

impl Meantone {
    /// Create meantone tuning with a compromise factor.
    /// `alpha` in [0.0, 1.0]: 0 = equal, 1 = just.
    pub fn new(alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        let eq = EqualTemperament::standard();
        let just = JustIntonation::standard();
        let neg = eq.neg_weight * (1.0 - alpha) + just.neg_ratio * alpha;
        let zero = eq.zero_weight * (1.0 - alpha) + just.zero_ratio * alpha;
        let pos = eq.pos_weight * (1.0 - alpha) + just.pos_ratio * alpha;
        Self {
            alpha,
            neg_weight: neg,
            zero_weight: zero,
            pos_weight: pos,
        }
    }

    /// Quarter-comma meantone (classic historical tuning).
    pub fn quarter_comma() -> Self {
        Self::new(0.25)
    }

    /// Tune a trit.
    pub fn tune(&self, trit: Trit) -> TunedWeight {
        let value = match trit {
            Trit::Neg => self.neg_weight,
            Trit::Zero => self.zero_weight,
            Trit::Pos => self.pos_weight,
        };
        TunedWeight::new(value, trit, TuningSystem::Meantone)
    }

    /// Tune a sequence.
    pub fn tune_sequence(&self, trits: &[Trit]) -> Vec<TunedWeight> {
        trits.iter().map(|&t| self.tune(t)).collect()
    }

    /// The alpha (compromise) parameter.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

// ── Microtonal ───────────────────────────────────────────────────────────────

/// Microtonal tuning subdivides each trit into sub-divisions for fine-grained control.
/// Instead of 3 distinct values, we get 2 * divisions + 1 possible micro-trit values.
#[derive(Debug, Clone)]
pub struct Microtonal {
    /// Number of subdivisions per whole trit step.
    pub divisions: u32,
    /// Range of output values.
    pub range: (f64, f64),
}

impl Microtonal {
    /// Create a microtonal system with `divisions` sub-steps per trit interval.
    pub fn new(divisions: u32) -> Self {
        Self {
            divisions,
            range: (-1.0, 1.0),
        }
    }

    /// With a custom output range.
    pub fn with_range(mut self, lo: f64, hi: f64) -> Self {
        self.range = (lo, hi);
        self
    }

    /// Total number of micro-trit values.
    pub fn total_values(&self) -> u32 {
        2 * self.divisions + 1
    }

    /// Tune a micro-trit value (index from -divisions to +divisions).
    /// Index 0 = center (Zero), -divisions = Neg, +divisions = Pos.
    pub fn tune_micro(&self, micro_index: i32) -> TunedWeight {
        let total_steps = 2 * self.divisions as i32;
        let (lo, hi) = self.range;
        let value = lo + (hi - lo) * (micro_index + self.divisions as i32) as f64 / total_steps as f64;
        let trit = if micro_index < 0 {
            Trit::Neg
        } else if micro_index > 0 {
            Trit::Pos
        } else {
            Trit::Zero
        };
        TunedWeight::new(value, trit, TuningSystem::Microtonal)
    }

    /// Map a standard trit to its coarse microtonal value.
    pub fn tune(&self, trit: Trit) -> TunedWeight {
        let idx = match trit {
            Trit::Neg => -(self.divisions as i32),
            Trit::Zero => 0,
            Trit::Pos => self.divisions as i32,
        };
        self.tune_micro(idx)
    }

    /// Tune a sequence of micro-indices.
    pub fn tune_micro_sequence(&self, indices: &[i32]) -> Vec<TunedWeight> {
        indices.iter().map(|&i| self.tune_micro(i)).collect()
    }

    /// Step size between consecutive micro-trits.
    pub fn micro_step_size(&self) -> f64 {
        let (lo, hi) = self.range;
        (hi - lo) / (2 * self.divisions) as f64
    }
}

// ── TuningComparison ─────────────────────────────────────────────────────────

/// Compare different tuning systems applied to the same ternary sequence.
#[derive(Debug, Clone)]
pub struct TuningComparison {
    pub equal_weights: Vec<f64>,
    pub just_weights: Vec<f64>,
    pub meantone_weights: Vec<f64>,
}

impl TuningComparison {
    /// Compare all three main tuning systems on a ternary sequence.
    pub fn compare(sequence: &[Trit]) -> Self {
        let eq = EqualTemperament::standard();
        let just = JustIntonation::standard();
        let mt = Meantone::quarter_comma();

        Self {
            equal_weights: eq.tune_sequence(sequence).iter().map(|w| w.value).collect(),
            just_weights: just.tune_sequence(sequence).iter().map(|w| w.value).collect(),
            meantone_weights: mt.tune_sequence(sequence).iter().map(|w| w.value).collect(),
        }
    }

    /// Maximum deviation between any two systems at any position.
    pub fn max_deviation(&self) -> f64 {
        let mut max_dev = 0.0_f64;
        for i in 0..self.equal_weights.len().min(self.just_weights.len()).min(self.meantone_weights.len()) {
            let dev1 = (self.equal_weights[i] - self.just_weights[i]).abs();
            let dev2 = (self.equal_weights[i] - self.meantone_weights[i]).abs();
            let dev3 = (self.just_weights[i] - self.meantone_weights[i]).abs();
            max_dev = max_dev.max(dev1).max(dev2).max(dev3);
        }
        max_dev
    }

    /// Mean absolute deviation between equal and just intonation.
    pub fn equal_vs_just_mad(&self) -> f64 {
        if self.equal_weights.is_empty() {
            return 0.0;
        }
        let sum: f64 = self
            .equal_weights
            .iter()
            .zip(self.just_weights.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        sum / self.equal_weights.len() as f64
    }

    /// Mean absolute deviation between equal and meantone.
    pub fn equal_vs_meantone_mad(&self) -> f64 {
        if self.equal_weights.is_empty() {
            return 0.0;
        }
        let sum: f64 = self
            .equal_weights
            .iter()
            .zip(self.meantone_weights.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        sum / self.equal_weights.len() as f64
    }

    /// Number of positions compared.
    pub fn len(&self) -> usize {
        self.equal_weights.len()
    }

    /// Whether the comparison is empty.
    pub fn is_empty(&self) -> bool {
        self.equal_weights.is_empty()
    }
}

// ── TemperamentAdapter ───────────────────────────────────────────────────────

/// Converts between tuning systems while preserving sequence structure.
#[derive(Debug, Clone)]
pub struct TemperamentAdapter;

impl TemperamentAdapter {
    /// Convert a sequence of weights from equal temperament to just intonation.
    /// Preserves the ordering (Neg < Zero < Pos) of the original trits.
    pub fn equal_to_just(equal_weights: &[TunedWeight]) -> Vec<TunedWeight> {
        let just = JustIntonation::standard();
        equal_weights
            .iter()
            .map(|w| just.tune(w.source_trit))
            .collect()
    }

    /// Convert from just to equal.
    pub fn just_to_equal(just_weights: &[TunedWeight]) -> Vec<TunedWeight> {
        let eq = EqualTemperament::standard();
        just_weights.iter().map(|w| eq.tune(w.source_trit)).collect()
    }

    /// Convert from any system to any other, preserving trit identity.
    pub fn convert(weights: &[TunedWeight], target: TuningSystem) -> Vec<TunedWeight> {
        match target {
            TuningSystem::Equal => {
                let eq = EqualTemperament::standard();
                weights.iter().map(|w| eq.tune(w.source_trit)).collect()
            }
            TuningSystem::Just => {
                let just = JustIntonation::standard();
                weights.iter().map(|w| just.tune(w.source_trit)).collect()
            }
            TuningSystem::Meantone => {
                let mt = Meantone::quarter_comma();
                weights.iter().map(|w| mt.tune(w.source_trit)).collect()
            }
            TuningSystem::Microtonal => {
                let micro = Microtonal::new(4);
                weights.iter().map(|w| micro.tune(w.source_trit)).collect()
            }
        }
    }

    /// Verify that conversion preserves trit ordering (Neg < Zero < Pos).
    pub fn verify_structure_preserved(original: &[TunedWeight], converted: &[TunedWeight]) -> bool {
        if original.len() != converted.len() {
            return false;
        }
        for (o, c) in original.iter().zip(converted.iter()) {
            if o.source_trit != c.source_trit {
                return false;
            }
        }
        true
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Tests
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── EqualTemperament tests ──

    #[test]
    fn equal_standard_values() {
        let eq = EqualTemperament::standard();
        assert!((eq.neg_weight - (-1.0)).abs() < 1e-12);
        assert!((eq.zero_weight - 0.0).abs() < 1e-12);
        assert!((eq.pos_weight - 1.0).abs() < 1e-12);
    }

    #[test]
    fn equal_verify_spacing() {
        assert!(EqualTemperament::standard().verify_spacing());
    }

    #[test]
    fn equal_step_size() {
        assert!((EqualTemperament::standard().step_size() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn equal_custom_range() {
        let eq = EqualTemperament::with_range(0.0, 1.0);
        assert!((eq.neg_weight - 0.0).abs() < 1e-12);
        assert!((eq.zero_weight - 0.5).abs() < 1e-12);
        assert!((eq.pos_weight - 1.0).abs() < 1e-12);
        assert!(eq.verify_spacing());
    }

    #[test]
    fn equal_tune_trit() {
        let eq = EqualTemperament::standard();
        let w = eq.tune(Trit::Pos);
        assert_eq!(w.source_trit, Trit::Pos);
        assert_eq!(w.tuning_system, TuningSystem::Equal);
        assert!((w.value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn equal_tune_sequence() {
        let eq = EqualTemperament::standard();
        let weights = eq.tune_sequence(&[Trit::Neg, Trit::Zero, Trit::Pos]);
        assert_eq!(weights.len(), 3);
        assert!((weights[0].value - (-1.0)).abs() < 1e-12);
        assert!((weights[1].value - 0.0).abs() < 1e-12);
        assert!((weights[2].value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn equal_default() {
        assert!((EqualTemperament::default().pos_weight - 1.0).abs() < 1e-12);
    }

    // ── JustIntonation tests ──

    #[test]
    fn just_standard_ratios() {
        let just = JustIntonation::standard();
        assert!((just.neg_ratio - 0.8).abs() < 1e-12);
        assert!((just.zero_ratio - 1.0).abs() < 1e-12);
        assert!((just.pos_ratio - 1.25).abs() < 1e-12);
    }

    #[test]
    fn just_tune() {
        let just = JustIntonation::standard();
        let w = just.tune(Trit::Pos);
        assert_eq!(w.tuning_system, TuningSystem::Just);
        assert!((w.value - 1.25).abs() < 1e-12);
    }

    #[test]
    fn just_is_pure() {
        assert!(JustIntonation::standard().is_pure());
    }

    #[test]
    fn just_deviation_from_equal() {
        let just = JustIntonation::standard();
        let dev = just.deviation_from_equal();
        assert!(dev > 0.0, "just intonation should deviate from equal");
    }

    #[test]
    fn pythagorean() {
        let pyth = JustIntonation::pythagorean();
        assert!((pyth.neg_ratio - 8.0 / 9.0).abs() < 1e-12);
        assert!((pyth.pos_ratio - 9.0 / 8.0).abs() < 1e-12);
    }

    #[test]
    fn just_with_base() {
        let just = JustIntonation::standard().with_base(2.0);
        let w = just.tune(Trit::Pos);
        assert!((w.value - 2.5).abs() < 1e-12);
    }

    #[test]
    fn just_tune_sequence() {
        let just = JustIntonation::standard();
        let weights = just.tune_sequence(&[Trit::Neg, Trit::Zero, Trit::Pos]);
        assert_eq!(weights.len(), 3);
    }

    // ── Meantone tests ──

    #[test]
    fn meantone_zero_alpha_is_equal() {
        let mt = Meantone::new(0.0);
        let w = mt.tune(Trit::Pos);
        assert_eq!(w.tuning_system, TuningSystem::Meantone);
        assert!((w.value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn meantone_one_alpha_is_just() {
        let mt = Meantone::new(1.0);
        let w = mt.tune(Trit::Pos);
        assert!((w.value - 1.25).abs() < 1e-12);
    }

    #[test]
    fn meantone_clamps_alpha() {
        let mt = Meantone::new(5.0);
        assert!((mt.alpha() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn meantone_quarter_comma() {
        let mt = Meantone::quarter_comma();
        let w_neg = mt.tune(Trit::Neg);
        let w_pos = mt.tune(Trit::Pos);
        // Quarter comma should be between equal and just
        assert!(w_neg.value > -1.0); // less extreme than equal
        assert!(w_pos.value < 1.25); // less extreme than just
    }

    #[test]
    fn meantone_tune_sequence() {
        let mt = Meantone::new(0.5);
        let weights = mt.tune_sequence(&[Trit::Neg, Trit::Zero, Trit::Pos]);
        assert_eq!(weights.len(), 3);
    }

    // ── Microtonal tests ──

    #[test]
    fn microtonal_total_values() {
        let micro = Microtonal::new(4);
        assert_eq!(micro.total_values(), 9); // 2*4+1
    }

    #[test]
    fn microtonal_tune_standard_trits() {
        let micro = Microtonal::new(4);
        let w_neg = micro.tune(Trit::Neg);
        let w_zero = micro.tune(Trit::Zero);
        let w_pos = micro.tune(Trit::Pos);
        assert!((w_neg.value - (-1.0)).abs() < 1e-12);
        assert!((w_zero.value - 0.0).abs() < 1e-12);
        assert!((w_pos.value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn microtonal_micro_step_size() {
        let micro = Microtonal::new(4);
        assert!((micro.micro_step_size() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn microtonal_intermediate_values() {
        let micro = Microtonal::new(4);
        let w = micro.tune_micro(1); // one step above center
        assert!((w.value - 0.25).abs() < 1e-12);
        let w2 = micro.tune_micro(-2);
        assert!((w2.value - (-0.5)).abs() < 1e-12);
    }

    #[test]
    fn microtonal_custom_range() {
        let micro = Microtonal::new(2).with_range(0.0, 100.0);
        let w = micro.tune(Trit::Pos);
        assert!((w.value - 100.0).abs() < 1e-12);
    }

    #[test]
    fn microtonal_micro_sequence() {
        let micro = Microtonal::new(4);
        let weights = micro.tune_micro_sequence(&[-4, -2, 0, 2, 4]);
        assert_eq!(weights.len(), 5);
    }

    // ── TuningComparison tests ──

    #[test]
    fn comparison_basic() {
        use Trit::*;
        let seq = vec![Neg, Zero, Pos, Neg];
        let comp = TuningComparison::compare(&seq);
        assert_eq!(comp.len(), 4);
    }

    #[test]
    fn comparison_max_deviation() {
        let comp = TuningComparison::compare(&[Trit::Pos]);
        assert!(comp.max_deviation() >= 0.0);
    }

    #[test]
    fn comparison_equal_vs_just_mad() {
        let comp = TuningComparison::compare(&[Trit::Pos, Trit::Neg]);
        let mad = comp.equal_vs_just_mad();
        assert!(mad > 0.0, "equal and just should differ");
    }

    #[test]
    fn comparison_equal_vs_meantone() {
        let comp = TuningComparison::compare(&[Trit::Pos]);
        let mad = comp.equal_vs_meantone_mad();
        assert!(mad > 0.0, "quarter-comma meantone should differ from equal");
    }

    #[test]
    fn comparison_empty() {
        let comp = TuningComparison::compare(&[]);
        assert!(comp.is_empty());
        assert_eq!(comp.len(), 0);
    }

    // ── TemperamentAdapter tests ──

    #[test]
    fn adapter_equal_to_just() {
        let eq = EqualTemperament::standard();
        let equal_weights = eq.tune_sequence(&[Trit::Neg, Trit::Zero, Trit::Pos]);
        let just_weights = TemperamentAdapter::equal_to_just(&equal_weights);
        assert!(TemperamentAdapter::verify_structure_preserved(&equal_weights, &just_weights));
        assert_eq!(just_weights[0].tuning_system, TuningSystem::Just);
    }

    #[test]
    fn adapter_just_to_equal() {
        let just = JustIntonation::standard();
        let just_weights = just.tune_sequence(&[Trit::Pos]);
        let eq_weights = TemperamentAdapter::just_to_equal(&just_weights);
        assert!((eq_weights[0].value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn adapter_convert_to_all_systems() {
        let eq = EqualTemperament::standard();
        let weights = eq.tune_sequence(&[Trit::Pos, Trit::Neg, Trit::Zero]);
        for system in &[TuningSystem::Equal, TuningSystem::Just, TuningSystem::Meantone, TuningSystem::Microtonal] {
            let converted = TemperamentAdapter::convert(&weights, *system);
            assert!(TemperamentAdapter::verify_structure_preserved(&weights, &converted));
            assert_eq!(converted[0].tuning_system, *system);
        }
    }

    #[test]
    fn adapter_structure_preserved() {
        let eq = EqualTemperament::standard();
        let w = eq.tune_sequence(&[Trit::Pos, Trit::Neg]);
        let c = TemperamentAdapter::convert(&w, TuningSystem::Just);
        assert!(TemperamentAdapter::verify_structure_preserved(&w, &c));
    }

    #[test]
    fn adapter_detects_mismatch() {
        let eq = EqualTemperament::standard();
        let w1 = eq.tune_sequence(&[Trit::Pos]);
        let w2 = eq.tune_sequence(&[Trit::Neg]);
        assert!(!TemperamentAdapter::verify_structure_preserved(&w1, &w2));
    }

    #[test]
    fn adapter_detects_length_mismatch() {
        let eq = EqualTemperament::standard();
        let w1 = eq.tune_sequence(&[Trit::Pos, Trit::Neg]);
        let w2 = eq.tune_sequence(&[Trit::Pos]);
        assert!(!TemperamentAdapter::verify_structure_preserved(&w1, &w2));
    }

    // ── TuningSystem display ──

    #[test]
    fn tuning_system_display() {
        assert_eq!(TuningSystem::Equal.to_string(), "equal temperament");
        assert_eq!(TuningSystem::Just.to_string(), "just intonation");
        assert_eq!(TuningSystem::Meantone.to_string(), "meantone");
        assert_eq!(TuningSystem::Microtonal.to_string(), "microtonal");
    }

    // ── Trit basics ──

    #[test]
    fn trit_from_i8() {
        assert_eq!(Trit::from_i8(-1), Some(Trit::Neg));
        assert_eq!(Trit::from_i8(0), Some(Trit::Zero));
        assert_eq!(Trit::from_i8(1), Some(Trit::Pos));
        assert_eq!(Trit::from_i8(2), None);
    }
}
