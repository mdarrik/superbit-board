use core::ops::RangeInclusive;
use embedded_hal::blocking::i2c;
use pwm_pca9685::Pca9685 as pwmPca9685;
use pwm_pca9685::{Channel, Error};

//Some logic inspired by https:://github.com/lzty634158/SuperBit

const SUPERBIT_PCA9685_ADDRESS: u8 = 0x40;
const SUPERBIT_PCA_PRESCALE: u8 = 121; // this corresponds to 50hz which is the frequency of the superbit servos

///Struct for
#[non_exhaustive]
#[derive(Debug)]
pub struct Pca9685<I2C> {
    pca_board: pwmPca9685<I2C>,
}

impl<I2C, E> Pca9685<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2C) -> Self {
        let mut pca_board = pwmPca9685::new(i2c, SUPERBIT_PCA9685_ADDRESS).unwrap();
        // set the prescale to 121 which corr
        pca_board.set_prescale(SUPERBIT_PCA_PRESCALE).unwrap();
        Pca9685 { pca_board }
    }

    /// Moves a 270 degree servo at [`ServoPosition`] the number of degrees specified by [`degrees`]. Note that [`degrees`] must be in 0..=270;
    pub fn move_270_servo(
        &mut self,
        servo_pos: ServoPosition,
        degrees: u16,
    ) -> Result<(), Error<E>> {
        let mut scaled_degrees: u32 = degrees.into();
        if scaled_degrees > 270 {
            scaled_degrees = 270
        }
        scaled_degrees = Pca9685::<I2C>::map_number_to_new_range(scaled_degrees, 0..=270, 0..=180)
            .ok_or(Error::InvalidInputData)?;

        // pulse low range is .6 us, pulse high range is 2.4us, but calculated in milliseconds
        let degrees_mapped_to_ms = scaled_degrees * 10 + 600;

        let pulse_duration = degrees_mapped_to_ms * 4096 / 20_000;
        //write to the channel corresponding to the servo with the calculated pulse_duration
        self.pca_board.set_channel_on_off(
            servo_pos.get_board_channel(),
            0,
            pulse_duration
                .try_into()
                .map_err(|_| Error::InvalidInputData)?,
        )
    }
    /// drive a motor at [`MotorPosition`] at the speed set by `speed`.
    pub fn drive_motor(
        &mut self,
        motor: MotorPosition,
        speed: u8,
        direction: MotorDirection,
    ) -> Result<(), Error<E>> {
        let mut scaled_up_speed = u16::from(speed) * 16u16;
        if scaled_up_speed >= 4096 {
            scaled_up_speed = 4095;
        }
        let (forward_motor_channel, reverse_motor_channel) = match motor {
            MotorPosition::M1 | MotorPosition::M2 => {
                let (reverse_motor_channel, forward_motor_channel) = motor.get_board_channels();
                (forward_motor_channel, reverse_motor_channel)
            }
            MotorPosition::M3 | MotorPosition::M4 => motor.get_board_channels(),
        };

        if let MotorDirection::Forward = direction {
            self.pca_board
                .set_channel_on_off(forward_motor_channel, 0, scaled_up_speed)?;
            self.pca_board
                .set_channel_on_off(reverse_motor_channel, 0, 0)?;
        } else {
            self.pca_board
                .set_channel_on_off(forward_motor_channel, 0, 0)?;
            self.pca_board
                .set_channel_on_off(reverse_motor_channel, 0, scaled_up_speed)?;
        }

        Ok(())
    }

    fn map_number_to_new_range(
        value: u32,
        original_range: RangeInclusive<u32>,
        new_range: RangeInclusive<u32>,
    ) -> Option<u32> {
        if !original_range.contains(&value) {
            return None;
        }
        let value_in_new_range = ((value - original_range.start())
            * (new_range.end() - new_range.start()))
            / (original_range.end() - original_range.start())
            + new_range.start();
        Some(value_in_new_range)
    }
}

pub enum ServoPosition {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
}
impl ServoPosition {
    /// Returns the channel for the servo of this [`ServoPosition`].
    fn get_board_channel(&self) -> Channel {
        match self {
            ServoPosition::S1 => Channel::C0,
            ServoPosition::S2 => Channel::C1,
            ServoPosition::S3 => Channel::C2,
            ServoPosition::S4 => Channel::C3,
            ServoPosition::S5 => Channel::C4,
            ServoPosition::S6 => Channel::C5,
            ServoPosition::S7 => Channel::C6,
            ServoPosition::S8 => Channel::C7,
        }
    }
}

pub enum MotorPosition {
    M1,
    M2,
    M3,
    M4,
}
impl MotorPosition {
    pub fn get_board_channels(&self) -> (Channel, Channel) {
        match self {
            MotorPosition::M1 => (Channel::C8, Channel::C9),
            MotorPosition::M2 => (Channel::C10, Channel::C11),
            MotorPosition::M3 => (Channel::C12, Channel::C13),
            MotorPosition::M4 => (Channel::C14, Channel::C15),
        }
    }
}

pub enum MotorDirection {
    Forward,
    Reverse,
}
