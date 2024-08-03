use pyo3::prelude::*;
use pyo3::types::PyTuple;

mod pylogger;

fn convert_to_args(args: &Bound<'_, PyTuple>) -> Vec<pylogger::PrintableObject> {
    // try to convert to PrintableObject primitive
    // if that fails, just use a string repesentation of the object
    Python::with_gil(|_py| -> Vec<pylogger::PrintableObject> {
        args.into_iter()
            .map(|v| match v.extract() {
                Ok(printable) => printable,
                Err(error) => {
                    // TODO: remove below messages,
                    println!("Cannot easily convert type, using str repr");
                    println!("Error: {}", error);
                    pylogger::PrintableObject::PrintableObj(v.to_string())
                }
            })
            .collect()
    })
}

/// Prints a log to console for simple primitives (int, float, str)
#[pyfunction]
#[pyo3(signature=(format_str, *args))]
fn display_log(format_str: &str, args: &Bound<'_, PyTuple>) {
    let log_entry = pylogger::new(format_str, convert_to_args(args));
    pylogger::log_to_console(&log_entry);
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyzeromq_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(display_log, m)?)?;
    Ok(())
}
