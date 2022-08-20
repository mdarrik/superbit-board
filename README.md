# Super:bit Board Rust Lib

A Rust Lib for Yahboom's Super:bit expansion board. This lib tries to abstract away a bunch of common functionality, like moving servos and motors, and setting the Super:bit's LEDs. 

This should make coding the various learning projects more straightforward since you can use the methods on the defined struct to map to the instructions for python or the MakeCode blocks.



## Caveats

1. This lib is very much a WIP and I'm adding functionality as I need it.

2. There's some unsafe code in the neopixel crate due to the need for static lifetimes.Technically the only unsafe code is constrained to the new method, but the whole show method involves modifying the attached `&mut'static` array. It should be fine, but setting neopixels inside and outside of interrupts could cause a chance for issues. I didn't use Critical Sections since I wanted to leave the possibility for using the [softdevice bluetooth module](https://github.com/embassy-rs/nrf-softdevice) which strongly discourages disabling interrupts. If the board is behind a mutex which you can only access from critical sections, it should definitely be okay to run the neopixel commands in/out of interrupts.

3. This crate currently will not support a microbit v1 since there's no pwm peripheral on that chipset.
