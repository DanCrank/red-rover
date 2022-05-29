#![no_std]
#![no_main]

mod display;

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use feather_m4 as bsp;
use bsp::hal;
use bsp::pac;
use bsp::{entry, periph_alias};
use pac::{CorePeripherals, Peripherals};
use hal::{
    clock::GenericClockController,
    timer::*,
    prelude::*,
    sercom::{i2c, uart},
    delay::Delay,
};
use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;
use ws2812_timer_delay::Ws2812;
use sh1107::{prelude::*, Builder};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};

#[entry]
fn main() -> ! {
    // set up peripherals
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);

    // Take SDA and SCL for i2c
    let (sda, scl) = (pins.sda, pins.scl);

    // clocks and timers
    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    // tc3 is used for delays
    let mut delay_timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    delay_timer.start(3.mhz());
    // tc2 is used for the interrupt driving the display refresh
    let mut interrupt_timer = TimerCounter::tc2_(&timer_clock, peripherals.TC2, &mut peripherals.MCLK);

    // set up I2C
    let sercom2_clock = &clocks.sercom2_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let i2c_sercom = periph_alias!(peripherals.i2c_sercom);
    let i2c = i2c::Config::new(&peripherals.MCLK, i2c_sercom, pads, sercom2_clock.freq())
        .baud(100.khz())
        .enable();

    // set up a second UART on SERCOM3 for talking to the GPS
    // see: https://docs.rs/atsamd-hal/0.15.1/atsamd_hal/sercom/uart/index.html
    let gps_sercom_pads = uart::Pads::<Sercom3>::default()
        .rx(pins.pa23)
        .tx(pins.pa22);
    let gps_sercom_pm = peripherals.PM;
    let gps_sercom = peripherals.SERCOM3;
    let gps_sercom_freq = 10.mhz();
    let gps_uart =
        uart::Config::new(&gps_sercom_pm,
                              gps_sercom,
                              gps_sercom_pads,
                              gps_sercom_freq)
            .enable();

    // initialize the GPS
    ROVER_GPS.init(uart);

    // initialize the display and start its interrupt
    ROVER_DISPLAY.init(i2c);
    interrupt_timer.enable_interrupt();

    // turn off the NeoPixel because it comes on full bright white and burns my retinas
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = Ws2812::new(timer, neopixel_pin);
    let black: [RGB8; 1] = [RGB8 { r: 0, g: 0, b: 0 }];
    loop {
        // for some reason it still comes on full bright GREEN after a h/w reset, so keep turning it off
        neopixel.write(black.iter().cloned()).unwrap(); // make this an iter::once()?
        delay.delay_ms(250u16);
    }
}
