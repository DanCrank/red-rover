// management of the onboard OLED display
// hardware: https://learn.adafruit.com/adafruit-128x64-oled-featherwing/overview
// driver: https://github.com/DanCrank/sh1107
//   which is a fork of: https://github.com/aaron-hardin/sh1107
//   which is a fork of: https://github.com/jamwaffles/sh1106
// note that the aaron-hardin driver (and thus mine also) require a non-current
//   version of embedded-graphics (0.6).

// For now, the display is just a simple text panel:
// GPS: <# of GPS satellites locked>
// Rcv: <RSSI of received signal from ground station>
// more to come...

#[macro_use(singleton)]
extern crate cortex_m;

use feather_m4 as bsp;
use bsp::I2c;
use sh1107::{prelude::*, Builder};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};

pub struct RoverDisplay {
    gps_sats: u8,
    rcv_rssi: f32,
    display: GraphicsMode<_>,
}

static mut ROVER_DISPLAY: RoverDisplay = RoverDisplay {
    gps_sats: 0u8,
    rcv_rssi: 0.0f32,
    display: GraphicsMode,
};

impl Display {
    pub fn update_sats(&mut self) {
        // update sats directly from the GPS
        // TODO: mutex/critsec
        self.gps_sats = ROVER_GPS.get_sats();
    }

    pub fn update_rcv_rssi(&mut self) {
        // TODO: update rssi directly from the radio
        // TODO: mutex/critsec
    }

    pub fn init(&mut self, i2c: I2c)
    {
        self.gps_sats = 0;
        self.rcv_rssi = 0.0;
        self.display = Builder::new()
            .with_size(DisplaySize::Display64x128)
            .with_rotation(DisplayRotation::Rotate0)
            .connect_i2c(i2c)
            .into();
        self.display.init().unwrap();
        self.display.clear();
        self.display.flush().unwrap();
    }

    fn refresh_display() {
        self.display.clear();
        let display_str = format!("GPS:{}\nRCV:{}", self.gps_sats, self.rcv_rssi);
        Text::new(display_str, Point::new(0, 0))
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}

#[interrupt]
fn TIM2() {
    unsafe { ROVER_DISPLAY.refresh_display(); }
}