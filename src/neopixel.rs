use crate::helper_types::Error;
use nrf_hal_common::pwm::{CounterMode, Instance, LoadMode, Loop, Prescaler, Pwm};

// 4 neopixels * 3 color bits per neopixel * 8 bits of instruction per color + 2 bits for end
const PIXEL_BUFFER_BIT_COUNT: usize = 98;
static mut PIXEL_BUFFER: [u16; PIXEL_BUFFER_BIT_COUNT] = [0; PIXEL_BUFFER_BIT_COUNT];
// T0H and T1H signals for neopixels
const PIXEL_T0H: u16 = 32774;
const PIXEL_T1H: u16 = 32781;
#[non_exhaustive]
pub struct Neopixel<T: Instance> {
    pwm: Option<Pwm<T>>,
    pixel_buffer: Option<&'static mut [u16; PIXEL_BUFFER_BIT_COUNT]>,
    neopixel_color_bits: [u8; 12],
}

impl<T> Neopixel<T>
where
    T: Instance,
{
    pub fn new(pwm: Pwm<T>) -> Self {
        unsafe {
            Neopixel {
                pwm: Some(pwm),
                pixel_buffer: Some(&mut PIXEL_BUFFER),
                neopixel_color_bits: [0; 12],
            }
        }
    }

    pub fn show(&mut self) -> Result<(), Error<nrf_hal_common::pwm::Error>> {
        self.set_color_buffer();
        if let Some(pwm) = self.pwm.take() {
            pwm.set_counter_mode(CounterMode::Up);
            pwm.set_prescaler(Prescaler::Div1);
            pwm.set_max_duty(20);
            pwm.set_loop(Loop::Disabled);
            pwm.set_load_mode(LoadMode::Common);
            let pixel_buffer = self.pixel_buffer.take();
            let pwm_seq = pwm
                .load(pixel_buffer, None::<&u16>, true)
                .map_err(|(err, ..)| Error::Pwm(err))?;
            let (returned_pixel_buffer, _, pwm) = pwm_seq.split();
            self.pwm = Some(pwm);
            self.pixel_buffer = returned_pixel_buffer;

            Ok(())
        } else {
            Err(Error::PwmIsUnbound)
        }
    }

    fn set_color_buffer(&mut self) {
        let mut pos = 0usize;
        if let Some(pixel_buffer) = self.pixel_buffer.take() {
            for n in 0..12usize {
                let pix: u8 = self.neopixel_color_bits[n];
                let mut mask: u8 = 0x80;
                while mask > 0 {
                    pixel_buffer[pos] = if (pix & mask) == 0 {
                        PIXEL_T0H
                    } else {
                        PIXEL_T1H
                    };
                    pos += 1;
                    mask >>= 1;
                }
            }
            // signal that the instructions are done with 2 0 codes
            pixel_buffer[pos] = 0x8000;
            pos += 1;
            pixel_buffer[pos] = 0x8000;
        }
    }

    pub fn set_neopixel_color<E: core::fmt::Debug>(
        &mut self,
        light_position: NeopixelPosition,
        color: RgbColor,
    ) -> Result<(), Error<E>> {
        self.neopixel_color_bits[light_position as usize * 3] = color.g;
        self.neopixel_color_bits[light_position as usize * 3 + 1] = color.r;
        self.neopixel_color_bits[light_position as usize * 3 + 2] = color.b;
        Ok(())
    }

    pub fn set_all_neopixels<E: core::fmt::Debug>(
        &mut self,
        color: RgbColor,
    ) -> Result<(), Error<E>> {
        for light_position in [
            NeopixelPosition::Pixel0,
            NeopixelPosition::Pixel1,
            NeopixelPosition::Pixel2,
            NeopixelPosition::Pixel3,
            NeopixelPosition::Pixel4,
        ] {
            self.set_neopixel_color(light_position, color)?;
        }
        Ok(())
    }

    pub fn get_current_colors(&self) -> [RgbColor; 4] {
        let mut current_colors = [RgbColor::default(); 4];
        for (index, color) in self.neopixel_color_bits.chunks(3).enumerate() {
            if let [g, r, b] = *color {
                current_colors[index] = RgbColor { g, r, b };
            }
        }
        current_colors
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NeopixelPosition {
    Pixel0 = 0,
    Pixel1 = 1,
    Pixel2 = 2,
    Pixel3 = 3,
    Pixel4 = 4,
}
