// management of the rover's GPS
// hardware: https://learn.adafruit.com/adafruit-ultimate-gps-featherwing
// driver: https://crates.io/crates/nmea-parser

#[macro_use(singleton)]
extern crate cortex_m;

pub struct RoverGPS {
    gps_sats: u8,
    //add more when I get comm working
}

static mut ROVER_GPS: RoverGPS = RoverGPS {
    gps_sats: 0u8,
};

impl RoverGPS {
    pub fn get_sats(&mut self) {

    }

    pub fn init(&mut self, uart: Uart) {
        // TODO initialize the hardware
    }

    fn refresh_gps() {
        // TODO parse the nmea sentence and update the fields
    }
}

#[interrupt]
fn SOMEINTERRUPT() { // TODO which interrupt?
    unsafe { ROVER_GPS.refresh_gps(); }
}