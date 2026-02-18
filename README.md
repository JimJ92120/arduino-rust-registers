# arduino-asm

Example to run Assembly via Rust for Arduino Uno.

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

# references

- https://gist.github.com/mhitza/8a4608f4dfdec20d3879
- https://ww1.microchip.com/downloads/en/devicedoc/AVR-Instruction-Set-Manual-DS40002198A.pdf
- https://github.com/Rahix/avr-hal/blob/main/arduino-hal/src/port/uno.rs#L124
- https://github.com/Rahix/avr-hal/blob/main/mcu/atmega-hal/src/port.rs
