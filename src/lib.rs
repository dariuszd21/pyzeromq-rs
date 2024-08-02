use pyo3::prelude::*;
use pyo3::types::PyTuple;

mod pylogger;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
#[pyo3(signature=(format_str, *args))]
fn display_log(format_str: &str, args: &Bound<'_, PyTuple>) {
    for a in args {
        println!("{}", a);
    }
    pylogger::log(format_str);
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyzeromq_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(display_log, m)?)?;
    Ok(())
}
