#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error<E> {
    Pwm(E),
    Pca9685(E),
    PwmIsUnbound,
    InvalidNeoPixelPosition,
}
