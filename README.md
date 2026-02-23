# arduino-rust-registers

Example implements a digital input and a digital output to:

- read an input from **GPIO 7** and set a state
- toggle (`high` / `low`) **GPIO 12** based on **GPIO 7** input state
- send the current state (`led on` / `led off`) and action (`button pressed`) via UART

**Note**: holding the button pressed will toggle the LED rapidly, additional logic and checks may be implemented to prevent this.

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
