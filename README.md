# arduino-rust-registers

Examples to use registers via Rust for Arduino Uno (`ATmega328P`).

Each example will be added to a different branch.  
`main` will contain the initial setup and environment as project's boilerplate.

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

---

# usage

Note that in most cases, `root` permission is required to access `/dev/ttyXXX`.

```sh
# run with serial output with `ravedude`
cargo run

# build .elf in target/avr-none/arduino.elf
cargo build --release

# push to board
sudo avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-none/release/arduino.elf
```

---

# targets

### rust

`target` may be different based on the `nightly` version used (e.g `avr-none`, `avr-unknown-unknown`, via `json`, etc).
The current setup uses `latest` branch (`1.95`).

See `.cargo/config.toml`.

### other boards

Below files must be updated to the correct target:

```toml
# `.cargo/config.toml`
[build]
...
rustflags = ["-C", "target-cpu=$TARGET"] # replace $TARGET

# `Raveduded.toml`
[general]
...
board = "$BOARD" # replace $BOARD

```

#### commands

```sh
# push to board
avrdude -p$BOARD -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-none/release/arduino.elf
```

---

# references

- https://gist.github.com/mhitza/8a4608f4dfdec20d3879
- https://ww1.microchip.com/downloads/en/devicedoc/AVR-Instruction-Set-Manual-DS40002198A.pdf
- https://github.com/Rahix/avr-hal/blob/main/arduino-hal/src/port/uno.rs#L124
- https://github.com/Rahix/avr-hal/blob/main/mcu/atmega-hal/src/port.rs
- https://github.com/Aronnok093/USART-Communication
- https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=275
