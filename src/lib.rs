use pyo3::prelude::*;
pub mod basic;
pub mod spi_adc;

/// A Python module implemented in Rust.
#[pymodule]
fn raspecio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let basic = PyModule::new(m.py(), "basic")?;
    basic.add_function(wrap_pyfunction!(basic::blink_single, &basic)?)?;
    basic.add_function(wrap_pyfunction!(basic::blink_multiple, &basic)?)?;
    basic.add_function(wrap_pyfunction!(basic::pwm_single, &basic)?)?;
    m.add_submodule(&basic)?;
    let adc_module = PyModule::new(m.py(), "adc")?;
    adc_module.add_class::<spi_adc::MCP3204>()?;
    adc_module.add_class::<spi_adc::MCP3208>()?;
    m.add_submodule(&adc_module)?;
    Ok(())
}
