#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::entry;
use panic_halt as _;
use arduino_hal::simple_pwm::SimplePwm;

#[entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Configuramos el pin D9 (PWM) para controlar el servo
    let mut pwm = SimplePwm::new(pins.d9.into_output(), dp.TC0);
    pwm.set_prescale::<arduino_hal::simple_pwm::Prescale64>();

    loop {
        // Gira el servo a 0 grados
        pwm.set_duty(0);
        arduino_hal::delay_ms(1000);

        // Gira el servo a 90 grados (ajusta el valor según sea necesario)
        pwm.set_duty(128); // Valor para girar a 90 grados
        arduino_hal::delay_ms(1000);

        // Gira el servo a 180 grados
        pwm.set_duty(255);
        arduino_hal::delay_ms(1000);
    }
}
