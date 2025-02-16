use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars_core::prelude::*;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::{time, thread};

#[pyclass]
pub struct MCP3204 {
    spi_num: u8,
    ch_bits: [u8; 4],
}

#[pymethods]
impl MCP3204 {
    #[new]
    fn new(spi_num: u8) -> Self {
        MCP3204 {
            spi_num: spi_num,
            ch_bits: [0x06 | 0x00, 0x06 | 0x40, 0x06 | 0x80, 0x06 | 0xC0],
        }
    }

    fn read_oneshot(&self) -> PyResult<Vec<u16>> {
        let spi = match self.spi_num {
            0 => Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            1 => Spi::new(Bus::Spi1, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid SPI number")),
        };

        let mut result_vec = Vec::new();
        for channel_bit in self.ch_bits {
            let mut read_buffer = [0u8; 3];
            let write_buffer = [channel_bit, 0x00, 0x00];
            spi.transfer(&mut read_buffer, &write_buffer).unwrap();
            let msb = (read_buffer[1] & 0x0F) as u16;
            let lsb = read_buffer[2] as u16; // Convert third byte to u16
            let result = (msb << 8) | lsb;
            result_vec.push(result);

        }
        return Ok(result_vec);
    }

    fn read_continuous(&self, n_samples: u32, sleep_msec: u16) -> PyResult<PyDataFrame> {
        let spi = match self.spi_num {
            0 => Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            1 => Spi::new(Bus::Spi1, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid SPI number")),
        };

        let mut time_vec = Vec::new();
        let mut ch1_vec = Vec::new();
        let mut ch2_vec = Vec::new();
        let mut ch3_vec = Vec::new();
        let mut ch4_vec = Vec::new();


        for i_sample in 0..n_samples {
            time_vec.push((i_sample as f64) * (sleep_msec as f64) * 0.001);
            let mut ch_count = 0;            
            for ch_bit in self.ch_bits {
                let mut read_buffer = [0u8; 3];
                let write_buffer = [ch_bit, 0x00, 0x00];
                spi.transfer(&mut read_buffer, &write_buffer).unwrap();
                let msb = (read_buffer[1] & 0x0F) as u16;
                let lsb = read_buffer[2] as u16; // Convert third byte to u16
                let result = (msb << 8) | lsb;
                match ch_count {
                    0 => ch1_vec.push(result as f64),
                    1 => ch2_vec.push(result as f64),
                    2 => ch3_vec.push(result as f64),
                    3 => ch4_vec.push(result as f64),
                    _ => (),
                }
                ch_count += 1;
                thread::sleep(time::Duration::from_micros(10));
            }
            thread::sleep(time::Duration::from_millis(sleep_msec as u64 - 40));
        }
        let data = df!(
            "Time(sec)" => time_vec,
            "CH1" => ch1_vec,
            "CH2" => ch2_vec,
            "CH3" => ch3_vec,
            "CH4" => ch4_vec
        ).unwrap();
        return Ok(PyDataFrame(data));
    }
}

#[pyclass]
pub struct MCP3208 {
    spi_num: u8,
    ch_bits: [u8; 8],
}

#[pymethods]
impl MCP3208 {
    #[new]
    fn new(spi_num: u8) -> Self {
        MCP3208 { 
            spi_num: spi_num,
            ch_bits: [0x00, 0x40, 0x80, 0xC0, 0x00, 0x40, 0x80, 0xC0],
        }
    }

    fn read_oneshot(&self) -> PyResult<Vec<u16>> {
        let spi = match self.spi_num {
            0 => Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            1 => Spi::new(Bus::Spi1, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid SPI number")),
        };

        let mut result_vec = Vec::new();
        for ch_bit in self.ch_bits {
            let mut read_buffer = [0u8; 3];
            let write_buffer = [0x06, ch_bit, 0x00];
            spi.transfer(&mut read_buffer, &write_buffer).unwrap();
            let msb = (read_buffer[1] & 0x0F) as u16;
            let lsb = read_buffer[2] as u16; // Convert third byte to u16
            let result = (msb << 8) | lsb;
            result_vec.push(result);
        }
        return Ok(result_vec);
    }

    fn read_continuous(&self, n_samples: u32, sleep_msec: u16) -> PyResult<PyDataFrame> {
        let spi = match self.spi_num {
            0 => Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            1 => Spi::new(Bus::Spi1, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap(),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid SPI number")),
        };

        let mut time_vec = Vec::new();
        let mut ch1_vec = Vec::new();
        let mut ch2_vec = Vec::new();
        let mut ch3_vec = Vec::new();
        let mut ch4_vec = Vec::new();
        let mut ch5_vec = Vec::new();
        let mut ch6_vec = Vec::new();
        let mut ch7_vec = Vec::new();
        let mut ch8_vec = Vec::new();

        for i_sample in 0..n_samples {
            time_vec.push((i_sample as f64) * (sleep_msec as f64) * 0.001);
            let mut ch_count = 0;            
            for ch_bit in self.ch_bits {
                let mut read_buffer = [0u8; 3];
                let write_buffer = [ch_bit, 0x00, 0x00];
                spi.transfer(&mut read_buffer, &write_buffer).unwrap();
                let msb = (read_buffer[1] & 0x0F) as u16;
                let lsb = read_buffer[2] as u16; // Convert third byte to u16
                let result = (msb << 8) | lsb;
                match ch_count {
                    0 => ch1_vec.push(result as f64),
                    1 => ch2_vec.push(result as f64),
                    2 => ch3_vec.push(result as f64),
                    3 => ch4_vec.push(result as f64),
                    4 => ch5_vec.push(result as f64),
                    5 => ch6_vec.push(result as f64),
                    6 => ch7_vec.push(result as f64),
                    7 => ch8_vec.push(result as f64),
                    _ => (),
                }
                ch_count += 1;
                thread::sleep(time::Duration::from_micros(10));
            }
            thread::sleep(time::Duration::from_millis(sleep_msec as u64 - 40));
        }
        let data = df!(
            "Time(sec)" => time_vec,
            "CH1" => ch1_vec,
            "CH2" => ch2_vec,
            "CH3" => ch3_vec,
            "CH4" => ch4_vec,
            "CH5" => ch5_vec,
            "CH6" => ch6_vec,
            "CH7" => ch7_vec,
            "CH8" => ch8_vec
        ).unwrap();
        return Ok(PyDataFrame(data));
    }
}