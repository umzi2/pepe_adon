//! Per-channel histogram construction and smoothing.

pub const BINS: usize = 256;
pub const CHANNELS: usize = 3;

pub type Hist = [usize; BINS];

/// Builds R, G and B histograms from interleaved RGB8 pixels.
pub fn rgb_hists(img: &[u8]) -> [Hist; CHANNELS] {
    let mut hists = [[0usize; BINS]; CHANNELS];
    let (pixels, _) = img.as_chunks::<CHANNELS>();
    for px in pixels {
        hists[0][px[0] as usize] += 1;
        hists[1][px[1] as usize] += 1;
        hists[2][px[2] as usize] += 1;
    }
    hists
}

/// Smoothes a histogram with the `[1/4, 1/2, 1/4]` kernel, applied `iterations` times.
pub fn blur(hist: &mut Hist, iterations: usize) {
    for _ in 0..iterations {
        let prev = *hist;
        for i in 0..BINS {
            let lo = prev[i.saturating_sub(1)];
            let mid = prev[i];
            let hi = prev[(i + 1).min(BINS - 1)];
            hist[i] = lo / 4 + mid / 2 + hi / 4;
        }
    }
}
