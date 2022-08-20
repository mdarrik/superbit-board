#![no_std]
mod helper_types;
mod neopixel;
mod pca9685_controller;
use embedded_hal::blocking::i2c;
use neopixel::Neopixel;
use nrf_hal_common::pwm::{Instance, Pwm};
use pca9685_controller::Pca9685;

pub use helper_types::Error;
pub use neopixel::{NeopixelPosition, RgbColor};
// re-export nested error type for pwm
pub use nrf_hal_common::pwm::Error as NeopixelPwmError;
pub use pca9685_controller::{MotorDirection, MotorPosition, ServoPosition};
// re-export nested error type for pwm_pca9685
pub use pwm_pca9685::Error as Pca9685Error;

/// Struct for controlling the Superbit board.
/// Construct with new() and then can use the various methods to control servos, motors, and the neopixel leds.
#[non_exhaustive]
pub struct SuperBit<I2C, T: Instance> {
    pca_9685: Pca9685<I2C>,
    neopixel: Neopixel<T>,
}

impl<I2C, T: Instance, E> SuperBit<I2C, T>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    E: core::fmt::Debug,
    T: Instance,
{
    /// Create a new instance of the board. Takes an implementation of I2C & a PWM crate.
    pub fn new(i2c: I2C, pwm: Pwm<T>) -> Self {
        let neopixel = Neopixel::new(pwm);
        let pca_9685 = Pca9685::new(i2c);
        Self { pca_9685, neopixel }
    }

    ///Drive a motor specified by [`MotorPosition`] at the speed specified by `speed`, in either forward or reverse direction, as defined by [`MotorDirection`]
    pub fn drive_motor(
        self: &mut Self,
        motor: MotorPosition,
        speed: u8,
        direction: MotorDirection,
    ) -> Result<(), Error<Pca9685Error<E>>> {
        self.pca_9685
            .drive_motor(motor, speed, direction)
            .map_err(|e| Error::Pca9685(e))
    }
    ///Move a 270 degree servo at position defined by [`ServoPosition`].
    /// Note that degrees is constrained between 0 & 270, an will round down to 270 if > 270;
    pub fn move_270_servo(
        self: &mut Self,
        servo_pos: ServoPosition,
        degrees: u16,
    ) -> Result<(), Error<Pca9685Error<E>>> {
        self.pca_9685
            .move_270_servo(servo_pos, degrees)
            .map_err(|e| Error::Pca9685(e))
    }
    /// Prepares a single neopixel LED at position [`NeopixelPosition`] to be the RGB color defined.
    /// Note that you need to call `neopixel_show` for the color change to show.
    pub fn set_neopixel_color(
        self: &mut Self,
        neopixel_position: NeopixelPosition,
        color: RgbColor,
    ) -> Result<(), Error<E>> {
        self.neopixel.set_neopixel_color(neopixel_position, color)
    }
    /// Prepares all 4 neopixel LEDs to be set to a single RGB color.
    /// Note that you'll need to call `neopixel_show` for the change to show.
    pub fn set_all_neopixel_colors(self: &mut Self, color: RgbColor) -> Result<(), Error<E>> {
        self.neopixel.set_all_neopixels(color)
    }
    /// prepares to turn off the specified neopixel by setting the color to 0,0,0
    /// note that you'll need to call `neopixel_show` for this change to take effect.
    pub fn turn_off_neopixel(
        self: &mut Self,
        neopixel_position: NeopixelPosition,
    ) -> Result<(), Error<E>> {
        self.set_neopixel_color(neopixel_position, RgbColor { r: 0, g: 0, b: 0 })
    }

    /// Transmits the pixel data to the neopixels
    /// Since we're using a pwm peripheral and DMA access, this shouldn't require all of the processing power like bit banging does.
    pub fn neopixel_show(self: &mut Self) -> Result<(), Error<nrf_hal_common::pwm::Error>> {
        self.neopixel.show()
    }

    pub fn get_current_colors(self: &Self) -> [RgbColor; 4] {
        self.neopixel.get_current_colors()
    }
}
