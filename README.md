# red-rover

Rust port of my MkII rover code.

I may or may not continue this. I am a Rust newbie, and my progress on this code is
painstakingly slow. On top of which, I'm not finding pre-made crate support for
much of the hardware I'm trying to integrate.

I still want to do some embedded Rust, and this little bit here was good experience,
but I need to start with something simpler. I'll be going back to development on
the C/C++ version at https://github.com/DanCrank/rover.

============================================================================================

This project is based on the cortex-m-quickstart template (the template's README is below).

This code targets the following hardware...

Adafruit Feather M4 Express board:
https://learn.adafruit.com/adafruit-feather-m4-express-atsamd51

Adafruit 128x64 OLED display Featherwing:
https://learn.adafruit.com/adafruit-128x64-oled-featherwing

Adafruit Radio FeatherWing - RFM69HCW 900MHz
https://learn.adafruit.com/radio-featherwing

Adafruit DC Motor + Stepper FeatherWing
https://learn.adafruit.com/adafruit-stepper-dc-motor-featherwing

Adafruit 9-DoF IMU ISM330DHCX + LIS3MDL Featherwing
https://learn.adafruit.com/st-9-dof-combo

Adafruit Ultimate GPS Featherwing
https://learn.adafruit.com/adafruit-ultimate-gps-featherwing

Adafruit Adalogger Featherwing
https://learn.adafruit.com/adafruit-adalogger-featherwing

Adafruit INA219 Power Monitoring Featherwing
https://learn.adafruit.com/adafruit-ina219-current-sensor-breakout

Slamtec RoboPeak RPLIDAR
https://www.adafruit.com/product/4010
https://cdn-shop.adafruit.com/product-files/4010/4010_datasheet.pdf
http://www.slamtec.com/en/Support#rplidar-a1
http://www.robopeak.net/data/doc/rplidar/appnote/RPLDAPPN01-rplidar_appnote_arduinolib-enUS.pdf
https://github.com/robopeak/rplidar_arduino
See also: https://github.com/DanCrank/rplidar_arduino/tree/begin-returns-void

DFRobot Devastator Tank Mobile Robot Platform (Metal DC Gear Motor version)
https://www.robotshop.com/en/devastator-tank-mobile-robot-platform-metal-dc-gear-motor.html

# `cortex-m-quickstart`

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

0. Before we begin you need to identify some characteristics of the target
  device as these will be used to configure the project:

- The ARM core. e.g. Cortex-M3.

- Does the ARM core include an FPU? Cortex-M4**F** and Cortex-M7**F** cores do.

- How much Flash memory and RAM does the target device has? e.g. 256 KiB of
  Flash and 32 KiB of RAM.

- Where are Flash memory and RAM mapped in the address space? e.g. RAM is
  commonly located at address `0x2000_0000`.

You can find this information in the data sheet or the reference manual of your
device.

In this example we'll be using the STM32F3DISCOVERY. This board contains an
STM32F303VCT6 microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 256 KiB of Flash located at address 0x0800_0000.

- 40 KiB of RAM located at address 0x2000_0000. (There's another RAM region but
  for simplicity we'll ignore it).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Set a default compilation target. There are four options as mentioned at the
   bottom of `.cargo/config`. For the STM32F303VCT6, which has a Cortex-M4F
   core, we'll pick the `thumbv7em-none-eabihf` target.

``` console
$ tail -n6 .cargo/config
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

3. Enter the memory region information into the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the STM32F303VCT6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
