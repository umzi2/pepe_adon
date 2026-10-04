//! Level correction for monochrome images stored with per-channel black/white points.

use crate::histogram::CHANNELS;

/// Rec.601 luma weights.
const LUMA: [f32; CHANNELS] = [0.299, 1.0-0.299-0.114, 0.114];

/// Stretches every channel from its own `low`/`high` levels to `[0, 1]`, then writes the
/// Rec.601 luma of the stretched pixel back as neutral grey.
pub fn stretch_to_luma(img: &mut [u8], low: [u8; CHANNELS], high: [u8; CHANNELS]) {
    let low = low.map(|level| level as f32 / 255.0);
    let high = high.map(|level| level as f32 / 255.0);
    let range = std::array::from_fn::<_, CHANNELS, _>(|c| (high[c] - low[c]).max(f32::EPSILON));

    let (pixels, _) = img.as_chunks_mut::<CHANNELS>();

    for px in pixels {
        let luma: f32 = (0..CHANNELS)
            .map(|c| ((px[c] as f32 / 255.0 - low[c]) / range[c]).clamp(0.0, 1.0) * LUMA[c])
            .sum();

        px.fill((luma * 255.0).round().clamp(0.0, 255.0) as u8);
    }
}
