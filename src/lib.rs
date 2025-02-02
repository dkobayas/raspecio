use pyo3::prelude::*;
use rppal::gpio::Gpio;
use std::{time, thread};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn blink(pin_id: u8, on_time: u64, off_time: u64, n_cycles: usize, unit: String) -> PyResult<()> {
    let allowed_pins = [4, 5, 6, 12, 13, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27];
    if !allowed_pins.contains(&pin_id) {
        return Err(pyo3::exceptions::PyValueError::new_err("Invalid pin number"));
    }
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(pin_id).unwrap().into_output();

    let (on_time_unit, off_time_unit) = match unit.as_str() {
        "sec" => (on_time * 1_000_000, off_time * 1_000_000),
        "msec" => (on_time * 1_000, off_time * 1_000),
        "usec" => (on_time, off_time),
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid unit")),
    };

    for _ in 0..n_cycles {
        pin.set_high();
        thread::sleep(time::Duration::from_micros(on_time_unit as u64));
        pin.set_low();
        thread::sleep(time::Duration::from_micros(off_time_unit as u64));
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn raspecio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(blink, m)?)?;
    Ok(())
}
