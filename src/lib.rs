use pyo3::prelude::*;
use aleph_client::{RawKeyPair,Pair};



#[pyfunction]
fn generate_phrase() -> PyResult<String> {
    let key = RawKeyPair::generate_with_phrase(Some("password"));
    Ok(key.1)
}


/// A Python module implemented in Rust.
#[pymodule]
fn aleph_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_phrase, m)?)?;
    Ok(())
}
