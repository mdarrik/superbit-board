#[non_exhaustive]
pub enum Error<E> {
    Pwm(E),
    Pca9685(E),
    PwmIsUnbound,
    InvalidNeoPixelPosition,
}
