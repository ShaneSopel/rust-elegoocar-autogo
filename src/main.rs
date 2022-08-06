#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! 
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
 
    let mut input5  = pins.d5.into_output();
    let mut input6  = pins.d6.into_output();
    let mut motor1  = pins.d7.into_output();
    let mut motor2  = pins.d8.into_output();
    let mut motor3  = pins.d9.into_output();
    let mut motor4  = pins.d11.into_output();

    init_pins();
    loop {
        // Moving Forward 1 Second
        input5.toggle();
        input6.toggle();
        motor1.toggle();
        motor4.toggle();
        arduino_hal::delay_ms(1000);

        // Moving in Reverse 1 Second
        input5.toggle();
        input6.toggle();
        motor2.toggle();
        motor3.toggle();
        arduino_hal::delay_ms(1000);

        // Moving Left 1 Second
        input5.toggle();
        input6.toggle();
        motor2.toggle();
        motor4.toggle();
        arduino_hal::delay_ms(1000);
  
        // Moving Right 1 Second
        input5.toggle();
        input6.toggle();
        motor1.toggle();
        motor3.toggle();
        arduino_hal::delay_ms(1000)
    }
}

