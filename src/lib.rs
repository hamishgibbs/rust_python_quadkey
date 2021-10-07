use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

struct Range(f64, f64);

const EARTH_RADIUS: f64 = 6378137.0;
const LATITUDE_RANGE: Range = Range(-85.05112878, 85.05112878);


/// Clip number of a range
fn clip(n: f64, range: Range) -> f64 {
    return f64::min(f64::max(n, range.0), range.1)
}

/// Get map size for a certain level
fn map_size(level: u64) -> u64 {
    let base: u64 = 256;
    return base << level
}

/// Get ground resolution for a given latitude & level
fn ground_resolution(mut lat: f64, level: u64) -> f64 {
     lat = clip(lat, LATITUDE_RANGE);
     return (lat * std::f64::consts::PI / 180.0).cos() * 2.0 * std::f64::consts::PI * EARTH_RADIUS / map_size(level) as f64;
}


#[pyfunction]
fn py_ground_resolution(lat: f64, level: u64) -> PyResult<f64> {

    Ok(ground_resolution(lat, level))

}

/// A Python module implemented in Rust.
#[pymodule]
fn quadkeys(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_ground_resolution, m)?)?;

    Ok(())
}
