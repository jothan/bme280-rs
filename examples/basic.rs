extern crate bme280;

use bme280::i2c::BME280;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
fn main() {
    use linux_embedded_hal::{Delay, I2cdev};

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_secondary(i2c_bus);
    let mut delay = Delay;
    bme280.init(&mut delay).unwrap();
    loop {
        let measurements = bme280.measure(&mut delay).unwrap();
        println!("Relative Humidity = {}%", measurements.humidity);
        println!("Temperature = {} deg C", measurements.temperature);
        println!("Pressure = {} pascals", measurements.pressure);
        thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This example is only for Linux with I2C support.");
    println!("Please run it on a compatible device with the BME280 sensor connected via I2C.");
}
