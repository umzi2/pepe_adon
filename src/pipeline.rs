//! Monochrome detection and level fix over interleaved RGB8 pixels.

use crate::color::stretch_to_luma;
use crate::histogram::{blur, rgb_hists};
use crate::params::Params;
use crate::peaks::{Peak, detect};

/// A channel set is a monochrome candidate while every channel has few, dominant peaks.
const MAX_PEAKS: usize = 3;

/// Applies the monochrome fix to `img` in place when the image reads as neutral.
pub fn process_rgb(img: &mut [u8], params: &Params) {
    if img.len() < 3 {
        return;
    }

    let mut hists = rgb_hists(img);
    for hist in &mut hists {
        blur(hist, params.blur_n);
    }

    let min_prominence = params.min_prominence(img.len() / 3);
    let [r, g, b] = hists.map(|hist| {
        detect(
            &hist,
            params.window_radius,
            min_prominence,
            params.min_distance,
        )
    });

    if ![&r, &g, &b].iter().all(|peaks| !peaks.is_empty() && peaks.len() < MAX_PEAKS) {
        return;
    }

    let (low, high): ([Peak; 3], [Peak; 3]) = ([r[0], g[0], b[0]], [
        r.get(1).copied().unwrap_or(r[0]),
        g.get(1).copied().unwrap_or(g[0]),
        b.get(1).copied().unwrap_or(b[0]),
    ]);

    let neutral = [&low, &high]
        .iter()
        .all(|[r, g, b]| is_monochrome(r, g, b, params.percentage));

    if neutral {
        stretch_to_luma(
            img,
            low.map(|peak| peak.index),
            high.map(|peak| peak.index),
        );
    }
}

/// Channels whose mass deviates from the mean by less than `percentage` are neutral.
fn is_monochrome(r: &Peak, g: &Peak, b: &Peak, percentage: f64) -> bool {
    let mean = (r.mass + g.mass + b.mass) as f64 / 3.0;
    if mean == 0.0 {
        return true;
    }

    [r, g, b]
        .iter()
        .all(|peak| (peak.mass as f64 - mean).abs() / mean < percentage)
}
