#[derive(Debug, Clone)]
/// The desired detection configuration.
pub struct DetectionOptions {
    pub max_hands: usize,
    pub score_threshold: f32,
}

impl DetectionOptions {
    /// Creates a new `DetectionOptions` from provided preferences.
    pub fn new(max_hands: usize, score_threshold: f32) -> Self {
        Self {
            max_hands,
            score_threshold,
        }
    }
}
