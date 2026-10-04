//! Python bindings: `process` takes an `(H, W, 3)` uint8 array and returns the processed copy.

mod color;
mod histogram;
mod params;
mod peaks;
mod pipeline;

use numpy::ndarray::Array3;
use numpy::{IntoPyArray, PyArray3, PyReadonlyArray3, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::params::{
    DEFAULT_BLUR_N, DEFAULT_MIN_DISTANCE, DEFAULT_MIN_PROMINENCE_PERCENT, DEFAULT_PERCENTAGE,
    DEFAULT_WINDOW_RADIUS, Params,
};

/// Keeps the Python-visible defaults in `process` in sync with `params`.
const _: () = {
    assert!(DEFAULT_BLUR_N == 3);
    assert!(DEFAULT_WINDOW_RADIUS == 3);
    assert!(DEFAULT_MIN_PROMINENCE_PERCENT.to_bits() == 0.5f64.to_bits());
    assert!(DEFAULT_MIN_DISTANCE == 10);
    assert!(DEFAULT_PERCENTAGE.to_bits() == 0.22f64.to_bits());
};

/// Fix black/white levels of monochrome (neutral) images.
///
/// Args:
///     img: ``(H, W, 3)`` uint8 RGB array.
///     blur_n: histogram smoothing passes, ``>= 0``.
///     window_radius: local maximum search radius in histogram bins.
///     min_prominence: minimum peak prominence in percent of the total pixel count.
///     min_distance: minimum distance between accepted peaks, in bins.
///     percentage: max relative channel mass deviation treated as neutral colour.
///
/// Returns:
///     New ``(H, W, 3)`` uint8 array; unchanged pixels when the image is not monochrome.
#[pyfunction]
#[pyo3(signature = (
    img,
    blur_n = 3,
    window_radius = 3,
    min_prominence = 0.5,
    min_distance = 10,
    percentage = 0.22,
))]
fn monochrome<'py>(
    py: Python<'py>,
    img: PyReadonlyArray3<'py, u8>,
    blur_n: usize,
    window_radius: usize,
    min_prominence: f64,
    min_distance: usize,
    percentage: f64,
) -> PyResult<Bound<'py, PyArray3<u8>>> {
    let shape = img.shape();
    let (height, width, channels) = (shape[0], shape[1], shape[2]);
    if channels != 3 {
        return Err(PyValueError::new_err(format!(
            "expected an (H, W, 3) array, got {channels} channels"
        )));
    }

    let params = Params::new(
        blur_n,
        window_radius,
        min_prominence,
        min_distance,
        percentage,
    )
    .map_err(PyValueError::new_err)?;

    let mut data = match img.as_slice() {
        Ok(contiguous) => contiguous.to_vec(),
        Err(_) => img.as_array().iter().copied().collect(),
    };
    py.detach(|| pipeline::process_rgb(&mut data, &params));

    let processed = Array3::from_shape_vec((height, width, 3), data)
        .map_err(|err| PyValueError::new_err(err.to_string()))?;

    Ok(processed.into_pyarray(py))
}

#[pymodule]
fn pepe_addon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(monochrome, m)?)?;
    m.add("__doc__", "Monochrome level fix for RGB8 image arrays.")?;
    Ok(())
}
