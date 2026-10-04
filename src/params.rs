//! Processing knobs shared by the whole pipeline.

use crate::histogram::BINS;

pub const DEFAULT_BLUR_N: usize = 3;
pub const DEFAULT_WINDOW_RADIUS: usize = 3;
/// Percent of the total pixel count; the original fixed threshold was `total / 200`.
pub const DEFAULT_MIN_PROMINENCE_PERCENT: f64 = 0.5;
pub const DEFAULT_MIN_DISTANCE: usize = 10;
pub const DEFAULT_PERCENTAGE: f64 = 0.22;

pub const MAX_BLUR_N: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// Number of histogram smoothing passes (`[1/4, 1/2, 1/4]` kernel each).
    pub blur_n: usize,
    /// Neighbourhood radius, in histogram bins, used to look for local maxima.
    pub window_radius: usize,
    /// Minimum prominence of a peak, as a percentage of the total pixel count.
    pub min_prominence_percent: f64,
    /// Minimum distance, in histogram bins, between two accepted peaks.
    pub min_distance: usize,
    /// Maximum relative channel mass deviation that still counts as neutral colour.
    pub percentage: f64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            blur_n: DEFAULT_BLUR_N,
            window_radius: DEFAULT_WINDOW_RADIUS,
            min_prominence_percent: DEFAULT_MIN_PROMINENCE_PERCENT,
            min_distance: DEFAULT_MIN_DISTANCE,
            percentage: DEFAULT_PERCENTAGE,
        }
    }
}

impl Params {
    pub fn new(
        blur_n: usize,
        window_radius: usize,
        min_prominence_percent: f64,
        min_distance: usize,
        percentage: f64,
    ) -> Result<Self, String> {
        if blur_n > MAX_BLUR_N {
            return Err(format!("blur_n must be <= {MAX_BLUR_N}, got {blur_n}"));
        }
        if window_radius == 0 || window_radius >= BINS {
            return Err(format!(
                "window_radius must be in 1..{BINS}, got {window_radius}"
            ));
        }
        if min_distance == 0 || min_distance >= BINS {
            return Err(format!(
                "min_distance must be in 1..{BINS}, got {min_distance}"
            ));
        }
        if !min_prominence_percent.is_finite() || min_prominence_percent < 0.0 {
            return Err(format!(
                "min_prominence must be a finite percentage >= 0, got {min_prominence_percent}"
            ));
        }
        if !percentage.is_finite() || percentage < 0.0 {
            return Err(format!(
                "percentage must be a finite ratio >= 0, got {percentage}"
            ));
        }

        Ok(Self {
            blur_n,
            window_radius,
            min_prominence_percent,
            min_distance,
            percentage,
        })
    }

    /// Absolute prominence threshold for an image of `total_pixels` pixels.
    pub fn min_prominence(&self, total_pixels: usize) -> usize {
        ((total_pixels as f64 * self.min_prominence_percent / 100.0) as usize).max(1)
    }
}
