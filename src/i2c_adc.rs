// use polars_core::config;
use pyo3::prelude::*;
// use pyo3_polars::PyDataFrame;
// use polars_core::prelude::*;
use rppal::i2c::I2c;
use std::{time, thread};

#[pyclass]
pub struct ADS1015 {
    i2c_address: u16,
    ch_bits: [u16; 4],
}

#[pymethods]
impl ADS1015 {
    #[new]
    fn new(i2c_address: u16) -> Self {
        ADS1015 {
            i2c_address,
            ch_bits: [0x4000, 0x5000, 0x6000, 0x7000],
        }
    }

    fn read_oneshot(&self) -> PyResult<Vec::<f32>> {
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(self.i2c_address).unwrap();
        let mut data = vec![0; 2];
        let mut result = vec![0.0; 4];
        for i in 0..4 {
            let config = 0x8583 | self.ch_bits[i];
            i2c.write(&[0x01, (config >> 8) as u8, config as u8]).unwrap();
            thread::sleep(time::Duration::from_millis(10));
            i2c.read(&mut data).unwrap();
            let value = ((data[0] as u16) << 8) | data[1] as u16;
            result[i] = (value as f32) * 4.096 / 0x7FFF as f32;
        }
        Ok(result)
    }
}