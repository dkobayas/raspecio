use pyo3::prelude::*;
use rppal::gpio::Gpio;
use std::{time, thread};


#[pyfunction]
pub fn blink_single(pin_id: u8, on_time: u64, off_time: u64, n_cycles: usize, unit: String) -> PyResult<()> {
    let allowed_pins = [5, 6, 12, 13, 16, 17, 19, 20, 21, 26];
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
    pin.set_low();
    Ok(())
}

#[pyfunction]
pub fn blink_multiple(pin_ids: Vec<u8>, on_time: u64, off_time: u64, n_cycles: usize, unit: String) -> PyResult<()> {
    let allowed_pins = [5, 6, 12, 13, 16, 17, 19, 20, 21, 26];
    for pin_id in pin_ids.iter() {
        if !allowed_pins.contains(&pin_id) {
            return Err(pyo3::exceptions::PyValueError::new_err("Invalid pin number"));
        }
    }
    let gpio = Gpio::new().unwrap();
    let mut pins = vec![];
    for pin_id in pin_ids.iter() {
        pins.push(gpio.get(*pin_id).unwrap().into_output());
    }

    let (on_time_unit, off_time_unit) = match unit.as_str() {
        "sec" => (on_time * 1_000_000, off_time * 1_000_000),
        "msec" => (on_time * 1_000, off_time * 1_000),
        "usec" => (on_time, off_time),
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid unit")),
    };

    for _ in 0..n_cycles {
        for pin in pins.iter_mut() {
            pin.set_high();
        }
        thread::sleep(time::Duration::from_micros(on_time_unit as u64));
        for pin in pins.iter_mut() {
            pin.set_low();
        }
        thread::sleep(time::Duration::from_micros(off_time_unit as u64));
    }
    for pin in pins.iter_mut() {
        pin.set_low();
    }
    Ok(())
}


#[pyfunction]
pub fn pwm_single(pin_id: u8, pitch: u64, duties: Vec<u64>, unit: String) -> PyResult<()> {
    let allowed_pins = [5, 6, 12, 13, 16, 17, 19, 20, 21, 26];
    if !allowed_pins.contains(&pin_id) {
        return Err(pyo3::exceptions::PyValueError::new_err("Invalid pin number"));
    }
    if pitch < 1_000 && unit == "usec" {
        return Err(pyo3::exceptions::PyValueError::new_err("Pitch must be greater than 100 usec"));
    }
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(pin_id).unwrap().into_output();

    let pitch_unit = match unit.as_str() {
        "sec" => pitch * 1_000_000,
        "msec" => pitch * 1_000,
        "usec" => pitch,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid unit")),
    };
    for duty in duties.iter() {
        let on_time = ((*duty as f64 / 100.0) * pitch_unit as f64) as u64;
        let off_time = ((1.0 - *duty as f64 / 100.0) * pitch_unit as f64) as u64;
        pin.set_high();
        thread::sleep(time::Duration::from_micros(on_time));
        pin.set_low();
        thread::sleep(time::Duration::from_micros(off_time));
    }
    pin.set_low();
    Ok(())
}