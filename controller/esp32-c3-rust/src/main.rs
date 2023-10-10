use esp_idf_hal::{
	delay::FreeRtos,
	i2c::{I2cConfig, I2cDriver},
	prelude::*,
};
use esp_idf_sys as _;
use icm42670::{accelerometer::vector::F32x3, prelude::_accelerometer_Accelerometer, Address};
//use shtcx::{Measurement, PowerMode::*};

fn main() {
	esp_idf_sys::link_patches();

	let peripherals = Peripherals::take().expect("Failed to take peripherals");

	let i2c_config = I2cConfig::new()
		.baudrate(400.kHz().into())
		.sda_enable_pullup(true)
		.scl_enable_pullup(true);

	let shared_bus = shared_bus::BusManagerSimple::new(
		I2cDriver::new(
			peripherals.i2c0,
			peripherals.pins.gpio10,
			peripherals.pins.gpio8,
			&i2c_config,
		)
		.expect("Failed to create i2c driver"),
	);


	let mut icm42670 = icm42670::Icm42670::new(shared_bus.acquire_i2c(), Address::Primary)
		.expect("Failed to instantiate icm42670");

	loop {

		let temp = icm42670
			.temperature()
			.expect("Failed to read ICM42670 temp");

		// let F32x3 {
		// 	x: ax,
		// 	y: ay,
		// 	z: az,
		// } = icm42670
		// 	.accel_norm()
		// 	.expect("Failed to read ICM42670 accel data");

		let F32x3 {
			x: gx,
			y: gy,
			z: gz,
		} = icm42670
			.gyro_norm()
			.expect("Failed to read ICM42670 gyro data");

		//println!(
		//	"ICM42670:\n\t- temp: {}\n\t- accel: {}, {}, {}\n\t- gyro: {}, {}, {}",
		//	temp, ax, ay, az, gx, gy, gz
	    //);
            println!("{}", gy);
	}
}
