#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*, Delay};
use icm42670::{accelerometer::vector::F32x3, Address, GyroRange};

const DEG_TO_RAD: f32 = 0.017453293;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut icm42670 =
        icm42670::Icm42670::new(i2c, Address::Primary).expect("Failed to instantiate icm42670");

    icm42670
        .set_gyro_range(GyroRange::Deg250)
        .expect("Failed to set gyro range");

    let mut delay = Delay::new(&clocks);

    loop {
        let F32x3 {
            x: _gx,
            y: gy,
            z: _gz,
        } = icm42670
            .gyro_norm()
            .expect("Failed to read ICM42670 gyro data");
        println!("{}", gy * DEG_TO_RAD);
        delay.delay_ms(100u32);
    }
}
