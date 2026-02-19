# arduino-rust-registers

Example to use registers via Rust for Arduino Uno.

**Pin 13** (built-in) and **Pin 8** are set here to turn on and off and characters are being sent via `serial`.

---

# installation

### requirements

|            |                                                     |
| ---------- | --------------------------------------------------- |
| `rust`     | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `cargo`    | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `gcc-avr`  | `14.3.0`                                            |
| `avrdude`  | `8.1`                                               |
| `ravedude` | `0.2.2`                                             |

### setup

#### nix

Build dependencies and environment are setup in `shell.nix`.

```sh
nix-shell
```

### notes

`target` may be different based on the `nightly` version used (e.g `avr-none`, `avr-unknown-unknown`, via `json`, etc).
The current setup uses `latest` branch (`1.95`).

See `.cargo/config.toml`.

### other boards

Update the `.cargo/config.toml` with the correct processor:

```toml
[build]
...
rustflags = ["-C", "target-cpu=$TARGET"] # replace $TARGET
```

Then update `Raveduded.toml` with the correct board:

```toml
[general]
...
board = "$BOARD" # replace $BOARD

```

---

# usage

```sh
# run with serial output
# (may require `root` privilege to access USB)
cargo run

# build .elf in target/avr-none/arduino.elf
cargo build --release

# push to board
# (may require `root` privilege to access USB)
avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-none/arduino.elf
```

---

# assembly alternative

An alternative may be to use `asm!()` method and translate Assembly lines to Rust.
e.g to turn `PIN 13` on / off:

```rust
#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    // PORT_B: 0x05
    // DDR_B: 0x04
    // PIN_13: 5 (in PORT_B)

    // 1 second delay with `nop`
    const DELAY: u32 = 1000000;

    unsafe {
        asm!{
            // set pin 13 input
            "sbi 0x04, 5",
            // set register `r20` to 0
            "clr r20",
        };

    }

    loop {
        unsafe {
            // on
            asm!{
                "sbi 0x05, 5"
            };

            for _ in 1..DELAY {
                asm!("nop");
            }

            // off
            asm!{
                "cbi 0x05, 5"
            };

            for _ in 1..DELAY {
                asm!("nop");
            }
        }
    }
}
```

---

# references

- https://gist.github.com/mhitza/8a4608f4dfdec20d3879
- https://ww1.microchip.com/downloads/en/devicedoc/AVR-Instruction-Set-Manual-DS40002198A.pdf
- https://github.com/Rahix/avr-hal/blob/main/arduino-hal/src/port/uno.rs#L124
- https://github.com/Rahix/avr-hal/blob/main/mcu/atmega-hal/src/port.rs
- https://github.com/Aronnok093/USART-Communication
- https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=275
