//! Peak detection over a single channel histogram.

use crate::histogram::{BINS, Hist};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peak {
    /// Bin index of the peak.
    pub index: u8,
    /// Pixel count of the bin range owned by this peak (midpoints between neighbours).
    pub mass: usize,
}

/// Local maxima of `hist` inside a `window_radius` neighbourhood; plateau centres are kept.
///
/// An entirely flat histogram yields no peaks.
pub fn find_windowed(hist: &Hist, window_radius: usize) -> Vec<usize> {
    let mut peaks = Vec::new();
    let mut i = 0;

    while i < BINS {
        let value = hist[i];

        let mut plateau_end = i;
        while plateau_end + 1 < BINS && hist[plateau_end + 1] == value {
            plateau_end += 1;
        }

        let left = i.saturating_sub(window_radius);
        let right = (plateau_end + window_radius).min(BINS - 1);
        let is_peak = (left..=right)
            .filter(|&k| k < i || k > plateau_end)
            .all(|k| hist[k] <= value);

        if is_peak && !(i == 0 && plateau_end == BINS - 1) {
            peaks.push((i + plateau_end) / 2);
        }

        i = plateau_end + 1;
    }

    peaks
}

/// Topographic prominence of `hist[peak_idx]` relative to the closest higher bins.
pub fn prominence(hist: &Hist, peak_idx: usize) -> usize {
    let height = hist[peak_idx];

    let mut left_min = height;
    for i in (0..peak_idx).rev() {
        left_min = left_min.min(hist[i]);
        if hist[i] > height {
            break;
        }
    }

    let mut right_min = height;
    for &value in &hist[peak_idx + 1..] {
        right_min = right_min.min(value);
        if value > height {
            break;
        }
    }

    let base_level = match peak_idx {
        0 => right_min,
        idx if idx == BINS - 1 => left_min,
        _ => left_min.max(right_min),
    };

    height - base_level
}

/// Peaks with prominence of at least `min_prominence`, thinned out by `min_distance`.
///
/// Accepted peaks are ordered by index and carry the mass of their bin range.
pub fn detect(
    hist: &Hist,
    window_radius: usize,
    min_prominence: usize,
    min_distance: usize,
) -> Vec<Peak> {
    let mut candidates: Vec<(usize, usize)> = find_windowed(hist, window_radius)
        .into_iter()
        .map(|index| (index, prominence(hist, index)))
        .filter(|&(_, p)| p >= min_prominence)
        .collect();

    // strongest first, so neighbours are dropped in favour of the dominant peak
    candidates.sort_by_key(|&(_, p)| std::cmp::Reverse(p));

    let mut indices: Vec<u8> = Vec::new();
    for (index, _) in candidates {
        let index = index as u8;
        if indices
            .iter()
            .all(|&kept| kept.abs_diff(index) as usize >= min_distance)
        {
            indices.push(index);
        }
    }
    indices.sort_unstable();

    let count = indices.len();
    (0..count)
        .map(|i| {
            let index = indices[i] as usize;
            let left = if i == 0 {
                0
            } else {
                (indices[i - 1] as usize + index) / 2
            };
            let right = if i + 1 == count {
                BINS - 1
            } else {
                (index + indices[i + 1] as usize) / 2
            };

            Peak {
                index: indices[i],
                mass: hist[left..=right].iter().sum(),
            }
        })
        .collect()
}
