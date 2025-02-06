use pyo3::prelude::*;
pub mod lib_basic;
pub mod lib_adc;


/// A Python module implemented in Rust.
#[pymodule]
fn raspecio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let basic = PyModule::new(m.py(), "basic")?;
    basic.add_function(wrap_pyfunction!(lib_basic::blink, &basic)?)?;
    m.add_submodule(&basic)?;
    let adc_module = PyModule::new(m.py(), "adc")?;
    adc_module.add_class::<lib_adc::MCP3204>()?;
    m.add_submodule(&adc_module)?;
    Ok(())
}
