use pyo3::prelude::*;

/// Clean HTML with a conservative set of defaults.
#[pyfunction]
fn clean(src: &str) -> PyResult<String> {
    Ok(ammonia::clean(&src))
}

/// Turn an arbitrary string into unformatted HTML.
#[pyfunction]
fn clean_text(src: &str) -> PyResult<String> {
    Ok(ammonia::clean_text(src))
}

/// Determine if a given string contains HTML
#[pyfunction]
fn is_html(input: &str) -> PyResult<bool> {
    Ok(ammonia::is_html(input))
}

/// Whitewash is python binding for Ammonia.
///
/// Ammonia is a whitelist-based HTML sanitization library. It is designed to
/// prevent cross-site scripting, layout breaking, and clickjacking caused
/// by untrusted user-provided HTML being mixed into a larger web page.
///
/// Ammonia uses [html5ever] to parse and serialize document fragments the same way browsers do,
/// so it is extremely resilient to syntactic obfuscation.
///
/// Ammonia parses its input exactly according to the HTML5 specification;
/// it will not linkify bare URLs, insert line or paragraph breaks, or convert `(C)` into &copy;.
#[pymodule]
fn whitewash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(clean, m)?)?;
    m.add_function(wrap_pyfunction!(clean_text, m)?)?;
    m.add_function(wrap_pyfunction!(is_html, m)?)?;

    Ok(())
}
